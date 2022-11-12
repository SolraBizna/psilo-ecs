#[cfg(test)]
mod test;

use std::{
    alloc, alloc::Layout,
    mem::{size_of, transmute}, // transmute is unavoidable for this :C
    ops::{Deref, DerefMut, Range, RangeBounds, Bound},
};

use crate::EntityId;

// The hash function(s) we use are a simple modular multiply by a prime. This
// is injective; if A != B then H(A) != H(B). We discard low bits to create the
// bucket index, but within the bucket we retain the hash order. When we expand
// or contract the map, we move the "decimal point" to the right or left. Thus,
// when iterating, hash order is preserved.

#[cfg(feature="dynamic-hash")]
mod dynamic_hash;
#[cfg(feature="dynamic-hash")]
pub(crate) use dynamic_hash::hash;

#[cfg(not(feature="dynamic-hash"))]
mod constant_hash;
#[cfg(not(feature="dynamic-hash"))]
pub(crate) use constant_hash::hash;

#[cfg(feature="use-usize")]
type Size = usize;
#[cfg(not(feature="use-usize"))]
type Size = u32;

type EntityCount = EntityId;
type EntityHash = EntityId;

/// We try to determine how many values to directly embed based on some
/// heuristics. This sets a minimum on how many values we will embed directly.
/// Even if the values are enormous, we will always directly embed at least
/// this many values.
/// 
/// TODO: tune this, maybe 1 is actually better?
const MIN_MAX_EMBEDDED_VALUES: Size = 2;

/// The numerator of the load factor fraction we will rehash upward at.
const MAX_LOAD_FACTOR_NUMERATOR: Size = 3;
/// The denominator of the load factor fraction we will rehash upward at.
const MAX_LOAD_FACTOR_DENOMINATOR: Size = 4;

/// The numerator of the load factor fraction we will rehash downward at.
const MIN_LOAD_FACTOR_NUMERATOR: Size = 1;
/// The denominator of the load factor fraction we will rehash downward at.
const MIN_LOAD_FACTOR_DENOMINATOR: Size = 4;

#[derive(Debug)]
pub struct EcHashMap {
    dropper: Option<fn(*mut u8)>,
    cloner: fn(*mut u8, *const u8),
    bucket_layout_size: Size,
    bucket_layout_align: Size,
    max_direct_values: Size,
    direct_value_start: Size,
    value_layout_size: Size,
    _value_layout_align: Size,
    value_stride: Size,
    indirect_bucket_member_layout_size: Size,
    indirect_bucket_member_layout_align: Size,
    indirect_bucket_value_offset: Size,
    indirect_bucket_member_stride: Size,
    indirect_bucket_initial_capacity: Size,
    buckets_ptr: *mut u8,
    /// Number of hash bits we're using
    bits: u32,
    capacity: EntityCount,
    num_entries: EntityCount,
}

pub struct EcHashMapIter<T: Deref<Target=EcHashMap>> {
    map: T,
    next_bucket: EntityCount,
    next_entry: Size,
}

pub struct EcHashMapIterMut<T: Deref<Target=EcHashMap>> {
    map: T,
    next_bucket: EntityCount,
    next_entry: Size,
}

/* Direct buckets are as if:
struct DirectBucket {
    num_in_bucket: Size,
    ids_in_bucket: [MAX_NUM_IN_BUCKET; EntityId],
    things_in_bucket: [MAX_NUM_IN_BUCKET; Thing],
}
*/
/* The bytes pointed to by IndirectBucket are an array of:
struct {
    id: EntityId,
    thing: Thing,
}
*/
#[repr(C)]
struct IndirectBucket {
    num_in_bucket: Size,
    room_in_bucket: Size,
    bucket_ptr: *mut u8,
}

// shades of para-Dimethylaminobenzaldehyde
struct EnumBucketValuesIterIndirectMapClosure {
    indirect_bucket_value_offset: Size,
}
struct EnumBucketValuesIterMutIndirectMapClosure {
    indirect_bucket_value_offset: Size,
}

impl Fn<(&[u8], )> for EnumBucketValuesIterIndirectMapClosure {
    extern "rust-call" fn call(&self, args: (&[u8], )) -> (EntityId, *const u8) {
        let eid = unsafe { transmute::<*const u8, *const EntityId>(&args.0[0]).read() };
        let block = args.0[self.indirect_bucket_value_offset as usize ..].as_ptr();
        (eid, block)
    }
}

impl FnMut<(&[u8], )> for EnumBucketValuesIterIndirectMapClosure {
    extern "rust-call" fn call_mut(&mut self, args: (&[u8], )) -> (EntityId, *const u8) {
        self.call(args)
    }
}

impl FnOnce<(&[u8], )> for EnumBucketValuesIterIndirectMapClosure {
    type Output = (EntityId, *const u8);
    extern "rust-call" fn call_once(self, args: (&[u8], )) -> (EntityId, *const u8) {
        self.call(args)
    }
}

impl Fn<(&mut [u8], )> for EnumBucketValuesIterMutIndirectMapClosure {
    extern "rust-call" fn call(&self, args: (&mut [u8], )) -> (EntityId, *mut u8) {
        let eid = unsafe { transmute::<*const u8, *const EntityId>(&args.0[0]).read() };
        let block = args.0[self.indirect_bucket_value_offset as usize ..].as_mut_ptr();
        (eid, block)
    }
}

impl FnMut<(&mut [u8], )> for EnumBucketValuesIterMutIndirectMapClosure {
    extern "rust-call" fn call_mut(&mut self, args: (&mut [u8], )) -> (EntityId, *mut u8) {
        self.call(args)
    }
}

impl FnOnce<(&mut [u8], )> for EnumBucketValuesIterMutIndirectMapClosure {
    type Output = (EntityId, *mut u8);
    extern "rust-call" fn call_once(self, args: (&mut [u8], )) -> (EntityId, *mut u8) {
        self.call(args)
    }
}

enum BucketValuesIter<'a> {
    Direct(std::iter::Zip<std::slice::Iter<'a, EntityId>, std::slice::ChunksExact<'a, u8>>),
    DirectZST(std::slice::Iter<'a, EntityId>, *const u8),
    Indirect(std::iter::Map<std::slice::ChunksExact<'a, u8>, EnumBucketValuesIterIndirectMapClosure>),
}

enum BucketValuesIterMut<'a> {
    Direct(std::iter::Zip<std::slice::Iter<'a, EntityId>, std::slice::ChunksExactMut<'a, u8>>),
    DirectZST(std::slice::Iter<'a, EntityId>, *mut u8),
    Indirect(std::iter::Map<std::slice::ChunksExactMut<'a, u8>, EnumBucketValuesIterMutIndirectMapClosure>),
}

impl Iterator for BucketValuesIter<'_> {
    type Item = (EntityId, *const u8);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BucketValuesIter::Direct(x) => x.next().map(|(a,b)| (*a, b.as_ptr())),
            BucketValuesIter::DirectZST(x, ptr) => x.next().map(|a| (*a, *ptr)),
            BucketValuesIter::Indirect(x) => x.next(),
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            BucketValuesIter::Direct(x) => x.nth(n).map(|(a,b)| (*a, b.as_ptr())),
            BucketValuesIter::DirectZST(x, ptr) => x.nth(n).map(|a| (*a, *ptr)),
            BucketValuesIter::Indirect(x) => x.nth(n),
        }
    }
}

impl Iterator for BucketValuesIterMut<'_> {
    type Item = (EntityId, *mut u8);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BucketValuesIterMut::Direct(x) => x.next().map(|(a,b)| (*a, b.as_mut_ptr())),
            BucketValuesIterMut::DirectZST(x, ptr) => x.next().map(|a| (*a, *ptr)),
            BucketValuesIterMut::Indirect(x) => x.next(),
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            BucketValuesIterMut::Direct(x) => x.nth(n).map(|(a,b)| (*a, b.as_mut_ptr())),
            BucketValuesIterMut::DirectZST(x, ptr) => x.nth(n).map(|a| (*a, *ptr)),
            BucketValuesIterMut::Indirect(x) => x.nth(n),
        }
    }
}

fn bound_range<R: RangeBounds<Size>>(range: R, max: Size) -> Range<Size> {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x+1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => max,
        Bound::Included(x) => x+1,
        Bound::Excluded(x) => *x,
    };
    debug_assert!(end >= start);
    debug_assert!(start <= max);
    debug_assert!(end <= max);
    Range { start, end }
}

impl IndirectBucket {
    fn new(ehm: &EcHashMap, target_size: u32) -> IndirectBucket {
        let mut size = ehm.indirect_bucket_initial_capacity;
        while size < target_size { size = size.checked_mul(2).unwrap() }
        IndirectBucket {
            bucket_ptr: unsafe { alloc::alloc(ehm.indirect_bucket_member_layout_repeat(size)) },
            room_in_bucket: size,
            num_in_bucket: 0,
        }
    }
    unsafe fn pair_ptr(&self, ehm: &EcHashMap, index: Size) -> *const u8 {
        unsafe {
            self.bucket_ptr.add((ehm.indirect_bucket_member_stride * index) as usize)
        }
    }
    unsafe fn pair_mut_ptr(&mut self, ehm: &EcHashMap, index: Size) -> *mut u8 {
        unsafe {
            self.bucket_ptr.add((ehm.indirect_bucket_member_stride * index) as usize)
        }
    }
    unsafe fn pair_slice<R: RangeBounds<Size>>(&self, ehm: &EcHashMap, range: R) -> &[u8] {
        let range = bound_range(range, self.num_in_bucket);
        unsafe {
            std::slice::from_raw_parts(self.pair_ptr(ehm, range.start), ((range.end - range.start) * ehm.indirect_bucket_member_stride) as usize)
        }
    }
    unsafe fn pair_mut_slice<R: RangeBounds<Size>>(&mut self, ehm: &EcHashMap, range: R) -> &mut [u8] {
        let range = bound_range(range, self.num_in_bucket);
        unsafe {
            std::slice::from_raw_parts_mut(self.pair_mut_ptr(ehm, range.start), ((range.end - range.start) * ehm.indirect_bucket_member_stride) as usize)
        }
    }
    unsafe fn insert_at(&mut self, ehm: &EcHashMap, new_id: EntityId, target_index: Size) -> *mut u8 {
        unsafe {
            debug_assert!(self.num_in_bucket <= self.room_in_bucket);
            if self.num_in_bucket == self.room_in_bucket {
                let new_room = self.room_in_bucket.checked_mul(2).unwrap();
                self.bucket_ptr = alloc::realloc(self.bucket_ptr, ehm.indirect_bucket_member_layout_repeat(self.room_in_bucket), ehm.indirect_bucket_member_layout_repeat(new_room).size());
                self.room_in_bucket = new_room;
            }
            self.num_in_bucket += 1;
            let pair_slice = self.pair_mut_slice(ehm, ..);
            pair_slice.copy_within(
                (target_index * ehm.indirect_bucket_member_stride) as usize
                .. pair_slice.len() - ehm.indirect_bucket_member_stride as usize,
                (ehm.indirect_bucket_member_stride * (target_index + 1)) as usize);
            if ehm.value_stride > 0 {
                *transmute::<&mut u8, &mut EntityId>(&mut pair_slice[(ehm.indirect_bucket_member_stride * target_index) as usize])
                    = new_id;
                &mut pair_slice[(ehm.indirect_bucket_member_stride * target_index + ehm.indirect_bucket_value_offset) as usize]
            }
            else {
                pair_slice.as_mut_ptr().add(ehm.indirect_bucket_value_offset as usize)
            }
        }
    }
    unsafe fn remove_at(&mut self, ehm: &EcHashMap, idx: u32) {
        unsafe {
            let pair_slice = self.pair_mut_slice(ehm, ..);
            pair_slice.copy_within(
                ((idx + 1) * ehm.indirect_bucket_member_stride) as usize
                .. pair_slice.len() as usize,
                (idx * ehm.indirect_bucket_member_stride) as usize);
        }
        self.num_in_bucket -= 1;
    }
    unsafe fn dealloc(&mut self, ehm: &EcHashMap) {
        alloc::dealloc(self.bucket_ptr, ehm.indirect_bucket_member_layout_repeat(self.room_in_bucket));
    }
}

#[derive(Copy,Clone)]
struct BucketPointer(*mut u8);
impl BucketPointer {
    unsafe fn from(ptr: *mut u8) -> BucketPointer { BucketPointer(ptr) }
    unsafe fn direct_eid_ptr(&self, _ehm: &EcHashMap, index: Size) -> *const EntityId {
        unsafe {
            transmute::<*const u8, *const EntityId>(self.0.add(Layout::new::<Size>().extend(Layout::new::<EntityId>().repeat(index as usize).unwrap().0).unwrap().0.extend(Layout::new::<EntityId>()).unwrap().1))
        }
    }
    unsafe fn direct_eid_mut_ptr(&self, _ehm: &EcHashMap, index: Size) -> *mut EntityId {
        unsafe {
            transmute::<*mut u8, *mut EntityId>(self.0.add(Layout::new::<Size>().extend(Layout::new::<EntityId>().repeat(index as usize).unwrap().0).unwrap().0.extend(Layout::new::<EntityId>()).unwrap().1))
        }
    }
    unsafe fn direct_eid_slice<R: RangeBounds<Size>>(&self, ehm: &EcHashMap, range: R) -> &[EntityId] {
        let range = bound_range(range, self.len());
        unsafe {
            std::slice::from_raw_parts(self.direct_eid_ptr(ehm, range.start), (range.end - range.start) as usize)
        }
    }
    unsafe fn direct_eid_mut_slice<R: RangeBounds<Size>>(&self, ehm: &EcHashMap, range: R) -> &mut [EntityId] {
        let range = bound_range(range, self.len());
        unsafe {
            std::slice::from_raw_parts_mut(self.direct_eid_mut_ptr(ehm, range.start), (range.end - range.start) as usize)
        }
    }
    unsafe fn direct_value_ptr(&self, ehm: &EcHashMap, index: Size) -> *const u8 {
        unsafe {
            self.0.add((ehm.direct_value_start + index * ehm.value_stride) as usize)
        }
    }
    unsafe fn direct_value_mut_ptr(&self, ehm: &EcHashMap, index: Size) -> *mut u8 {
        unsafe {
            self.0.add((ehm.direct_value_start + index * ehm.value_stride) as usize)
        }
    }
    unsafe fn direct_value_slice<R: RangeBounds<Size>>(&self, ehm: &EcHashMap, range: R) -> &[u8] {
        let range = bound_range(range, self.len());
        unsafe {
            std::slice::from_raw_parts(self.direct_value_ptr(ehm, range.start), ((range.end - range.start) * ehm.value_stride) as usize)
        }
    }
    unsafe fn direct_value_mut_slice<R: RangeBounds<Size>>(&self, ehm: &EcHashMap, range: R) -> &mut [u8] {
        let range = bound_range(range, self.len());
        unsafe {
            std::slice::from_raw_parts_mut(self.direct_value_mut_ptr(ehm, range.start), ((range.end - range.start) * ehm.value_stride) as usize)
        }
    }
    unsafe fn as_indirect(&self, _ehm: &EcHashMap) -> &IndirectBucket {
        unsafe { transmute::<*const u8, &IndirectBucket>(self.0) }
    }
    unsafe fn as_indirect_mut(&self, _ehm: &EcHashMap) -> &mut IndirectBucket {
        // we might be transforming into an indirect bucket, in which case it's
        // not an error if the length doesn't yet match up
        unsafe { transmute::<*mut u8, &mut IndirectBucket>(self.0) }
    }
    fn values(&self, ehm: &EcHashMap) -> BucketValuesIter {
        unsafe {
            let num_values = self.len();
            if num_values <= ehm.max_direct_values {
                if ehm.value_stride == 0 {
                    let ids_slice = self.direct_eid_slice(ehm, ..);
                    BucketValuesIter::DirectZST(ids_slice.iter(), self.direct_value_ptr(ehm, 0))
                }
                else {
                    // TODO: optimize this not to use zip
                    let ids_slice = self.direct_eid_slice(ehm, ..);
                    let things_slice = self.direct_value_slice(ehm, ..);
                    BucketValuesIter::Direct(ids_slice.iter().zip(things_slice.chunks_exact(ehm.value_stride as usize)))
                }
            }
            else {
                let indirect_bucket = self.as_indirect(ehm);
                BucketValuesIter::Indirect(indirect_bucket.pair_slice(ehm, ..).chunks_exact(ehm.indirect_bucket_member_stride as usize).map(EnumBucketValuesIterIndirectMapClosure {
                    indirect_bucket_value_offset: ehm.indirect_bucket_value_offset,
                }))
            }
        }
    }
    fn values_mut(&mut self, ehm: &EcHashMap) -> BucketValuesIterMut {
        unsafe {
            let num_values = self.len();
            if num_values <= ehm.max_direct_values {
                if ehm.value_stride == 0 {
                    let ids_slice = self.direct_eid_slice(ehm, ..);
                    BucketValuesIterMut::DirectZST(ids_slice.iter(), self.direct_value_mut_ptr(ehm, 0))
                }
                else {
                    // TODO: optimize this not to use zip
                    let ids_slice = self.direct_eid_slice(ehm, ..);
                    let things_slice = self.direct_value_mut_slice(ehm, ..);
                    BucketValuesIterMut::Direct(ids_slice.iter().zip(things_slice.chunks_exact_mut(ehm.value_stride as usize)))
                }
            }
            else {
                let indirect_bucket = self.as_indirect_mut(ehm);
                BucketValuesIterMut::Indirect(indirect_bucket.pair_mut_slice(ehm, ..).chunks_exact_mut(ehm.indirect_bucket_member_stride as usize).map(EnumBucketValuesIterMutIndirectMapClosure {
                    indirect_bucket_value_offset: ehm.indirect_bucket_value_offset,
                }))
            }
        }
    }
    /// If this EntityID is found in the bucket, return Err(the pointer to the
    /// existing thing)---as in, insertion failed because the target already
    /// existed. If it's not found, return Ok(pointer to newly-initialized
    /// thing)---as in, insertion succeeded.
    fn get_or_insert_with<F: FnOnce(*mut u8)>(&mut self, ehm: &EcHashMap, new_id: EntityId, new_hash: EntityHash, initializer: F) -> Result<*mut u8, *mut u8> {
        let mut target_index = None;
        for (n, (id, thing)) in self.values_mut(ehm).enumerate() {
            if id == new_id { return Err(thing) }
            else if hash(id) > new_hash {
                target_index = Some(n as Size);
                break
            }
        }
        let target_index = target_index.unwrap_or(self.len());
        let ret = if self.is_indirect(ehm) {
            unsafe {
                self.as_indirect_mut(ehm).insert_at(ehm, new_id, target_index)
            }
        }
        else if self.will_become_indirect_on_insert(ehm) {
            unsafe {
                let mut new_me = self.become_indirect(ehm);
                let result = new_me.insert_at(ehm, new_id, target_index);
                transmute::<*mut u8, *mut IndirectBucket>(self.0).write(new_me);
                result
            }
        }
        else {
            unsafe {
                self.direct_insert_at(ehm, new_id, target_index)
            }
        };
        initializer(ret);
        Ok(ret)
    }
    /// Move every value we currently have into a new `IndirectBucket`.
    /// Whoever calls us is responsible for overwriting ourselves with this
    /// bucket when they're done doing whatever they're going to do!
    unsafe fn become_indirect(&mut self, ehm: &EcHashMap) -> IndirectBucket {
        unsafe {
            let mut ret = IndirectBucket::new(ehm, ehm.indirect_bucket_initial_capacity);
            ret.num_in_bucket = self.len();
            for ((old_eid, old_value), new_pair) in self.values(ehm).zip(ret.pair_mut_slice(ehm, ..).chunks_exact_mut(ehm.indirect_bucket_member_stride as usize)) {
                *transmute::<&mut u8, &mut EntityId>(&mut new_pair[0]) = old_eid;
                new_pair[ehm.indirect_bucket_value_offset as usize
                    .. (ehm.indirect_bucket_value_offset + ehm.value_layout_size) as usize]
                    .copy_from_slice(std::slice::from_raw_parts(old_value, ehm.value_layout_size as usize));
            }
            ret
        }
    }
    unsafe fn direct_insert_at(&mut self, ehm: &EcHashMap, new_id: EntityId, target_index: Size) -> *mut u8 {
        unsafe {
            self.set_len(self.len()+1);
            let ids_slice = self.direct_eid_mut_slice(ehm, ..);
            let things_slice = self.direct_value_mut_slice(ehm, ..);
            ids_slice.copy_within(
                target_index as usize
                .. ids_slice.len()-1,
                (target_index+1) as usize);
            ids_slice[target_index as usize] = new_id;
            if ehm.value_stride > 0 {
                things_slice.copy_within(
                    (target_index * ehm.value_stride) as usize
                    .. things_slice.len() - ehm.value_stride as usize,
                    (ehm.value_stride * (target_index + 1)) as usize);
                &mut things_slice[(ehm.value_stride * target_index) as usize]
            }
            else {
                things_slice.as_mut_ptr()
            }
        }
    }
    /// Iff this EntityID is found in the bucket, call the dropper and remove
    /// it.
    fn remove(&mut self, ehm: &EcHashMap, key: EntityId) -> bool {
        if let Some((idx, thing)) = self.values_mut(ehm).enumerate().filter_map(|(idx, (id, thing))| if id == key { Some((idx, thing)) } else { None}).next() {
            if let Some(dropper) = ehm.dropper { (dropper)(thing); }
            if self.is_direct(ehm) {
                unsafe {
                    let ids_slice = self.direct_eid_mut_slice(ehm, ..);
                    let things_slice = self.direct_value_mut_slice(ehm, ..);
                    ids_slice.copy_within(
                        (idx + 1) as usize
                        .. ids_slice.len(),
                        idx as usize);
                    things_slice.copy_within(
                        (idx + 1) * ehm.value_stride as usize
                        .. things_slice.len(),
                        idx * ehm.value_stride as usize);
                    self.set_len(self.len()-1);
                }
            }
            else if self.will_become_direct_on_remove(ehm) {
                unsafe {
                    let mut indirect = IndirectBucket { ..*self.as_indirect(ehm) };
                    self.set_len(self.len()-1);
                    let mut pairs = indirect.pair_slice(ehm, ..).chunks_exact(ehm.indirect_bucket_member_stride as usize);
                    let ids_slice = self.direct_eid_mut_slice(ehm, ..);
                    let things_slice = self.direct_value_mut_slice(ehm, ..);
                    for n in 0 .. self.len() {
                        // skip the one we're deleting
                        if n as usize == idx { pairs.next(); }
                        let pair = pairs.next().unwrap();
                        let eid = *transmute::<&u8, &EntityId>(&pair[0]);
                        ids_slice[n as usize] = eid;
                        things_slice[(n * ehm.value_stride) as usize
                            .. (n * ehm.value_stride + ehm.value_stride) as usize].copy_from_slice(&pair[ehm.indirect_bucket_value_offset as usize ..]);
                    }
                    indirect.dealloc(ehm);
                }
            }
            else {
                debug_assert!(self.is_indirect(ehm));
                unsafe { self.as_indirect_mut(ehm).remove_at(ehm, idx as u32) };
            }
            true
        } else { false }
    }
    fn len(&self) -> Size {
        unsafe { *transmute::<*const u8, &Size>(self.0) }
    }
    fn is_direct(&self, ehm: &EcHashMap) -> bool {
        self.len() <= ehm.max_direct_values
    }
    fn is_indirect(&self, ehm: &EcHashMap) -> bool {
        self.len() > ehm.max_direct_values
    }
    fn will_become_indirect_on_insert(&self, ehm: &EcHashMap) -> bool {
        self.len() == ehm.max_direct_values
    }
    fn will_become_direct_on_remove(&self, ehm: &EcHashMap) -> bool {
        self.len() == ehm.max_direct_values+1
    }
    unsafe fn set_len(&mut self, new_len: Size) {
        unsafe { *transmute::<*mut u8, &mut Size>(self.0) = new_len };
    }
    /// Makes this bucket the same size as another bucket, and clone every
    /// element from that bucket into this one.
    ///
    /// Assumes this bucket is uninitialized.
    unsafe fn clone_from(&mut self, ehm: &EcHashMap, other_bucket: BucketPointer) {
        unsafe {
            if other_bucket.is_direct(ehm) {
                self.set_len(0);
                for (n, (eid, value)) in other_bucket.values(ehm).enumerate() {
                    let ptr = self.direct_insert_at(ehm, eid, n as u32);
                    (ehm.cloner)(ptr, value);
                }
            }
            else {
                let indirect = self.as_indirect_mut(ehm);
                *indirect = IndirectBucket::new(ehm, other_bucket.len());
                for (n, (eid, value)) in other_bucket.values(ehm).enumerate() {
                    let ptr = indirect.insert_at(ehm, eid, n as u32);
                    (ehm.cloner)(ptr, value);
                }
            }
        }
    }
    /// Initialize this uninitialized bucket with the given number of values
    /// from the given iterator.
    unsafe fn initialize_from_moved_values<I: Iterator<Item=(EntityId, *const u8)>>(&mut self, ehm: &EcHashMap, old_values: &mut I, num_in_low: u32) {
        if num_in_low <= ehm.max_direct_values {
            unsafe {
                self.set_len(0);
                for n in 0 .. num_in_low {
                    let (eid, old_ptr) = old_values.next().unwrap();
                    let new_ptr = self.direct_insert_at(ehm, eid, n);
                    std::slice::from_raw_parts_mut(new_ptr, ehm.value_layout_size as usize).copy_from_slice(std::slice::from_raw_parts(old_ptr, ehm.value_layout_size as usize));
                }
            }
        }
        else {
            unsafe {
                let indirect = self.as_indirect_mut(ehm);
                *indirect = IndirectBucket::new(ehm, num_in_low);
                for n in 0 .. num_in_low {
                    let (eid, old_ptr) = old_values.next().unwrap();
                    let new_ptr = indirect.insert_at(ehm, eid, n);
                    std::slice::from_raw_parts_mut(new_ptr, ehm.value_layout_size as usize).copy_from_slice(std::slice::from_raw_parts(old_ptr, ehm.value_layout_size as usize));
                }
            }
        }
    }
    /// Deallocates this bucket's indirect block, if it has one. DOES NOT DROP.
    fn dealloc(&mut self, ehm: &EcHashMap) {
        unsafe {
            if self.is_indirect(ehm) { self.as_indirect_mut(ehm).dealloc(ehm); }
        }
    }
}

impl EcHashMap {
    /// Create a new `EcHashMap`, but only allocate enough room for a single
    /// entity. The map will automatically grow as needed.
    pub fn new(value_layout: Layout, dropper: Option<fn(*mut u8)>, cloner: fn(*mut u8, *const u8)) -> EcHashMap {
        EcHashMap::with_capacity(value_layout, dropper, cloner, 0)
    }
    /// Create a new `EcHashMap` with the given initial number of buckets. The
    /// map will automatically grow as needed.
    /// 
    /// At least one bucket will *always* be allocated.
    pub fn with_capacity(value_layout: Layout, dropper: Option<fn(*mut u8)>, cloner: fn(*mut u8, *const u8), capacity: usize) -> EcHashMap {
        let base_layout = Layout::new::<usize>();
        let eid_layout = Layout::new::<EntityId>();
        let value_stride = value_layout.repeat(1).unwrap().1;
        assert!(value_stride >= value_layout.size());
        let mut max_direct_values: usize = MIN_MAX_EMBEDDED_VALUES.try_into().unwrap();
        // Support a minimum of two embedded values
        let (mut bucket_layout, mut direct_value_start)
        = base_layout
        .extend(eid_layout.repeat(max_direct_values).unwrap().0).unwrap().0
        .extend(value_layout.repeat(max_direct_values).unwrap().0).unwrap();
        // Increase this number as long as the increased size will fit within
        // the same power-of-two-aligned size as the current size
        loop {
            let (moar_bucket_layout, moar_value_start)
            = base_layout
            .extend(eid_layout.repeat(max_direct_values+1).unwrap().0).unwrap().0
            .extend(value_layout.repeat(max_direct_values+1).unwrap().0).unwrap();
            if bucket_layout.size() < bucket_layout.align()
            || bucket_layout.size() < size_of::<IndirectBucket>()
            || bucket_layout.size().next_power_of_two() == moar_bucket_layout.size().next_power_of_two() {
                max_direct_values += 1;
                (bucket_layout, direct_value_start) = (moar_bucket_layout, moar_value_start);
            }
            else {
                break
            }
        }
        // This part is important! The size and the stride must be the same!
        let bucket_layout = Layout::from_size_align(bucket_layout.size().next_power_of_two(), bucket_layout.align()).unwrap();
        debug_assert!(bucket_layout.size() >= bucket_layout.align());
        debug_assert_eq!(bucket_layout.size() % bucket_layout.align(), 0);
        debug_assert_eq!(bucket_layout.size(), bucket_layout.repeat(1).unwrap().1);
        let (indirect_bucket_member_layout, indirect_bucket_value_offset) = Layout::new::<usize>().extend(value_layout).unwrap();
        let (_, indirect_bucket_member_stride) = indirect_bucket_member_layout.repeat(1).unwrap();
        debug_assert!(indirect_bucket_member_stride >= indirect_bucket_member_layout.size());
        // indirect buckets will initially be allocated to fill 256 bytes OR
        // hold twice as many entries as a direct bucket, whichever is larger
        let indirect_bucket_initial_capacity
            = (256 / indirect_bucket_member_stride).max(max_direct_values*2);
        let capacity = capacity.max(1).next_power_of_two();
        let (buckets_ptr, bits) = unsafe {
            let full_layout = bucket_layout.repeat(capacity).unwrap().0;
            (alloc::alloc_zeroed(full_layout), capacity.trailing_zeros())
        };
        debug_assert_eq!((1 << bits), capacity);
        EcHashMap {
            dropper,
            cloner,
            bucket_layout_size: bucket_layout.size().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            bucket_layout_align: bucket_layout.align().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            max_direct_values: max_direct_values.try_into().expect("More than four billion values embedded?!"),
            direct_value_start: direct_value_start.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            value_layout_size: value_layout.size().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            _value_layout_align: value_layout.align().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            value_stride: value_stride.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            indirect_bucket_member_layout_size: indirect_bucket_member_layout.size().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            indirect_bucket_member_layout_align: indirect_bucket_member_layout.align().try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            indirect_bucket_value_offset: indirect_bucket_value_offset.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            indirect_bucket_member_stride: indirect_bucket_member_stride.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            indirect_bucket_initial_capacity: indirect_bucket_initial_capacity.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            buckets_ptr,
            bits,
            capacity: capacity.try_into().expect("Component unreasonably huge, you need the use_usize feature"),
            num_entries: 0,
        }
    }
    fn truncate_hash_for_bits(&self, hash: EntityHash, bits: u32) -> EntityHash {
        debug_assert_ne!(self.capacity, 0);
        // single-entry `EcHashMap` will be used for singletons
        if bits == 0 { 0 }
        else {
            hash >> (if cfg!(feature="u64_entity_ids") { 64 } else { 32 } - bits)
        }
    }
    fn truncate_hash(&self, hash: EntityHash) -> EntityHash {
        self.truncate_hash_for_bits(hash, self.bits)
    }
    /// Get the bucket with the given index.
    /// 
    /// This is unsafe because, when debug assertions are disabled, this does
    /// not perform a bounds check. It is safe so long as you only use an
    /// appropriately bit-shifted truncated hash as an index.
    unsafe fn get_bucket(&self, trunc_hash: EntityHash) -> BucketPointer {
        debug_assert!((trunc_hash as usize) < (self.capacity as usize));
        unsafe { BucketPointer::from(self.buckets_ptr.add(self.bucket_layout_size as usize * trunc_hash as usize)) }
    }
    /// Decrease the number of buckets, allocate the new ones, and merge the
    /// existing ones in.
    fn contract_in_place(&mut self) {
        debug_assert_ne!(self.capacity, 0);
        if self.capacity == 1 { return }
        let new_capacity = self.capacity.checked_div(2).unwrap().max(1);
        let new_bits = self.bits - 1;
        debug_assert_eq!(1 << new_bits, new_capacity);
        let new_layout = self.bucket_layout_repeat(new_capacity);
        let new_buckets = unsafe { alloc::alloc(new_layout) };
        let mut old_bucket_ptr = self.buckets_ptr;
        let mut new_bucket_ptr = new_buckets;
        for _ in 0 .. new_capacity {
            unsafe {
                let mut zero_bucket = BucketPointer::from(old_bucket_ptr);
                old_bucket_ptr = old_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut one_bucket = BucketPointer::from(old_bucket_ptr);
                old_bucket_ptr = old_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut new_bucket = BucketPointer::from(new_bucket_ptr);
                new_bucket_ptr = new_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut old_values = zero_bucket.values(self).chain(one_bucket.values(self));
                new_bucket.initialize_from_moved_values(self, &mut old_values, zero_bucket.len() + one_bucket.len());
                debug_assert!(old_values.next().is_none());
                zero_bucket.dealloc(self);
                one_bucket.dealloc(self);
            }
        }
        unsafe {
            alloc::dealloc(self.buckets_ptr, self.bucket_layout_repeat(self.capacity));
        }
        self.capacity = new_capacity;
        self.bits = new_bits;
        self.buckets_ptr = new_buckets;
    }
    /// Increase the number of buckets, allocate the new ones, and split the
    /// existing ones in half.
    fn expand_in_place(&mut self) {
        debug_assert_ne!(self.capacity, 0);
        let new_capacity = self.capacity.checked_mul(2).unwrap().max(1);
        let new_bits = self.bits + 1;
        debug_assert_eq!(1 << new_bits, new_capacity);
        let new_layout = self.bucket_layout_repeat(new_capacity);
        let new_buckets = unsafe { alloc::alloc(new_layout) };
        let mut old_bucket_ptr = self.buckets_ptr;
        let mut new_bucket_ptr = new_buckets;
        for _ in 0 .. self.capacity {
            unsafe {
                let mut old_bucket = BucketPointer::from(old_bucket_ptr);
                old_bucket_ptr = old_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut zero_bucket = BucketPointer::from(new_bucket_ptr);
                new_bucket_ptr = new_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut one_bucket = BucketPointer::from(new_bucket_ptr);
                new_bucket_ptr = new_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut num_in_low = 0;
                // we assume that they're in hash order, this will put
                // everything into the right buckets and preserve hash order
                // iff so
                for (eid, _) in old_bucket.values(self) {
                    let full_hash = hash(eid);
                    let trunc_hash = self.truncate_hash_for_bits(full_hash, new_bits);
                    if trunc_hash & 1 == 1 { break }
                    else { num_in_low += 1; }
                }
                let mut old_values = old_bucket.values(self);
                zero_bucket.initialize_from_moved_values(self, &mut old_values, num_in_low);
                one_bucket.initialize_from_moved_values(self, &mut old_values, old_bucket.len() - num_in_low);
                debug_assert!(old_values.next().is_none());
                old_bucket.dealloc(self);
            }
        }
        unsafe {
            alloc::dealloc(self.buckets_ptr, self.bucket_layout_repeat(self.capacity));
        }
        self.capacity = new_capacity;
        self.bits = new_bits;
        self.buckets_ptr = new_buckets;
    }
    /// If below the minimum load factor, shrink our memory allocation.
    /// Otherwise, do nothing. If you regularly `clone` this `EcHashMap` and
    /// work on that clone (double buffer style), contraction happens
    /// seamlessly and efficiently behind the scenes, and you don't have to
    /// call this to make it happen.
    pub fn contract(&mut self) {
        debug_assert_ne!(self.capacity, 0);
        if self.should_contract() {
            self.contract_in_place();
        }
    }
    /// Returns true if this `EcHashMap` is below the minimum load factor, and
    /// will contract the next time it's given an opportunity.
    pub fn should_contract(&self) -> bool {
        if self.capacity == 1 { false }
        else { self.num_entries < (self.capacity * MIN_LOAD_FACTOR_NUMERATOR / MIN_LOAD_FACTOR_DENOMINATOR).max(1) }
    }
    /// Returns a clone of this `EcHashMap` that always has the same number of
    /// buckets. You probably want to use `clone()` instead, which will
    /// automatically contract the clone when it makes sense to do so.
    pub fn exact_clone(&self) -> EcHashMap {
        unsafe {
            let new_layout = self.bucket_layout_repeat(self.capacity);
            let new_buckets = alloc::alloc(new_layout);
            let ret = EcHashMap {
                buckets_ptr: new_buckets,
                ..*self
            };
            let mut src_bucket_ptr = self.buckets_ptr;
            let mut dst_bucket_ptr = ret.buckets_ptr;
            for _ in 0 .. self.capacity {
                let src_bucket = BucketPointer::from(src_bucket_ptr);
                src_bucket_ptr = src_bucket_ptr.add(self.bucket_layout_size as usize);
                let mut dst_bucket = BucketPointer::from(dst_bucket_ptr);
                dst_bucket_ptr = dst_bucket_ptr.add(self.bucket_layout_size as usize);
                dst_bucket.clone_from(self, src_bucket);
            }
            ret
        }
    }
    /// If the given entity exists in this map, returns a **pointer** to it.
    pub fn get(&self, key: EntityId) -> Option<*const u8> {
        if self.capacity == 0 { return None }
        let full_hash = hash(key);
        let trunc_hash = self.truncate_hash(full_hash);
        let bucket = unsafe { self.get_bucket(trunc_hash) };
        for (id, thing) in bucket.values(self) {
            if id == key { return Some(thing) }
        }
        None
    }
    /// If the given entity exists in this map, returns a mutable **pointer**
    /// to it.
    pub fn get_mut(&mut self, key: EntityId) -> Option<*mut u8> {
        if self.capacity == 0 { return None }
        let full_hash = hash(key);
        let trunc_hash = self.truncate_hash(full_hash);
        let mut bucket = unsafe { self.get_bucket(trunc_hash) };
        for (id, thing) in bucket.values_mut(self) {
            if id == key { return Some(thing) }
        }
        None
    }
    /// If the given entity already exists in this map, return the pointer to
    /// it. If it doesn't, insert it, initialize it, and return the pointer to
    /// it. Second value is true if it was newly-created, false if it already
    /// existed.
    pub fn get_or_insert_with<F: FnOnce(*mut u8)>(&mut self, key: EntityId, initializer: F) -> (*mut u8, bool) {
        if self.capacity == 0 { todo!("expand from nothing") }
        let should_grow = if self.capacity == 0 { true }
        else if self.capacity == 1 {
            match unsafe { self.get_bucket(0) }.values(self).next() {
                None => false,
                Some((x, _)) => x != key,
            }
        }
        else {
            // This is a bit iffy. We will rehash even if we don't need to
            // insert.
            self.num_entries >= self.capacity * MAX_LOAD_FACTOR_NUMERATOR / MAX_LOAD_FACTOR_DENOMINATOR
        };
        if should_grow {
            self.expand_in_place();
        }
        let full_hash = hash(key);
        let trunc_hash = self.truncate_hash(full_hash);
        let mut bucket = unsafe { self.get_bucket(trunc_hash) };
        match bucket.get_or_insert_with(self, key, full_hash, initializer) {
            Err(x) => (x, false),
            Ok(x) => {
                self.num_entries += 1;
                (x, true)
            }
        }
    }
    /// If the given entity already exists in this map, remove it and return
    /// true. If it doesn't, return false.
    pub fn remove(&mut self, key: EntityId) -> bool {
        if self.capacity == 0 { return false }
        let full_hash = hash(key);
        let trunc_hash = self.truncate_hash(full_hash);
        let mut bucket = unsafe { self.get_bucket(trunc_hash) };
        if bucket.remove(self, key) {
            self.num_entries -= 1;
            true
        } else { false }
    }
    /// Iterate over every (entity, value) pair in this `EcHashMap`. Each
    /// iteration will yield a `(EntityId, *const u8)`, giving a **pointer**
    /// to the value.
    /// 
    /// This is an associated method. It allows you to iterate over anything
    /// that references or contains an `EcHashMap`, not just over a direct
    /// reference. In particular, this lets you iterate over an `EcHashMap`
    /// through a lock guard.
    pub fn iter<T: Deref<Target=EcHashMap>>(target: T) -> EcHashMapIter<T> {
        EcHashMapIter {
            map: target,
            next_bucket: 0,
            next_entry: 0,
        }
    }
    /// Iterate over every (entity, value) pair in this `EcHashMap`. Each
    /// iteration will yield a `(EntityId, *mut u8)`, giving a **pointer**
    /// to the value.
    /// 
    /// This is an associated method. It allows you to iterate over anything
    /// that references or contains an `EcHashMap`, not just over a direct
    /// reference. In particular, this lets you iterate over an `EcHashMap`
    /// through a lock guard.
    pub fn iter_mut<T: DerefMut<Target=EcHashMap>>(target: T) -> EcHashMapIterMut<T> {
        EcHashMapIterMut {
            map: target,
            next_bucket: 0,
            next_entry: 0,
        }
    }
    /// Returns the number of values in this `EcHashMap`.
    pub fn len(&self) -> usize {
        self.num_entries as usize
    }
    /// Returns the number of buckets in this `EcHashMap`. This is correlated
    /// with the number of values we will hold before reallocating, but we will
    /// actually reallocate somewhere below 100% utilization.
    pub fn capacity(&self) -> usize {
        self.capacity as usize
    }
    /// Returns the `Layout` for an array of `count` buckets.
    fn bucket_layout_repeat(&self, count: EntityCount) -> Layout {
        unsafe {
            Layout::from_size_align_unchecked(self.bucket_layout_size as usize, self.bucket_layout_align as usize).repeat(count as usize).unwrap().0
        }
    }
    /// Returns the `Layout` for an array of `count` pairs in an *indirect*
    /// bucket.
    fn indirect_bucket_member_layout_repeat(&self, count: Size) -> Layout {
        unsafe {
            let size;
            if count == 0 {
                size = 0;
            }
            else {
                size = (count - 1) as usize * self.indirect_bucket_member_stride as usize
                    + self.indirect_bucket_member_layout_size as usize;
            }
            Layout::from_size_align_unchecked(size, self.indirect_bucket_member_layout_align as usize)
        }
    }
}

impl Clone for EcHashMap {
    fn clone(&self) -> EcHashMap {
        if self.should_contract() {
            // TODO: optimize contracting case
            let mut ret = self.exact_clone();
            ret.contract_in_place();
            ret
        }
        else { self.exact_clone() }
    }
}

impl Drop for EcHashMap {
    fn drop(&mut self) {
        if self.capacity == 0 {
            debug_assert!(self.buckets_ptr.is_null());
            return
        }
        debug_assert!(!self.buckets_ptr.is_null());
        if let Some(dropper) = self.dropper {
            // it's terribly cute that without this `&mut *` I get an error
            for (_, value) in EcHashMap::iter_mut(&mut *self) {
                (dropper)(value);
            }
        }
        unsafe {
            for n in 0 .. self.capacity {
                let mut bucket = self.get_bucket(n);
                bucket.dealloc(self);
            }
            alloc::dealloc(self.buckets_ptr, self.bucket_layout_repeat(self.capacity));
        }
    }
}

impl<T: Deref<Target=EcHashMap>> Iterator for EcHashMapIter<T> {
    type Item = (EntityId, *const u8);

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_bucket < self.map.capacity {
            let bucket = unsafe { self.map.get_bucket(self.next_bucket) };
            if self.next_entry >= bucket.len() {
                self.next_bucket += 1;
                self.next_entry = 0;
                continue
            }
            else {
                let ret = bucket.values(&*self.map).nth(self.next_entry as usize)
                    .map(|(id, ptr)| (id, ptr as *const u8));
                self.next_entry += 1;
                return ret
            }
        }
        None
    }
}

impl<T: Deref<Target=EcHashMap>> Iterator for EcHashMapIterMut<T> {
    type Item = (EntityId, *mut u8);

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_bucket < self.map.capacity {
            let bucket = unsafe { self.map.get_bucket(self.next_bucket) };
            if self.next_entry >= bucket.len() {
                self.next_bucket += 1;
                self.next_entry = 0;
                continue
            }
            else {
                let ret = bucket.values(&*self.map).nth(self.next_entry as usize)
                    .map(|(id, ptr)| (id, ptr));
                // is this transmute (`*const u8` to `*mut u8`) sound?
                let ret = ret.map(|ret| (ret.0, unsafe { transmute(ret.1) }));
                self.next_entry += 1;
                return ret
            }
        }
        None
    }
}

