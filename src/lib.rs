#![feature(alloc_layout_extra, array_methods, core_intrinsics, fn_traits, hash_drain_filter, unboxed_closures)]

use std::{
    any::TypeId,
    collections::{HashMap, HashSet, hash_map::Entry as HashMapEntry},
    sync::Arc,
};

use arcow::Arcow;
use parking_lot::{
    RwLock,
    RwLockReadGuard, MappedRwLockReadGuard,
    RwLockWriteGuard, MappedRwLockWriteGuard
};

#[cfg(feature="u64-entity-ids")]
pub type EntityId = u64;
#[cfg(not(feature="u64-entity-ids"))]
pub type EntityId = u32;
type EntityCount = EntityId;
type EntityHash = EntityId;

mod echashmap;
use echashmap::EcHashMap;
#[cfg(test)]
mod test;

pub mod iter;

#[cfg(feature="randomize-entity-ids")]
const ENTITY_ID_LIMBO_TICKS: u16 = 5;

pub struct EcsSchema {
    #[cfg(debug_assertions)]
    component_names: HashMap<TypeId, &'static str>,
}

pub struct EcsWorld {
    /// Central depository of information about all component types we know
    /// about. Currently very anemic, as a lot of this data is actually stored
    /// in `EcHashMap`.
    schema: Arc<RwLock<EcsSchema>>,
    /// All entities that are considered to exist in this world.
    entities: HashSet<EntityId>,
    /// For each component type, a hash map that maps every entity ID that has
    /// the particular component to the instance of that component for that
    /// entity.
    components: HashMap<TypeId, RwLock<EcHashMap>>,
    /// Only exists if we're double buffered
    origin: Option<Arcow<EcsWorld>>,
    /// `true` if we are currently "hot" (mutably borrowed and ticking),
    /// `false` if we are "cold" (not ticking, therefore possibly shared
    /// borrowed)
    is_ticking: bool,
    /// If we are not randomizing entity IDs, this is the next entity ID to
    /// [attempt to] use. Only exists without feature `randomize-entity-ids`.
    #[cfg(not(feature="randomize-entity-ids"))]
    next_entity_id: EntityId,
    /// Entities marked for deletion.
    entities_to_delete: RwLock<HashSet<EntityId>>,
    /// Entity IDs that have been deleted recently. They will not be available
    /// for reuse for five ticks, which should hopefully give enough time for
    /// any ID-based references to that entity to notice that it has died and
    /// go away.
    /// 
    /// Exists only when we are randomizing entity IDs. For nonrandom entity
    /// IDs:
    /// 
    /// - 32-bit: We hope that there are not close enough to four billion
    ///   active entities for rollover to actively be a problem.
    /// - 64-bit: We never re-use entity IDs, period.
    #[cfg(feature="randomize-entity-ids")]
    entity_ids_in_limbo: RwLock<HashMap<EntityId, u16>>,
}

/// This is a supertrait that all types that can be used as Components must
/// implement. Your Component type must be:
/// - `Clone` (so that double buffering can work)
/// - `Send` (because we might iterate in multiple threads)
/// - `Sized` (so that it fits in our custom hash map)
/// - `'static` (so that it ... exists.)
pub trait Component : Clone + Send + Sized + 'static {}
impl<T: Clone + Send + Sized + 'static> Component for T {}

impl EcsSchema {
    pub fn new() -> EcsSchema {
        EcsSchema { #[cfg(debug_assertions)] component_names: HashMap::new(), }
    }
}

impl EcsWorld {
    pub fn as_ref(&self) -> &EcsWorld { self }
    pub fn as_mut(&mut self) -> &mut EcsWorld { self }
    pub fn new(schema: Arc<RwLock<EcsSchema>>) -> EcsWorld {
        EcsWorld {
            schema,
            entities: HashSet::new(),
            components: HashMap::new(),
            origin: None,
            is_ticking: false,
            #[cfg(not(feature="randomize-entity-ids"))]
            next_entity_id: 1,
            entities_to_delete: RwLock::new(HashSet::new()),
            #[cfg(feature="randomize-entity-ids")]
            entity_ids_in_limbo: RwLock::new(HashMap::new()),
        }
    }
    pub fn with_blank_schema() -> EcsWorld {
        EcsWorld::new(Arc::new(RwLock::new(EcsSchema::new())))
    }
    #[cfg(all(not(feature="randomize-entity-ids"),feature="u64-entity-ids"))]
    fn make_new_entity_id(&mut self) -> EntityId {
        // With non-random 64-bit entity IDs, we assume there will never be a
        // wrap, and therefore will never be a collision.
        let ret = self.next_entity_id;
        self.next_entity_id = self.next_entity_id.wrapping_add(1);
        #[cfg(not(feature="gotta-go-fast"))]
        assert_ne!(self.next_entity_id, 0);
        ret
    }
    #[cfg(all(not(feature="randomize-entity-ids"),not(feature="u64-entity-ids")))]
    fn make_new_entity_id(&mut self) -> EntityId {
        // With non-random 32-bit entity IDs, there may be a wrap, and
        // therefore there may be a collision.
        loop {
            let ret = self.next_entity_id;
            self.next_entity_id = self.next_entity_id.wrapping_add(1);
            if ret != 0 && !self.entities.contains(&ret) { return ret; }
        }
    }
    #[cfg(feature="randomize-entity-ids")]
    fn make_new_entity_id(&mut self) -> EntityId {
        // With random entity IDs, we must obviously check for collisions.
        use rand::{Rng, thread_rng};
        let mut rng = thread_rng();
        let entity_ids_in_limbo = self.entity_ids_in_limbo.read();
        loop {
            let ret = rng.gen();
            if ret != 0 && !self.entities.contains(&ret)
            && !entity_ids_in_limbo.contains_key(&ret) { return ret; }
        }
    }
    /// Returns true if an entity with the given ID exists, false if it never
    /// existed or has been deleted.
    pub fn entity_exists(&self, eid: EntityId) -> bool {
        self.entities.contains(&eid)
    }
    /// Count the number of entities that exist and have the given component.
    /// If you want all the entities that exist with more than one component,
    /// use `ecs_iter!` and count the results.
    pub fn count<T: Component>(&self) -> usize {
        match self.components.get(&TypeId::of::<T>()) {
            None => 0,
            Some(x) => x.read().len() as usize,
        }
    }
    /// Creates a new entity with no components. You should use `ecs_spawn!`
    /// instead.
    #[doc(hidden)]
    pub fn spawn(&mut self) -> EntityId {
        let new_id = self.make_new_entity_id();
        self.entities.insert(new_id);
        new_id
    }
    /// Attaches a component to the given entity. You should use `ecs_spawn!`
    /// instead.
    #[doc(hidden)]
    pub fn attach_new_component<T: Component>(&mut self, eid: EntityId, component: T) {
        #[cfg(debug_assertions)]
        if !self.entities.contains(&eid) {
            panic!("Attempted to attach a Component of type {:?} to an Entity that did not exist!", std::any::type_name::<T>());
        }
        let tid = TypeId::of::<T>();
        let initializer = move |dst| unsafe {
            let dst = std::mem::transmute::<*mut u8, *mut T>(dst);
            dst.write(component);
        };
        match self.components.entry(tid) {
            HashMapEntry::Occupied(ent) => {
                let mut comps = ent.get().write();
                if !comps.get_or_insert_with(eid, initializer).1 {
                    panic!("Attempted to attach a Component of type {:?} to the same Entity twice!", std::any::type_name::<T>());
                }
            },
            HashMapEntry::Vacant(ent) => {
                #[cfg(debug_assertions)] {
                    let schema = self.schema.read();
                    if !schema.component_names.contains_key(&tid) {
                        drop(schema);
                        let mut schema = self.schema.write();
                        if !schema.component_names.contains_key(&tid) {
                            log::debug!("Adding {:?} to ECS schema", std::any::type_name::<T>());
                            schema.component_names.insert(tid, std::any::type_name::<T>());
                        }
                    }
                }
                let mut map = EcHashMap::new(
                    std::alloc::Layout::new::<T>(),
                    if std::mem::needs_drop::<T>() {
                        Some(|ptr| unsafe {
                            let ptr = std::mem::transmute::<*mut u8, *mut T>(ptr);
                            std::ptr::drop_in_place(ptr);
                        })
                    } else { None },
                    |dst, src| unsafe {
                        let dst = std::mem::transmute::<*mut u8, *mut T>(dst);
                        let src = std::mem::transmute::<*const u8, &T>(src);
                        dst.write(src.clone())
                    }
                );
                map.get_or_insert_with(eid, initializer);
                ent.insert(RwLock::new(map));
            },
        }
    }
    /// Mark the given entity for deletion. It won't actually be deleted until
    /// the end of a tick. Panics if the entity doesn't exist, but there's
    /// no problem if you mark the same entity for deletion more than once in
    /// the same tick. Panics if there isn't a tick happening.
    /// 
    /// If random entity IDs are being used, this entity ID will be ineligible
    /// from reuse for several more ticks, to give references to this ID time
    /// to notice its deletion and handle it somehow.
    pub fn mark_for_deletion(&self, eid: EntityId) {
        if !self.is_ticking {
            panic!("Atttempted to delete an entity outside of a tick.\n\
                    `mark_for_deletion` may only be called inside a call to \
                    `EcsWorld::unbuffered_tick` or \
                    Arcow<EcsWorld>::buffered_tick`.")
        }
        if !self.entities.contains(&eid) {
            #[cfg(feature="randomize-entity-ids")]
            if self.entity_ids_in_limbo.read().contains_key(&eid) {
                panic!("Attempted to mark a nonexistent entity for deletion.\n\
                        Note: this entity recently existed!")
            }
            panic!("Attempted to mark a nonexistent entity for deletion.")
        }
        self.entities_to_delete.write().insert(eid);
    }
    /// Perform a single-buffered tick. This world will be modified in place,
    /// advancing in time by one tick.
    /// 
    /// It is strongly recommended to use `Arcow<EcsWorld>` instead of doing
    /// single-buffered ticks on `EcsWorld`s directly, even if you only use
    /// single-buffered ticks.
    /// 
    /// During a single-buffered tick, `cur` and `mut` component iteration is
    /// possible, but not `prev`. You may get improved performance and
    /// semantics by using `prev` wherever you can live with one tick of
    /// latency.
    pub fn unbuffered_tick<F: FnOnce(&mut EcsWorld)>(&mut self, handler: F) {
        if self.is_ticking {
            panic!("Cannot perform unbuffered_tick() on an EcsWorld that is \
                currently being ticked.")
        }
        // if `is_ticking` is false, `origin` should be `None`, there should be
        // no way to use `with_origin` to screw with that
        debug_assert!(self.origin.is_none());
        self.is_ticking = true;
        handler(self);
        self.is_ticking = false;
        self.post_tick();
        // Contract the `EcHashMap` if possible. We have to do this "by hand"
        // if we're not doing double buffering. (Double buffering handles this
        // transparently in `EcHashMap::clone`.)
        for (_, component) in self.components.iter_mut() {
            debug_assert!(!component.is_locked());
            component.get_mut().contract();
        }
    }
    /// Runs a closure that can do `prev` iteration and getting, treating the
    /// given world as the origin. Useful for render code.
    pub fn with_origin<F: FnOnce(&EcsWorld)>(&mut self, origin: Arcow<EcsWorld>, f: F) {
        if self.is_ticking {
            panic!("Cannot perform with_origin() on an Ecsworld that is \
                currently being ticked.")
        }
        let mut origin = Some(origin);
        std::mem::swap(&mut self.origin, &mut origin);
        f(self);
        std::mem::swap(&mut self.origin, &mut origin);
    }
    fn post_tick(&mut self) {
        #[cfg(feature="randomize-entity-ids")]
        let mut entity_ids_in_limbo = self.entity_ids_in_limbo.write();
        #[cfg(feature="randomize-entity-ids")]
        entity_ids_in_limbo.drain_filter(|_, v| {
            if *v == 0 { true }
            else { *v -= 1; false }
        });
        for eid in self.entities_to_delete.write().drain() {
            if self.entities.remove(&eid) {
                for (_, component) in self.components.iter_mut() {
                    debug_assert!(!component.is_locked());
                    component.get_mut().remove(eid);
                }
                #[cfg(feature="randomize-entity-ids")]
                entity_ids_in_limbo.insert(eid, ENTITY_ID_LIMBO_TICKS);
            }
        }
    }
    /// Don't call this directly! Use [ecs_detach!](macro.ecs_detach.html)
    /// instead.
    #[doc(hidden)]
    pub fn detach_components<I: Iterator<Item=EntityId>, const N: usize>(&self, eids: I, types: [TypeId; N]) {
        if !self.is_ticking {
            panic!("Attempted to detach components outside of a tick!\n\
                    `ecs_detach!` may only be used inside a call to \
                    `EcsWorld::unbuffered_tick` or \
                    Arcow<EcsWorld>::buffered_tick`.")
        }
        // Acquire all locks
        let mut locks = types.map(|typ| {
            self.components.get(&typ).map(|x| {
                if cfg!(debug_assertions) && x.is_locked() {
                    panic!("Attempted to detach a component type that is \
                            currently locked!")
                }
                RwLock::write(x)
            })
        });
        // THEN act through them.
        for eid in eids {
            #[cfg(any(debug_assertions,not(feature="gotta-go-fast")))]
            if !self.entity_exists(eid) {
                panic!("Tried to detach component(s) from nonexistent entity {:?}!", eid);
            }
            locks.each_mut().map(|x| {
                if let Some(x) = x { x.remove(eid); }
            });
        }
    }    
}

pub trait EcsWorldBufferedTick {
    fn buffered_tick<'b, F: FnOnce(&mut EcsWorld)>(&'b self, handler: F) -> Arcow<EcsWorld>;
}

impl EcsWorldBufferedTick for Arcow<EcsWorld> {
    /// Perform a double-buffered tick. The existing world will remain
    /// unchanged. A new world will be returned, representing that world one
    /// tick into the future.
    /// 
    /// During a double-buffered tick, all forms of component iteration are
    /// possible, including `prev`. If you don't use `prev` iteration anywhere,
    /// you may get better performance from using `unbuffered_tick` instead.
    fn buffered_tick<'a, 'b, F: FnOnce(&mut EcsWorld)>(&'b self, handler: F) -> Arcow<EcsWorld> {
        if self.is_ticking {
            panic!("Cannot perform buffered_tick() on an EcsWorld that is \
                currently being ticked.")
        }
        let mut clone = (**self).clone();
        clone.origin = Some(self.clone());
        clone.is_ticking = true;
        handler(&mut clone);
        clone.is_ticking = false;
        clone.origin = None;
        clone.post_tick();
        Arcow::new(clone)
    }
}

impl Clone for EcsWorld {
    fn clone(&self) -> EcsWorld {
        // parking_lot::RwLock doesn't implement clone... :(
        EcsWorld {
            schema: self.schema.clone(),
            entities: self.entities.clone(),
            components: self.components.iter().map(|(k,v)| {
                (*k, RwLock::new(v.read().clone()))
            }).collect(),
            origin: None,
            is_ticking: false,
            #[cfg(not(feature="randomize-entity-ids"))]
            next_entity_id: self.next_entity_id,
            entities_to_delete: RwLock::new(self.entities_to_delete.read().clone()),
            #[cfg(feature="randomize-entity-ids")]
            entity_ids_in_limbo: RwLock::new(self.entity_ids_in_limbo.read().clone()),
        }
    }
}

/// Attach components to an entity. Used internally by `ecs_spawn!`. Panics if
/// the same type of component is attached more than once.
#[macro_export]
macro_rules! ecs_attach {
    ($world:expr, $eid:expr, $comp:expr, $($comps:expr),+ $(,)?) => {
        $world.attach_new_component($eid, $comp);
        $crate::ecs_attach!($world, $eid, $($comps),+);
    };
    ($world:expr, $eid:expr, $comp:expr $(,)?) => {
        $world.attach_new_component($eid, $comp);
    };
}

/// Spawn a new entity. TODO document
#[macro_export]
macro_rules! ecs_spawn {
    ($world:expr, $($comps:expr),+ $(,)?) => {{
        let eid = $world.spawn();
        $crate::ecs_attach!($world, eid, $($comps),+);
        eid
    }};
}

/// Deletes all components of a given type from every given entity. It's
/// alright if any (or all) of the components are missing from a given entity.
/// It's not alright if an entity has already been deleted.
/// 
/// Provide entity IDs as an iterator (e.g. `entities_to_delete.into_iter()`).
///
/// Be aware that this mutably locks that component type, so you can't call
/// this during a mutable iteration over that same component type!
#[macro_export]
macro_rules! ecs_detach {
    ($world:expr, $eids:expr, $($comps:ty),+ $(,)?) => {
        $world.detach_components($eids, [$(std::any::TypeId::of::<$comps>()),+])
    };
}

/// Iterates through all entities in a world that have a given set of
/// components. This is the fundamental operation of a System.
/// 
/// ```rust ignore
/// # // This test doesn't compile at the moment. Bug in `proc-macro-crate`
/// # // considered responsible.
/// # use psilo_ecs::{ecs_iter, EcsWorld};
/// # #[derive(Clone)] struct FooComp {}
/// # #[derive(Clone)] struct BarComp {}
/// # let world = EcsWorld::with_blank_schema();
/// for (eid, foo, bar) in ecs_iter!(world, mut FooComp, cur BarComp) {
///     // do something with eid, foo, bar
/// }
/// ```
/// 
/// You can iterate over any number of component types greater than one. (If
/// you have more than a dozen, you should consider breaking up this System
/// into smaller, simpler ones.)
/// 
/// Each component to iterate over is an access type and a component type.
/// 
/// Access types:
/// 
/// - `prev`: Obtain the state at the **end** of the previous tick. This access
///   type is only available within [`buffered_tick`][1] or [`with_origin`][3]
///   context. It does not require any synchronization, and may be the fastest
///   option if mid-tick-up-to-date state is not required.
/// - `cur`: Obtain the current state. This access type is always available. It
///   requires a read lock on the component type, which adds a little bit of
///   fixed overhead and prevents any parallel `mut` iteration over that
///   component type.
/// - `mut`: Obtain a mutable reference to the current state. This access type
///   is only available "during a tick", i.e. in [`buffered_tick`][1] or
///   [`unbuffered_tick`][2] context. It requires a write lock on the component
///   type, which adds a little bit of fixed overhead and prevents any other
///   parallel iteration over that component type.
/// 
/// If a component type is *optional*, you can enclose the type in `Option<>`.
/// Entities that are missing some or all optional components will still be
/// iterated upon. This is especially useful with `prev`, as it allows you to
/// iterate even over entities that were spawned during this tick. The
/// **first** component type **must not** be optional.
/// 
/// You can do multiple `ecs_iter!` over the same `EcsWorld` in parallel
/// safely. `RwLock`s are used to ensure thread safety. *Iterating* in parallel
/// is not supported *yet*. When first-class `System`s are added, they will
/// include dynamic parallelism transparently, both between `System`s and
/// within them. This will be done before 1.0.
///
/// **WARNING:**
/// Due to the locking involved, and safety issues, you are not permitted to
/// have multiple `ecs_iter!` or `ecs_get!` references to the same `mut T`.
/// In debug builds, this will be detected and result in a panic. In release
/// builds, this will deadlock!
/// 
/// [1]: trait.EcsWorldBufferedTick.html#tymethod.buffered_tick
/// [2]: struct.EcsWorld.html#method.unbuffered_tick
/// [3]: struct.EcsWorld.html#method.with_origin
pub use psilo_ecs_procmacros::ecs_iter;

/// Attempt to get the given components for a given entity ID. If it does not
/// exist, or if it lacks some non-optional components, `None` will be
/// returned. Often, but not always, it's better to accomplish the same thing
/// using optional components in `ecs_iter`.
///
/// Syntax is similar to [`ecs_iter!`](macro.ecs_iter.html). There is a second
/// parameter, which is the entity ID to look for, and the entity
/// ID is not returned.
///
/// ```rust ignore
/// # // This test doesn't compile at the moment. Bug in `proc-macro-crate`
/// # // considered responsible.
/// # use psilo_ecs::{ecs_iter, EcsWorld};
/// # #[derive(Clone)] struct FooComp {}
/// # #[derive(Clone)] struct BarComp {}
/// # let world = EcsWorld::with_blank_schema();
/// # let eid = 456;
/// if let Some(foo, bar) = ecs_get!(world, eid, mut FooComp, cur BarComp) {
///     // do something with eid, foo, bar
/// }
/// ```
///
/// Never hardcode entity IDs. Only use entity IDs that were yielded from
/// `ecs_iter!` or returned from [`ecs_spawn!`](macro.ecs_spawn.html).
///
/// **WARNING:**
/// Due to the locking involved, and safety issues, you are not permitted to
/// have multiple `ecs_iter!` or `ecs_get!` references to the same `mut T`.
/// In debug builds, this will be detected and result in a panic. In release
/// builds, this will deadlock!
pub use psilo_ecs_procmacros::ecs_get;

