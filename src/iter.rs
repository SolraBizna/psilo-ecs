//! This module contains glue to allow you to iterate over components. You
//! really, really, really want to use the [ecs_iter!](macro.ecs_iter.html)
//! macro instead of doing any of this yourself.

use std::{
    any::TypeId,
    hint::unreachable_unchecked,
    marker::PhantomData,
};

use crate::{
    Component,
    EntityCount, EntityId,
    echashmap::{EcHashMap, EcHashMapIter, EcHashMapIterMut, hash},
    RwLockReadGuard, RwLockWriteGuard,
    MappedRwLockReadGuard, MappedRwLockWriteGuard,
};

#[doc(hidden)]
/// Internally used by `ecs_iter!` and `ecs_get!`.
pub enum ComponentAccess {
    /// Will reference this component as it existed at the beginning of the
    /// tick. No locking needed. Double buffered only.
    Prev(TypeId, bool),
    /// Will reference this component as it exists now. Shared lock.
    Cur(TypeId, bool),
    /// Will reference this component as it exists now. Exclusive lock.
    Mut(TypeId, bool),
}

impl ComponentAccess {
    fn is_optional(&self) -> bool {
        match self {
            ComponentAccess::Prev(_, optional)
            | ComponentAccess::Cur(_, optional)
            | ComponentAccess::Mut(_, optional)
            => *optional
        }
    }
}

pub enum ComponentIterator<'a> {
    Shared(EcHashMapIter<&'a EcHashMap>),
    Exclusive(EcHashMapIterMut<&'a mut EcHashMap>),
    SharedGd(EcHashMapIter<RwLockReadGuard<'a, EcHashMap>>),
    ExclusiveGd(EcHashMapIterMut<RwLockWriteGuard<'a, EcHashMap>>),
    Empty
}

pub struct ComponentBulkIterator<'a, const N: usize> {
    iterators: [ComponentIterator<'a>; N],
    optional: [bool; N],
    any_optional: bool,
}

pub enum ComponentIterated<'a> {
    Missing,
    Shared(EntityId, *const u8, PhantomData<&'a EcHashMap>),
    Exclusive(EntityId, *mut u8, PhantomData<&'a EcHashMap>),
}

pub enum ComponentGotten<'a> {
    Shared(*const u8),
    SharedGd(MappedRwLockReadGuard<'a, u8>),
    ExclusiveGd(MappedRwLockWriteGuard<'a, u8>),
}

impl<'a, const N: usize> ComponentBulkIterator<'a, N> {
    /// Splits this iterator into two iterators, each of which will iterate
    /// over a different half of the remaining buckets.
    pub fn split<'b>(&'b mut self) -> (ComponentBulkIterator<'b, N>, ComponentBulkIterator<'b, N>) {
        let mut iters = self.iterators.each_mut().map(|x| x.split());
        let left = iters.each_mut().map(|x| ComponentIterator::take(&mut x.0));
        let right = iters.each_mut().map(|x| ComponentIterator::take(&mut x.1));
        (ComponentBulkIterator { iterators: left, optional: self.optional.clone(), any_optional: self.any_optional },
            ComponentBulkIterator { iterators: right, optional: self.optional.clone(), any_optional: self.any_optional })
    }
    /// Returns the number of *buckets* covered by this iterator, not counting
    /// buckets it has already passed over.
    /// 
    /// This is the MINIMUM rank of any of the iterated-upon components.
    pub fn rank(&self) -> EntityCount {
        self.iterators.iter().fold(EntityCount::MAX, |a, x| a.min(x.rank()))
    }
}

impl<'a, const N: usize> Iterator for ComponentBulkIterator<'a, N> {
    type Item = (EntityId, [ComponentIterated<'a>; N]);
    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(N > 0);
        debug_assert!(self.optional[0] == false);
        let mut ret = self.iterators.each_mut().map(|x| x.next()).map(|a| a.map(|a| (hash(a.entity_id()), a)));
        'outer: while !ret.each_ref().iter().enumerate().any(|(n, x)| x.is_none() && !self.optional[n]) {
            let mut min_entity_hash = ret[0].as_ref().unwrap().0;
            let mut max_entity_hash = min_entity_hash;
            if N > 1 {
                for n in 1 .. N {
                    if let Some((entity_hash, _ptr)) = ret[n].as_ref() {
                        if *entity_hash < min_entity_hash { min_entity_hash = *entity_hash; }
                        if *entity_hash > max_entity_hash { max_entity_hash = *entity_hash; }
                    }
                    else {
                        if !self.optional[n] {
                            break 'outer
                        }
                    }
                }
            }
            let (may_yield, max_non_optional) = if min_entity_hash == max_entity_hash {
                // Definitely yield!
                (true, max_entity_hash)
            }
            else if self.any_optional {
                let mut max_non_optional = ret[0].as_ref().unwrap().0;
                for n in 1 .. N {
                    if let Some((entity_hash, _ptr)) = ret[n].as_ref() {
                        if !self.optional[n] && *entity_hash > max_non_optional {
                            max_non_optional = *entity_hash;
                        }
                    }
                    else if !self.optional[n] {
                        return None
                    }
                }
                let mut may_yield = true;
                for n in 0 .. N {
                    if let Some((entity_hash, _ptr)) = ret[n].as_ref() {
                        if self.optional[n] {
                            if *entity_hash > max_non_optional {
                                self.iterators[n].rewind();
                            }
                        }
                        else if *entity_hash != max_non_optional {
                            may_yield = false;
                        }
                    }
                    else {
                        debug_assert!(self.optional[n]);
                    }
                }
                (may_yield, max_non_optional)
            }
            else {
                (false, max_entity_hash)
            };
            if may_yield {
                return Some((ret[0].as_ref().unwrap().1.entity_id(), ret.map(|a| {
                    if let Some(a) = a {
                        if a.0 == max_non_optional { return a.1 }
                    }
                    ComponentIterated::Missing
                })))
            }
            for n in 0 .. N {
                while ret[n].is_some() && ret[n].as_ref().unwrap().0 < max_non_optional {
                    ret[n] = self.iterators[n].next().map(|a| (hash(a.entity_id()), a));
                }
                if ret[n].is_none() && (!self.any_optional || !self.optional[n]) { break 'outer }
            }
        }
        None
    }
}

impl<'a> ComponentIterator<'a> {
    fn take(&mut self) -> ComponentIterator<'a> {
        let mut ret = ComponentIterator::Empty;
        std::mem::swap(self, &mut ret);
        ret
    }
    fn rewind(&mut self) {
        match self {
            ComponentIterator::Shared(x) => x.rewind(),
            ComponentIterator::Exclusive(x) => x.rewind(),
            ComponentIterator::SharedGd(x) => x.rewind(),
            ComponentIterator::ExclusiveGd(x) => x.rewind(),
            ComponentIterator::Empty => (),
        }
    }
    fn split<'b>(&'b mut self) -> (ComponentIterator<'b>, ComponentIterator<'b>) {
        match self {
            ComponentIterator::Shared(x) => {
                let (left_iter, right_iter) = x.split();
                (ComponentIterator::Shared(left_iter), ComponentIterator::Shared(right_iter))   
            },
            ComponentIterator::Exclusive(x) => {
                let (left_iter, right_iter) = x.split();
                (ComponentIterator::Exclusive(left_iter), ComponentIterator::Exclusive(right_iter))   
            },
            ComponentIterator::SharedGd(x) => {
                let (left_iter, right_iter) = x.split();
                (ComponentIterator::Shared(left_iter), ComponentIterator::Shared(right_iter))   
            },
            ComponentIterator::ExclusiveGd(x) => {
                let (left_iter, right_iter) = x.split();
                (ComponentIterator::Exclusive(left_iter), ComponentIterator::Exclusive(right_iter))   
            },
            ComponentIterator::Empty => (ComponentIterator::Empty, ComponentIterator::Empty),
        }
    }
    fn rank(&self) -> EntityCount {
        match self {
            ComponentIterator::Shared(x) => x.rank(),
            ComponentIterator::Exclusive(x) => x.rank(),
            ComponentIterator::SharedGd(x) => x.rank(),
            ComponentIterator::ExclusiveGd(x) => x.rank(),
            ComponentIterator::Empty => 0,
        }
    }
}

impl<'a> Iterator for ComponentIterator<'a> {
    type Item = ComponentIterated<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ComponentIterator::Shared(x) => x.next().map(|(a,b)| ComponentIterated::Shared(a,b,PhantomData)),
            ComponentIterator::Exclusive(x) => x.next().map(|(a,b)| ComponentIterated::Exclusive(a,b,PhantomData)),
            ComponentIterator::SharedGd(x) => x.next().map(|(a,b)| ComponentIterated::Shared(a,b,PhantomData)),
            ComponentIterator::ExclusiveGd(x) => x.next().map(|(a,b)| ComponentIterated::Exclusive(a,b,PhantomData)),
            ComponentIterator::Empty => None,
        }
    }
}

impl<'a> ComponentIterated<'a> {
    fn entity_id(&self) -> EntityId {
        match self {
            ComponentIterated::Missing => panic!("tried to get eid of missing component"),
            ComponentIterated::Shared(eid, _, _) => *eid,
            ComponentIterated::Exclusive(eid, _, _) => *eid,
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_shared<T: Component>(self) -> &'a T {
        match self {
            ComponentIterated::Missing => unreachable_unchecked(),
            ComponentIterated::Shared(_, x, _) => std::mem::transmute::<*const u8, &T>(x),
            ComponentIterated::Exclusive(_, _, _) => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_shared_gd<T: Component>(self) -> &'a T {
        match self {
            ComponentIterated::Missing => unreachable_unchecked(),
            ComponentIterated::Shared(_, x, _) => std::mem::transmute::<*const u8, &T>(x),
            ComponentIterated::Exclusive(_, _, _) => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_exclusive_gd<T: Component>(self) -> &'a mut T {
        match self {
            ComponentIterated::Missing => unreachable_unchecked(),
            ComponentIterated::Shared(_, _, _) => unreachable_unchecked(),
            ComponentIterated::Exclusive(_, x, _) => std::mem::transmute::<*mut u8, &mut T>(x),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_optional_shared<T: Component>(self) -> Option<&'a T> {
        match self {
            ComponentIterated::Missing => None,
            ComponentIterated::Shared(_, x, _) => Some(std::mem::transmute::<*const u8, &T>(x)),
            ComponentIterated::Exclusive(_, _, _) => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_optional_shared_gd<T: Component>(self) -> Option<&'a T> {
        match self {
            ComponentIterated::Missing => None,
            ComponentIterated::Shared(_, x, _) => Some(std::mem::transmute::<*const u8, &T>(x)),
            ComponentIterated::Exclusive(_, _, _) => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_optional_exclusive_gd<T: Component>(self) -> Option<&'a mut T> {
        match self {
            ComponentIterated::Missing => None,
            ComponentIterated::Shared(_, _, _) => unreachable_unchecked(),
            ComponentIterated::Exclusive(_, x, _) => Some(std::mem::transmute::<*mut u8, &mut T>(x)),
        }
    }
}

impl<'a> ComponentGotten<'a> {
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_shared<T: Component>(self) -> &'a T {
        match self {
            ComponentGotten::Shared(x) => std::mem::transmute::<*const u8, &T>(x),
            _ => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_shared_gd<T: Component>(self) -> MappedRwLockReadGuard<'a, T> {
        match self {
            ComponentGotten::SharedGd(x) => MappedRwLockReadGuard::map(x, |x| unsafe { std::mem::transmute::<*const u8, &T>(x) }),
            _ => unreachable_unchecked(),
        }
    }
    #[doc(hidden)]
    /// Unsafe unless used exactly correctly! Please use `ecs_iter!` instead!
    pub unsafe fn unsafe_unwrap_exclusive_gd<T: Component>(self) -> MappedRwLockWriteGuard<'a, T> {
        match self {
            ComponentGotten::ExclusiveGd(x) => MappedRwLockWriteGuard::map(x, |x| unsafe { std::mem::transmute::<*mut u8, &mut T>(x) }),
            _ => unreachable_unchecked(),
        }
    }
}

impl crate::EcsWorld {
    /// Don't call this directly! Use [ecs_iter!](macro.ecs_iter.html) instead.
    #[doc(hidden)]
    pub fn iterate_on<'a, const N: usize>(&'a self, tuple: [ComponentAccess; N]) -> ComponentBulkIterator<'a, N> {
        let ret = ComponentBulkIterator {
            optional: tuple.each_ref().map(|x| x.is_optional()),
            any_optional: tuple.each_ref().iter().fold(false, |a, x| a || x.is_optional()),
            iterators: tuple.map(|access| {
                match access {
                    ComponentAccess::Prev(what, _optional) => {
                        let origin = match self.origin.as_ref() {
                            Some(x) => x,
                            None => panic!("Attempted to perform `prev` iteration outside of a buffered tick and without an explicit origin.\nYou may only perform `prev` iteration inside a call to `Arcow<EcsWorld>::buffered_tick` or `EcsWorld::with_origin`.")
                        };
                        // If we have an origin, we have an immutable reference to
                        // it. If we have an immutable reference to it, it cannot
                        // be ticking, and therefore cannot be mutated, and
                        // therefore its components are safe to access without
                        // locks.
                        debug_assert!(!origin.is_ticking);
                        let lock = match origin.components.get(&what) {
                            Some(x) => x,
                            None => {
                                return ComponentIterator::Empty
                                // panic!("Attempt to iterate on unknown component type (ID = {:?})", access.what());
                            },
                        };
                        let map = unsafe {
                            std::mem::transmute::<*mut EcHashMap, &EcHashMap>(lock.data_ptr())
                        };
                        ComponentIterator::Shared(EcHashMap::iter(map))
                    },
                    ComponentAccess::Cur(what, _optional) => {
                        let lock = match self.components.get(&what) {
                            Some(x) => x,
                            None => return ComponentIterator::Empty,
                        };
                        if cfg!(debug_assertions) && lock.is_locked_exclusive() {
                            panic!("Attempted to immutably iterate upon a \
                                    component type that is currently mutably \
                                    locked!")
                        }
                        let guard = lock.read();
                        ComponentIterator::SharedGd(EcHashMap::iter(guard))
                    },
                    ComponentAccess::Mut(what, _optional) => {
                        if !self.is_ticking {
                            panic!("Attempted to perform `mut` iteration outside of a tick.\nYou may only perform `mut` iteration inside a call to `EcsWorld::unbuffered_tick` or `Arcow<EcsWorld>::buffered_tick`.")
                        }
                        let lock = match self.components.get(&what) {
                            Some(x) => x,
                            None => return ComponentIterator::Empty,
                        };
                        if cfg!(debug_assertions) && lock.is_locked() {
                            panic!("Attempted to mutably iterate upon a \
                                    component type that is currently locked!")
                        }
                        let guard = lock.write();
                        ComponentIterator::ExclusiveGd(EcHashMap::iter_mut(guard))
                    },
                }
        }) };
        assert!(!ret.optional[0]);
        ret
    }
    /// Don't call this directly! Use [ecs_get!](macro.ecs_get.html) instead.
    #[doc(hidden)]
    pub fn get_entity<'a, const N: usize>(&'a self, eid: EntityId, tuple: [ComponentAccess; N]) -> Option<[ComponentGotten; N]> {
        let iterated = tuple.map(|access| {
            match access {
                ComponentAccess::Prev(what, optional) => {
                    if optional { todo!("optional") }
                    let origin = match self.origin.as_ref() {
                        Some(x) => x,
                        None => panic!("Attempted to get a `prev` component outside of a buffered tick and without an explicit origin.\nYou may only get `prev` components inside a call to `Arcow<EcsWorld>::buffered_tick` or `EcsWorld::with_origin`.")
                    };
                    // See iterate_on for justification of unsafe lock bypass
                    // in the origin case.
                    debug_assert!(!origin.is_ticking);
                    let lock = match origin.components.get(&what) {
                        Some(x) => x,
                        None => {
                            return None
                            // panic!("Attempt to iterate on unknown component type (ID = {:?})", access.what());
                        },
                    };
                    let map = unsafe {
                        std::mem::transmute::<*mut EcHashMap, &EcHashMap>(lock.data_ptr())
                    };
                    map.get(eid).map(|component| {
                        ComponentGotten::Shared(component)
                    })
                },
                ComponentAccess::Cur(what, optional) => {
                    if optional { todo!("optional") }
                    let lock = match self.components.get(&what) {
                        Some(x) => x,
                        None => return None,
                    };
                    if cfg!(debug_assertions) && lock.is_locked_exclusive() {
                        panic!("Attempted to immutably get a \
                                component type that is currently mutably \
                                locked!")
                    }
                    let guard = lock.read();
                    guard.get(eid).map(|component| {
                        ComponentGotten::SharedGd(RwLockReadGuard::map(guard, |_| unsafe { component.as_ref().unwrap() }))
                    })
                },
                ComponentAccess::Mut(what, optional) => {
                    if optional { todo!("optional") }
                    if !self.is_ticking {
                        panic!("Attempted to get a `mut` component outside of a tick.\nYou may only mutate components inside a call to `EcsWorld::unbuffered_tick` or `Arcow<EcsWorld>::buffered_tick`.")
                    }
                    let lock = match self.components.get(&what) {
                        Some(x) => x,
                        None => return None,
                    };
                    if cfg!(debug_assertions) && lock.is_locked() {
                        panic!("Attempted to get a `mut` component of a \
                                component type that is currently locked!")
                    }
                    let mut guard = lock.write();
                    guard.get_mut(eid).map(|component| {
                        ComponentGotten::ExclusiveGd(RwLockWriteGuard::map(guard, |_| unsafe { component.as_mut().unwrap() }))
                    })
                },
            }
        });
        if iterated.iter().any(|x| x.is_none()) { None }
        else {
            Some(iterated.map(|x| x.unwrap()))
        }
    }
}

// TODO: proc macro :(
#[macro_export]
macro_rules! ecs_iter_accessor {
    (prev_optional $wat:ty) => {
        $crate::iter::ComponentAccess::Prev(std::any::TypeId::of::<$wat>(), true)
    };
    (cur_optional $wat:ty) => {
        $crate::iter::ComponentAccess::Cur(std::any::TypeId::of::<$wat>(), true)
    };
    (mut_optional $wat:ty) => {
        $crate::iter::ComponentAccess::Mut(std::any::TypeId::of::<$wat>(), true)
    };
    (prev $wat:ty) => {
        $crate::iter::ComponentAccess::Prev(std::any::TypeId::of::<$wat>(), false)
    };
    (cur $wat:ty) => {
        $crate::iter::ComponentAccess::Cur(std::any::TypeId::of::<$wat>(), false)
    };
    (mut $wat:ty) => {
        $crate::iter::ComponentAccess::Mut(std::any::TypeId::of::<$wat>(), false)
    };
}

#[macro_export]
macro_rules! ecs_iter_iterated {
    ($el:expr, prev_optional $wat:ty) => {
        $el.unsafe_optional_shared::<$wat>()
    };
    ($el:expr, cur_optional $wat:ty) => {
        $el.unsafe_optional_shared_gd::<$wat>()
    };
    ($el:expr, mut_optional $wat:ty) => {
        $el.unsafe_optional_exclusive_gd::<$wat>()
    };
    ($el:expr, prev $wat:ty) => {
        $el.unsafe_unwrap_shared::<$wat>()
    };
    ($el:expr, cur $wat:ty) => {
        $el.unsafe_unwrap_shared_gd::<$wat>()
    };
    ($el:expr, mut $wat:ty) => {
        $el.unsafe_unwrap_exclusive_gd::<$wat>()
    };
}

#[macro_export]
macro_rules! ecs_iter {
    ($world:expr, $($comps:tt)+) => {
        {
            let world = $world.as_ref();
            let iterants = $crate::ecs_iter_accessors!($($comps)+);
            let iterator = world.iterate_on(iterants);
            iterator.map(|(eid, array)| {
                $crate::ecs_iterated_unfold!(eid, array, $($comps)+)
            })
        }
    };
}

#[macro_export]
macro_rules! ecs_get {
    ($world:expr, $eid:expr, $($comps:tt)+) => {
        {
            let world = $world.as_ref();
            let iterants = $crate::ecs_iter_accessors!($($comps)+);
            let found = world.get_entity($eid, iterants);
            found.map(|array| {
                $crate::ecs_gotten_unfold!(array, $($comps)+)
            })
        }
    };
}
