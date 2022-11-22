use super::*;

use std::{
    borrow::Borrow,
    collections::HashMap,
    mem::ManuallyDrop,
    rc::Rc,
    sync::{Arc, RwLock},
};

use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128StarStar;

type Uke = (i32, i64, i8);
const TEST_UKES: [Uke; 7] = [
    (173, 467321476, 52),
    (32789, 777643, 60),
    (732, 0, 70),
    (73117, 8887324767, 71),
    (897, 64376, 75),
    (314, 159265358, 76),
    (0x48656c6c, 0x6f2c206e656d6f21, 80),
];
fn create_uke_map(capacity: usize) -> EcHashMap {
    EcHashMap::with_capacity(Layout::new::<Uke>(), None, clone_uke, capacity)
}
fn clone_uke(dst: *mut u8, src: *const u8) {
    unsafe {
        transmute::<_, *mut Uke>(dst).write(transmute::<_, *const Uke>(src).read())
    }
}
fn write_uke(ptr: *mut u8, uke: Uke) {
    unsafe { transmute::<_, *mut Uke>(ptr).write(uke) }
}
fn insert_ukes<T: Borrow<Uke>, I: Iterator<Item=(usize, T)>, R: DerefMut<Target=EcHashMap>>(mut map: R, iter: I) {
    for (i, uke) in iter {
        assert!(map.get_or_insert_with(i as EntityId, |ptr| write_uke(ptr, uke.borrow().clone())).1);
    }
}
fn iter_mut_ukes<'a, R: 'a + DerefMut<Target=EcHashMap>>(map: R) -> std::iter::Map<EcHashMapIterMut<R>, fn((EntityId, *mut u8)) -> (EntityId, &'a mut Uke)> {
    EcHashMap::iter_mut(map).map(|(i, x)| (i, unsafe { transmute::<*mut u8, &mut Uke>(x) }))
}
fn check_ukes<T: Borrow<Uke>, I: Iterator<Item=(usize, T)>, R: Deref<Target=EcHashMap>>(map: R, iter: I) {
    let mut seen = vec![];
    for (id, uke) in EcHashMap::iter(map) {
        let id = id as usize;
        if seen.len() <= id {
            seen.resize_with(id+1, || Some(Vec::new()));
        }
        seen[id].as_mut().unwrap().push(unsafe { *transmute::<*const u8, &Uke>(uke) });
    }
    let mut ok = true;
    for (id, uke) in iter {
        match seen.get(id).and_then(|x| x.as_ref()) {
            Some(found) => {
                eprintln!("[{}] = {:?} = {:?}", id, uke.borrow(), found);
                seen[id] = None;
            },
            None => {
                eprintln!("[{}] = {:?} = NOTHING", id, uke.borrow());
                ok = false;
            },
        }
    }
    for (id, found) in seen.iter().enumerate() {
        if let Some(found) = found {
            if found.len() != 0 {
                eprintln!("[{}] = NOTHING = {:?}", id, found);
                ok = false;
            }
            else {
                eprintln!("[{}] = NOTHING = NOTHING", id);
            }
        }
    }
    if !ok { panic!() }
}
#[test]
fn direct_iter() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(16);
    eprintln!("{:?}", test_map);
    assert_eq!(test_map.len(), 0);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    assert_eq!(test_map.len(), TEST_UKES.len());
    check_ukes(&test_map, TEST_UKES.iter().enumerate());
}
#[test]
fn ex_nihilo_in_nihilum() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(0);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    check_ukes(&test_map, TEST_UKES.iter().enumerate());
    while test_map.capacity() > 1 {
        let old_capacity = test_map.capacity();
        test_map.contract_in_place();
        let new_capacity = test_map.capacity();
        // note: not the other way around (old_capacity / 2) == new_capacity
        assert_eq!(new_capacity * 2, old_capacity);
    }
    assert_eq!(test_map.capacity(), 1);
    test_map.contract_in_place();
    assert_eq!(test_map.capacity(), 1);
}
#[test]
fn guarded_iter() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let test_map = RwLock::new(create_uke_map(16));
    eprintln!("{:?}", test_map);
    insert_ukes(test_map.write().unwrap(), TEST_UKES.iter().enumerate());
    check_ukes(test_map.read().unwrap(), TEST_UKES.iter().enumerate());
}
#[test]
fn iter_mut() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(16);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate().map(|(i,(a,b,c))| (i, (*a, b - i as i64, *c))));
    check_ukes(&test_map, TEST_UKES.iter().enumerate().map(|(i,(a,b,c))| (i, (*a, b - i as i64, *c))));
    for (key, uke) in iter_mut_ukes(&mut test_map) {
        uke.1 += key as i64;
    }
    check_ukes(&test_map, TEST_UKES.iter().enumerate());
}
#[test]
fn crowded() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(4);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    check_ukes(&test_map, TEST_UKES.iter().enumerate());
}
#[test]
fn too_crowded() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(2);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    check_ukes(&test_map, TEST_UKES.iter().enumerate());
}
#[test]
fn crowded_remove() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(4);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    while test_map.capacity > 4 {
        test_map.contract_in_place();
    }
    for i in 0 .. TEST_UKES.len() {
        test_map.remove(i as EntityId);
    }
    assert_eq!(test_map.len(), 0);
    check_ukes(&test_map, TEST_UKES[0..0].iter().enumerate());
}
#[test]
fn too_crowded_remove() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(2);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    while test_map.capacity > 2 {
        test_map.contract_in_place();
    }
    for i in 0 .. TEST_UKES.len() {
        test_map.remove(i as EntityId);
    }
    assert_eq!(test_map.len(), 0);
    check_ukes(&test_map, TEST_UKES[0..0].iter().enumerate());
}
#[test]
fn too_crowded_remove_some() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(2);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    for i in 0 .. TEST_UKES.len() {
        if i % 2 == 1 {
            test_map.remove(i as EntityId);
        }
    }
    check_ukes(&test_map, TEST_UKES.iter().enumerate().filter_map(|(i, uke)| {
        if i % 2 == 0 { Some((i, uke)) } else { None }
    }));
}
#[test]
fn shrunken_clone() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let mut test_map = create_uke_map(8);
    eprintln!("{:?}", test_map);
    insert_ukes(&mut test_map, TEST_UKES.iter().enumerate());
    for i in 1 .. TEST_UKES.len() {
        test_map.remove(i as EntityId);
    }
    assert_eq!(test_map.len(), 1);
    assert!(test_map.should_contract());
    check_ukes(&test_map, TEST_UKES[0..1].iter().enumerate());
    while test_map.should_contract() {
        let test_map_2 = test_map.clone();
        check_ukes(&test_map_2, TEST_UKES[0..1].iter().enumerate());
        assert_eq!(test_map_2.len(), test_map.len());
        test_map = test_map_2;
    }
}
#[test]
fn drop_and_clone() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    let rcs = [4, 5, 6].map(Rc::new);
    let mut test_map = EcHashMap::with_capacity(Layout::new::<Rc<i32>>(), Some(|x| {
        eprintln!("Manually dropping: {:?}", x);
        unsafe { ManuallyDrop::drop(transmute::<*mut u8, &mut ManuallyDrop<Rc<i32>>>(x)) }
    }), |dst, src| {
        eprintln!("Cloning {:?} <- {:?}", dst, src);
        unsafe {
            transmute::<*mut u8, *mut Rc<i32>>(dst).write(transmute::<*const u8, &Rc<i32>>(src).clone())
        }
    }, 2);
    eprintln!("{:?}", test_map);
    for (i, rc) in rcs.iter().enumerate() {
        test_map.get_or_insert_with(i as EntityId, |x| {
            unsafe {
                transmute::<*mut u8, *mut Rc<i32>>(x).write(rc.clone())
            }
        });
    }
    eprintln!("Bucket status:");
    for n in 0 .. test_map.capacity {
        eprintln!("[{}] = {:?}", n, unsafe { test_map.get_bucket(n as EntityHash) }.values(&test_map).map(|(x, _)| x).collect::<Vec<EntityId>>());
    }
    // check that all the Rcs were cloned
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [2, 2, 2]);
    // check that a removed Rc is dropped
    assert!(test_map.remove(1));
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [2, 1, 2]);
    // check that a double remove won't lead to a double drop
    assert!(!test_map.remove(1));
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [2, 1, 2]);
    // clone the map
    eprintln!("test_map pointer = {:?}", test_map.buckets_ptr);
    let mut test_map_2 = test_map.exact_clone();
    eprintln!("test_map_2 pointer = {:?}", test_map_2.buckets_ptr);
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [3, 1, 3]);
    // remove an entry from one map but not the other
    assert!(test_map_2.remove(2));
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [3, 1, 2]);
    assert!(!test_map_2.remove(2));
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [3, 1, 2]);
    // check that dropping the map drops the remaining Rcs
    drop(test_map);
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [2, 1, 1]);
    drop(test_map_2);
    assert_eq!(rcs.each_ref().map(Rc::strong_count), [1, 1, 1]);
}

#[repr(C)]
#[derive(Debug,Clone)]
struct RealBig {
    clone_tracker: Arc<EntityId>,
    bytes: [u8; 456],
    extra: u8, 
}

impl AsRef<RealBig> for RealBig {
    fn as_ref(&self) -> &Self { self }
}

fn compare_std_and_ec_maps<T: AsRef<RealBig>>(std_map: &HashMap<EntityId, T>, ecs_map: &EcHashMap) {
    for (&key, std_value) in std_map.iter() {
        let std_value = std_value.as_ref();
        assert_eq!(*std_value.clone_tracker, key);
        let ecs_value = ecs_map.get(key).expect("value in std_map missing from ecs_map");
        let ecs_value = unsafe { transmute::<*const u8, &RealBig>(ecs_value) };
        assert_eq!(&*std_value.clone_tracker as *const EntityId,
            &*ecs_value.clone_tracker as *const EntityId);
        assert_eq!(std_value.bytes, ecs_value.bytes);
    }
    for (key, _std_value) in EcHashMap::iter(ecs_map) {
        std_map.get(&key).expect("value in ecs_map missing from std_map");
    }
    for n in 0 .. ecs_map.capacity() {
        let mut last_hash = None;
        let bucket;
        unsafe {
            bucket = ecs_map.get_bucket(n as EntityHash);
        }
        for (key, _) in bucket.values(ecs_map) {
            let hash = crate::echashmap::hash(key);
            if let Some(last_hash) = last_hash {
                assert!(hash > last_hash);
            }
            last_hash = Some(hash);
        }
    }
}

fn compare_std_maps<A: AsRef<RealBig>, B: AsRef<RealBig>>(map_a: &HashMap<EntityId, A>, map_b: &HashMap<EntityId, B>) {
    for (&key, a_value) in map_a.iter() {
        let a_value = a_value.as_ref();
        assert_eq!(*a_value.clone_tracker, key);
        let b_value = map_b.get(&key).expect("value in map_a missing from map_b");
        let b_value = b_value.as_ref();
        assert_eq!(&*a_value.clone_tracker as *const EntityId,
            &*b_value.clone_tracker as *const EntityId);
        assert_eq!(a_value.bytes, b_value.bytes);
    }
    for (&key, b_value) in map_b.iter() {
        let b_value = b_value.as_ref();
        assert_eq!(*b_value.clone_tracker, key);
        let a_value = map_a.get(&key).expect("value in map_b missing from map_a");
        let a_value = a_value.as_ref();
        assert_eq!(&*b_value.clone_tracker as *const EntityId,
            &*a_value.clone_tracker as *const EntityId);
        assert_eq!(b_value.bytes, a_value.bytes);
    }
}

fn accumulate_entries_from_split_iter(iter: EcHashMapIter<&EcHashMap>) -> HashMap<EntityId, &RealBig> {
    use std::collections::hash_map::Entry as HashMapEntry;
    let mut new_map = HashMap::new();
    for (key, value) in iter {
        match new_map.entry(key) {
            HashMapEntry::Occupied(_) => panic!("DUPE WITHIN SPLIT!"),
            HashMapEntry::Vacant(x) => { x.insert(unsafe { transmute::<*const u8, &RealBig>(value) }); }
        }
    }
    new_map
}

fn merge_maps<V>(mut a: HashMap<EntityId, V>, b: HashMap<EntityId, V>) -> HashMap<EntityId, V> {
    use std::collections::hash_map::Entry as HashMapEntry;
    for (key, value) in b.into_iter() {
        match a.entry(key) {
            HashMapEntry::Occupied(_) => panic!("DUPE ACROSS SPLIT!"),
            HashMapEntry::Vacant(x) => { x.insert(value); }
        }
    }
    a
}

fn compare_std_and_ec_maps_split1(std_map: &HashMap<EntityId, RealBig>, ecs_map: &EcHashMap) {
    let iter = EcHashMap::iter(ecs_map);
    let (left_iter, right_iter) = iter.split();
    let (left_map, right_map) = rayon::join(
        || {accumulate_entries_from_split_iter(left_iter)},
        || {accumulate_entries_from_split_iter(right_iter)},
    );
    let true_map = merge_maps(left_map, right_map);
    compare_std_maps(std_map, &true_map);
}

fn compare_std_and_ec_maps_split2(std_map: &HashMap<EntityId, RealBig>, ecs_map: &EcHashMap) {
    let iter = EcHashMap::iter(ecs_map);
    let (left_iter, right_iter) = iter.split();
    let (ll_iter, lr_iter) = left_iter.split();
    let (rl_iter, rr_iter) = right_iter.split();
    let (left_map, right_map) = rayon::join(
        || {
            let (left_map, right_map) = rayon::join(
                || {
                    accumulate_entries_from_split_iter(ll_iter)
                },
                || {
                    accumulate_entries_from_split_iter(lr_iter)
                }
            );
            merge_maps(left_map, right_map)
        },
        || {
            let (left_map, right_map) = rayon::join(
                || {
                    accumulate_entries_from_split_iter(rl_iter)
                },
                || {
                    accumulate_entries_from_split_iter(rr_iter)
                }
            );
            merge_maps(left_map, right_map)
        },
    );
    let true_map = merge_maps(left_map, right_map);
    compare_std_maps(std_map, &true_map);
}

#[test]
fn real_big_test() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    real_big_test_body();
}

fn real_big_test_body() {
    const TEST_ENTITY_COUNT: usize = 16384;
    const TEST_SEED: u64 = 0x4E95;
    let mut rng = Xoshiro128StarStar::seed_from_u64(TEST_SEED);
    let mut ecs_map = EcHashMap::with_capacity(Layout::new::<RealBig>(), Some(|x| {
        unsafe { ManuallyDrop::drop(transmute::<*mut u8, &mut ManuallyDrop<RealBig>>(x)) }
    }), |dst, src| {
        unsafe {
            transmute::<*mut u8, *mut RealBig>(dst).write(transmute::<*const u8, &RealBig>(src).clone())
        }
    }, 0);
    eprintln!("{:?}", ecs_map);
    let mut std_map = HashMap::new();
    for n in 0 .. TEST_ENTITY_COUNT {
        let entity_id = loop {
            let candidate = rng.gen::<EntityId>();
            if std_map.contains_key(&candidate) { continue }
            else { break candidate }
        };
        let mut bytes = [0u8; 456];
        rng.fill_bytes(&mut bytes[..]);
        let value = RealBig { clone_tracker: Arc::new(entity_id), bytes, extra: 0x45 };
        assert!(ecs_map.get_or_insert_with(entity_id, |dst| {
            unsafe { transmute::<*mut u8, *mut RealBig>(dst).write(value.clone()) }
        }).1);
        assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
        std_map.insert(entity_id, value);
        assert!(ecs_map.len() == n + 1);
        assert!(std_map.len() == n + 1);
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    // make mutant clones
    let mut std_map_clone = std_map.clone();
    let mut ecs_map_clone = ecs_map.clone();
    for (key, ecs_value) in EcHashMap::iter_mut(&mut ecs_map_clone) {
        let ecs_value = unsafe { transmute::<*mut u8, &mut RealBig>(ecs_value) };
        let std_value = std_map_clone.get_mut(&key).unwrap();
        ecs_value.bytes[420] ^= 69;
        std_value.bytes[420] ^= 69;
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    compare_std_and_ec_maps(&std_map_clone, &ecs_map_clone);
    // mutate them further
    for (&key, value) in std_map.iter() {
        assert_eq!(Arc::strong_count(&value.clone_tracker), 4);
        if rng.gen_bool(0.5) {
            assert!(std_map_clone.remove(&key).is_some());
            assert!(ecs_map_clone.remove(key));
            assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
        }
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    compare_std_and_ec_maps(&std_map_clone, &ecs_map_clone);
    drop(std_map_clone);
    drop(ecs_map_clone);
    compare_std_and_ec_maps(&std_map, &ecs_map);
    for (_, value) in std_map.iter() {
        assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
    }
}

// If you run this test, consider running it with
// `RUSTFLAGS=-Zsanitizer=address`
#[test] #[ignore]
fn supernova_test() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    const TEST_ENTITY_COUNT: usize = 16384;
    const TEST_SEED: u64 = 0x1F346;
    let mut rng = Xoshiro128StarStar::seed_from_u64(TEST_SEED);
    let mut ecs_map = EcHashMap::with_capacity(Layout::new::<RealBig>(), Some(|x| {
        unsafe { ManuallyDrop::drop(transmute::<*mut u8, &mut ManuallyDrop<RealBig>>(x)) }
    }), |dst, src| {
        unsafe {
            transmute::<*mut u8, *mut RealBig>(dst).write(transmute::<*const u8, &RealBig>(src).clone())
        }
    }, 0);
    eprintln!("{:?}", ecs_map);
    let mut std_map = HashMap::new();
    for n in 0 .. TEST_ENTITY_COUNT {
        let entity_id = loop {
            let candidate = rng.gen::<EntityId>();
            if std_map.contains_key(&candidate) { continue }
            else { break candidate }
        };
        let mut bytes = [0u8; 456];
        rng.fill_bytes(&mut bytes[..]);
        let value = RealBig { clone_tracker: Arc::new(entity_id), bytes, extra: 0x45 };
        assert!(ecs_map.get_or_insert_with(entity_id, |dst| {
            unsafe { transmute::<*mut u8, *mut RealBig>(dst).write(value.clone()) }
        }).1);
        assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
        std_map.insert(entity_id, value);
        assert!(ecs_map.len() == n + 1);
        assert!(std_map.len() == n + 1);
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    for (_, value) in std_map.iter() {
        assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
    }
    // now drop it into a neutron star
    while ecs_map.capacity() > 1 {
        ecs_map.contract_in_place();
        compare_std_and_ec_maps(&std_map, &ecs_map);
    }
    // and let it go supernova
    while ecs_map.capacity() < ecs_map.len() * 16 {
        ecs_map.expand_in_place();
        compare_std_and_ec_maps(&std_map, &ecs_map);
    }
}

#[test] #[ignore]
fn zst_supernova_test() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    const TEST_ENTITY_COUNT: usize = 16384;
    const TEST_SEED: u64 = 0x1F346;
    let mut rng = Xoshiro128StarStar::seed_from_u64(TEST_SEED);
    let mut ecs_map = EcHashMap::with_capacity(Layout::new::<()>(), None, |_dst, _src| {}, 0);
    eprintln!("{:?}", ecs_map);
    for _ in 0 .. TEST_ENTITY_COUNT {
        let entity_id = loop {
            let candidate = rng.gen::<EntityId>();
            if ecs_map.get(candidate).is_some() { continue }
            else { break candidate }
        };
        assert!(ecs_map.get_or_insert_with(entity_id, |_dst| {}).1);
    }
    for (_, _) in EcHashMap::iter(&ecs_map) {}
    for (_, _) in EcHashMap::iter_mut(&mut ecs_map) {}
    // now drop it into a neutron star
    while ecs_map.capacity() > 1 {
        ecs_map.contract_in_place();
        for (_, _) in EcHashMap::iter(&ecs_map) {}
        for (_, _) in EcHashMap::iter_mut(&mut ecs_map) {}
        if ecs_map.capacity() == 512 {
            let mut ecs_map_two = ecs_map.clone();
            for (eid, _) in EcHashMap::iter(&ecs_map) {
                if eid % 2 == 0 {
                    ecs_map_two.remove(eid);
                }
            }
            for (eid, _) in EcHashMap::iter(&ecs_map) {
                if eid % 2 == 1 {
                    ecs_map_two.remove(eid);
                }
            }
            for (eid, _) in EcHashMap::iter(&ecs_map_two) {
                println!("{}", eid);
            }
        }
    }
    // and let it go supernova
    while ecs_map.capacity() < ecs_map.len() * 16 {
        ecs_map.expand_in_place();
        for (_, _) in EcHashMap::iter(&ecs_map) {}
        for (_, _) in EcHashMap::iter_mut(&mut ecs_map) {}
    }
}

#[test]
fn rayon_test() {
    #[cfg(feature="dynamic-hash")]
    let _guard = dynamic_hash_guard();
    const TEST_ENTITY_COUNT: usize = 16384;
    const TEST_SEED: u64 = 0x4E95;
    let mut rng = Xoshiro128StarStar::seed_from_u64(TEST_SEED);
    let mut ecs_map = EcHashMap::with_capacity(Layout::new::<RealBig>(), Some(|x| {
        unsafe { ManuallyDrop::drop(transmute::<*mut u8, &mut ManuallyDrop<RealBig>>(x)) }
    }), |dst, src| {
        unsafe {
            transmute::<*mut u8, *mut RealBig>(dst).write(transmute::<*const u8, &RealBig>(src).clone())
        }
    }, 0);
    eprintln!("{:?}", ecs_map);
    let mut std_map = HashMap::new();
    for n in 0 .. TEST_ENTITY_COUNT {
        let entity_id = loop {
            let candidate = rng.gen::<EntityId>();
            if std_map.contains_key(&candidate) { continue }
            else { break candidate }
        };
        let mut bytes = [0u8; 456];
        rng.fill_bytes(&mut bytes[..]);
        let value = RealBig { clone_tracker: Arc::new(entity_id), bytes, extra: 0x45 };
        assert!(ecs_map.get_or_insert_with(entity_id, |dst| {
            unsafe { transmute::<*mut u8, *mut RealBig>(dst).write(value.clone()) }
        }).1);
        assert_eq!(Arc::strong_count(&value.clone_tracker), 2);
        std_map.insert(entity_id, value);
        assert!(ecs_map.len() == n + 1);
        assert!(std_map.len() == n + 1);
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    compare_std_and_ec_maps_split1(&std_map, &ecs_map);
    compare_std_and_ec_maps_split2(&std_map, &ecs_map);
    // make mutant clones
    let mut std_map_clone = std_map.clone();
    let mut ecs_map_clone = ecs_map.clone();
    let mut mutiter = EcHashMap::iter_mut(&mut ecs_map_clone);
    let (left_iter, right_iter) = mutiter.split();
    rayon::join(|| {
            for (_, ecs_value) in left_iter {
                let ecs_value = unsafe { transmute::<*mut u8, &mut RealBig>(ecs_value) };
                ecs_value.bytes[420] ^= 69;
            }
        }, || {
            for (_, ecs_value) in right_iter {
                let ecs_value = unsafe { transmute::<*mut u8, &mut RealBig>(ecs_value) };
                ecs_value.bytes[420] ^= 69;
            }
    });
    for (_, std_value) in std_map_clone.iter_mut() {
        std_value.bytes[420] ^= 69;
    }
    compare_std_and_ec_maps(&std_map, &ecs_map);
    compare_std_and_ec_maps(&std_map_clone, &ecs_map_clone);
    // good enough for me
}

#[cfg(feature="dynamic-hash")]
#[test]
fn real_dynamic_test() {
    if std::env::var(super::dynamic_hash::ECHASHMAP_DYNAMIC_PRIME).is_ok() {
        return;
    }
    let _guard = DYNAMIC_HASH_GUARD.write().unwrap();
    for _ in 0 .. 8 {
        unsafe { crate::echashmap::dynamic_hash::force_rehash() };
        real_big_test_body();
    }
}

#[cfg(feature="dynamic-hash")]
static DYNAMIC_HASH_GUARD: std::sync::RwLock<()> = std::sync::RwLock::new(());
/// Prevents the real_dynamic_test from running in parallel with any of the
/// other EcHashMap tests, which would corrupt their hash tables mid run.
#[cfg(feature="dynamic-hash")]
fn dynamic_hash_guard() -> std::sync::RwLockReadGuard<'static, ()> {
    DYNAMIC_HASH_GUARD.read().unwrap()
}