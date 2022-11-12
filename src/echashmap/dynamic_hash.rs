use std::{
    intrinsics::{likely, unlikely},
    sync::{atomic, atomic::Ordering},
    thread::yield_now,
};
use super::{EntityId, EntityHash};
use rand::Rng;

const SENTINEL_UNINIT: EntityHash = 0;
const SENTINEL_GENERATING: EntityHash = 1;

#[cfg(test)]
pub(crate) const ECHASHMAP_DYNAMIC_PRIME: &str = "ECHASHMAP_DYNAMIC_PRIME";

#[cfg(feature="u64-entity-ids")]
static DYNAMIC_PRIME: atomic::AtomicU64 = atomic::AtomicU64::new(SENTINEL_UNINIT);
#[cfg(not(feature="u64-entity-ids"))]
static DYNAMIC_PRIME: atomic::AtomicU32 = atomic::AtomicU32::new(SENTINEL_UNINIT);

/// Randomly generate a prime, with half of its bits being ones and half being
/// zeroes.
#[cold]
fn find_good_prime() -> EntityHash {
    let mut rng = rand::thread_rng();
    let num_bits = EntityHash::MAX.count_ones();
    assert!(num_bits % 2 == 0);
    #[cfg(test)]
    if let Ok(candidate) = std::env::var(ECHASHMAP_DYNAMIC_PRIME) {
        match candidate.parse() {
            Ok(x) => {
                if primal::is_prime(x as u64) {
                    eprintln!("Using {}: {}", ECHASHMAP_DYNAMIC_PRIME, x);
                    return x
                }
                else {
                    DYNAMIC_PRIME.store(SENTINEL_UNINIT, Ordering::Relaxed);
                    panic!("{} specified a non-prime", ECHASHMAP_DYNAMIC_PRIME);
                }
            },
            Err(_) => {
                DYNAMIC_PRIME.store(SENTINEL_UNINIT, Ordering::Relaxed);
                panic!("{} specified a non-integer", ECHASHMAP_DYNAMIC_PRIME);
            },
        }
    }
    loop {
        let mut candidate: EntityHash = 1;
        // Uncomment this if it turns out that having a leading 1 bit improves
        // the statistical properties.
        //candidate |= 1 << num_bits - 1;
        for _ in candidate.count_ones() .. num_bits / 2 {
            loop {
                let bit = rng.gen_range(1..num_bits);
                if candidate & (1 << bit) == 0 {
                    candidate |= 1 << bit;
                    break;
                }
            }
        }
        assert_eq!(candidate.count_ones(), num_bits / 2);
        if primal::is_prime(candidate as u64) {
            #[cfg(test)]
            eprintln!("Using generated prime: {}\n(You can force this same \
                prime to be used on a subsequent test by setting the {} \
                variable in the environment.)", candidate,
                ECHASHMAP_DYNAMIC_PRIME);
            break candidate
        }
    }
}

#[cold] #[inline(never)]
pub fn slow_path(key: EntityId) -> EntityHash {
    let mut stored = DYNAMIC_PRIME.load(Ordering::Relaxed);
    if unlikely(stored != SENTINEL_UNINIT && stored != SENTINEL_GENERATING) {
        return key.wrapping_mul(stored)
    }
    loop {
        while stored == SENTINEL_UNINIT {
            match DYNAMIC_PRIME.compare_exchange_weak(SENTINEL_UNINIT, SENTINEL_GENERATING, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(x) => assert!(x == SENTINEL_UNINIT),
                Err(x) => {
                    stored = x;
                    continue
                }
            }
            // This thread is in charge of determining the prime.
            let prime = find_good_prime();
            // TODO: replace with compare_exchange?
            DYNAMIC_PRIME.store(prime, Ordering::Relaxed);
            return key.wrapping_mul(prime)
        }
        assert_ne!(stored, SENTINEL_UNINIT);
        if stored == SENTINEL_GENERATING {
            while stored == SENTINEL_GENERATING {
                yield_now();
                stored = DYNAMIC_PRIME.load(Ordering::Relaxed);
            }
        }
        assert_ne!(stored, SENTINEL_UNINIT);
        assert_ne!(stored, SENTINEL_GENERATING);
        return key.wrapping_mul(stored)
    }
}

#[inline(always)]
pub fn hash(key: EntityId) -> EntityHash {
    let stored = DYNAMIC_PRIME.load(Ordering::Relaxed);
    if likely(stored != SENTINEL_UNINIT && stored != SENTINEL_GENERATING) {
        return key.wrapping_mul(stored)
    }
    slow_path(key)
}

#[cfg(test)]
pub(crate) unsafe fn force_rehash() {
    let stored = DYNAMIC_PRIME.load(Ordering::Relaxed);
    if stored == SENTINEL_GENERATING {
        panic!("force_rehash() was called while a thread was generating a hash!");
    }
    assert!(DYNAMIC_PRIME.compare_exchange(stored, SENTINEL_UNINIT, Ordering::Relaxed, Ordering::Relaxed).is_ok())
}