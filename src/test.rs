use super::*;

#[derive(Clone, Debug, PartialEq)]
struct TestCompA(i32);
#[derive(Clone, Debug, PartialEq)]
struct TestCompB {
    alpha: i32,
    beta: i32,
}

#[test] fn component_iter() {
    let mut world = EcsWorld::with_blank_schema();
    let nu = ecs_spawn!(world, TestCompA(456));
    let ou = ecs_spawn!(world, TestCompB { alpha: 0, beta: 123 });
    let pu = ecs_spawn!(world, TestCompA(101112), TestCompB { alpha: 65535, beta: 789 });
    println!("{} {} {}", nu, ou, pu);
    for (eid, a) in ecs_iter!(world, cur TestCompA) {
        println!("Just the As! {}, {:?}", eid, a);
    }
    for (eid, a, b) in ecs_iter!(world, cur TestCompA, cur TestCompB) {
        println!("Double-iterated components! {}, {:?}, {:?}", eid, a, b);
    }
    assert_eq!(ecs_iter!(world, cur TestCompA).fold(0, |a,_| a+1), 2);
    assert_eq!(ecs_iter!(world, cur TestCompA, cur TestCompB).fold(0, |a,_| a+1), 1);
}

#[test] fn component_optional_iter() {
    let mut world = EcsWorld::with_blank_schema();
    let nu = ecs_spawn!(world, TestCompA(456));
    let ou = ecs_spawn!(world, TestCompB { alpha: 0, beta: 123 });
    let pu = ecs_spawn!(world, TestCompA(101112), TestCompB { alpha: 65535, beta: 789 });
    println!("{} {} {}", nu, ou, pu);
    for (eid, a, b) in ecs_iter!(world, cur TestCompA, cur_optional TestCompB) {
        println!("Option option option! {}, {:?}, {:?}", eid, a, b);
    }
    assert_eq!(ecs_iter!(world, cur TestCompA, cur_optional TestCompB).fold(0, |a,_| a+1), 2);
}

#[test] #[should_panic] fn component_no_leading_optional() {
    let mut world = EcsWorld::with_blank_schema();
    let nu = ecs_spawn!(world, TestCompA(456));
    let ou = ecs_spawn!(world, TestCompB { alpha: 0, beta: 123 });
    let pu = ecs_spawn!(world, TestCompA(101112), TestCompB { alpha: 65535, beta: 789 });
    println!("{} {} {}", nu, ou, pu);
    for (eid, a, b) in ecs_iter!(world, cur_optional TestCompA, cur_optional TestCompB) {
        println!("Option option option! {}, {:?}, {:?}", eid, a, b);
    }
}

#[test] fn zst() {
    let mut world = EcsWorld::with_blank_schema();
    println!("{}", ecs_spawn!(world, ()));
    println!("{}", ecs_spawn!(world, (),));
    println!("{}", ecs_spawn!(world, ()));
    assert_eq!(ecs_iter!(world, cur ()).fold(0, |a,_| a+1), 3);
}

#[test] #[should_panic] fn component_double_attach() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_spawn!(world, TestCompA(456), TestCompA(123));
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn component_ghost_attach() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_attach!(world, 1, TestCompA(456));
}

#[test] #[should_panic] fn no_tick_i() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_spawn!(world, TestCompA(456));
    for _ in ecs_iter!(world, mut TestCompA) {}
}

#[test] #[should_panic] fn no_tick_origin() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_spawn!(world, TestCompA(456));
    for _ in ecs_iter!(world, prev TestCompA) {}
}

#[test] fn yes_tick_i() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_spawn!(world, TestCompA(456));
    world.unbuffered_tick(|world| {
        for _ in ecs_iter!(world, mut TestCompA) {}
    })
}

#[test] #[should_panic] fn no_yes_tick_origin() {
    let mut world = EcsWorld::with_blank_schema();
    ecs_spawn!(world, TestCompA(456));
    world.unbuffered_tick(|world| {
        for _ in ecs_iter!(world, prev TestCompA) {}
    })
}

#[test] fn yes_tick_origin() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        for _ in ecs_iter!(world, prev TestCompA) {}
        for _ in ecs_iter!(world, mut TestCompA) {}
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_double_mut() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        for _ in ecs_iter!(world, mut TestCompA) {
            for _ in ecs_iter!(world, mut TestCompA) {
            }
        }
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_double_iter_mut() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        for _ in ecs_iter!(world, mut TestCompA) {
            let _ = ecs_get!(world, 1, mut TestCompA);
        }
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_double_mut_iter() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    let eid = ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        let keep = ecs_get!(world, eid, mut TestCompA);
        for _ in ecs_iter!(world, mut TestCompA) {
        }
        let _ = keep.unwrap();
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_mut_immut() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        for _ in ecs_iter!(world, cur TestCompA) {
            for _ in ecs_iter!(world, mut TestCompA) {
            }
        }
    });
}

#[cfg(debug_assertions)] #[test] fn yes_mut_prev() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    world.buffered_tick(|world| {
        for _ in ecs_iter!(world, prev TestCompA) {
            for _ in ecs_iter!(world, mut TestCompA) {
            }
        }
    });
}

#[cfg(debug_assertions)] #[test] fn yes_origin_prev() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    let old_world = world.clone();
    world.with_origin(old_world, |world| {
        for _ in ecs_iter!(world, prev TestCompA) {
        }
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_origin_mut() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    let old_world = world.clone();
    world.with_origin(old_world, |world| {
        for _ in ecs_iter!(world, mut TestCompA) {
        }
    });
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn no_tick_no_delete() {
    let mut world = EcsWorld::with_blank_schema();
    let eid = ecs_spawn!(world, TestCompA(456));
    world.mark_for_deletion(eid)
}

#[cfg(debug_assertions)] #[test] fn yes_tick_yes_delete() {
    let mut world = EcsWorld::with_blank_schema();
    let eid = ecs_spawn!(world, TestCompA(456));
    world.unbuffered_tick(|world| {
        world.mark_for_deletion(eid)
    })
}

#[cfg(debug_assertions)] #[test] fn deletion_works() {
    let mut world = EcsWorld::with_blank_schema();
    let eid = ecs_spawn!(world, TestCompA(456));
    assert!(world.entity_exists(eid));
    world.unbuffered_tick(|world| {
        world.mark_for_deletion(eid);
        world.mark_for_deletion(eid);
        world.mark_for_deletion(eid);
        world.mark_for_deletion(eid);
        world.mark_for_deletion(eid);
        assert!(world.entity_exists(eid));
    });
    assert!(!world.entity_exists(eid));
}

#[test] fn get_entity() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    let target = ecs_spawn!(world, TestCompA(456), TestCompB { alpha: 444, beta: 777 });
    world.buffered_tick(|world| {
        let (a, b) = ecs_get!(world, target, cur TestCompA, cur TestCompB).unwrap();
        assert_eq!(*a, TestCompA(456));
        assert_eq!(*b, TestCompB { alpha: 444, beta: 777 });
    });
}

#[test] fn detach() {
    let mut world = Arcow::new(EcsWorld::with_blank_schema());
    ecs_spawn!(world, TestCompA(456));
    let target = ecs_spawn!(world, TestCompA(456), TestCompB { alpha: 444, beta: 777 });
    world.buffered_tick(|world| {
        let (a, b) = ecs_get!(world, target, cur TestCompA, cur TestCompB).unwrap();
        assert_eq!(*a, TestCompA(456));
        assert_eq!(*b, TestCompB { alpha: 444, beta: 777 });
        drop(a);
        drop(b);
        ecs_detach!(world, std::iter::once(target), TestCompB);
        assert!(ecs_get!(world, target, cur TestCompA, cur TestCompB).is_none());
        assert!(ecs_get!(world, target, cur TestCompB).is_none());
        assert!(ecs_get!(world, target, cur TestCompA).is_some());
    });
}

#[test] fn optional_no_miss() {
    let mut world = EcsWorld::with_blank_schema();
    for n in 0 .. 1024 {
        let blah = ecs_spawn!(world, TestCompA(456));
        if n % 16 == 7 {
            ecs_attach!(world, blah, TestCompB { alpha: 49151, beta: 1 });
        }
    }
    world.unbuffered_tick(|world| {
        for (_, a, b) in ecs_iter!(world, mut TestCompA, mut_optional TestCompB) {
            if let Some(b) = b {
                a.0 -= 456;
                b.alpha = 3; // pi
            }
            else {
                a.0 -= 123;
            }
        }
    });
    for (eid, _a, b) in ecs_iter!(world, cur TestCompA, cur_optional TestCompB) {
        let (_alt_a, alt_b) = ecs_get!(world, eid, cur TestCompA, cur_optional TestCompB).unwrap();
        assert_eq!(b.is_none(), alt_b.is_none());
    }
    let mut wrong = 0;
    for (eid, a) in ecs_iter!(world, cur TestCompA) {
        match ecs_get!(world, eid, cur TestCompB) {
            None => {
                if a.0 == 333 {
                    // OK
                }
                else {
                    eprintln!("{}のa should be 333, is {}", eid, a.0);
                    wrong += 1;
                }
            },
            Some(b) => {
                if a.0 == 0 {
                    // OK
                }
                else {
                    eprintln!("{}のa should be 0, is {}", eid, a.0);
                    wrong += 1;
                }
                if b.alpha != 3 {
                    eprintln!("{}のb.alpha should be 3, is {}", eid, b.alpha);
                    wrong += 1;
                }
            },
        }
    }
    if wrong > 0 {
        panic!("{} wrongs!", wrong);
    }
}