use super::{EntityId, EntityHash};

// TODO: do some statistical checks on these

#[cfg(feature="u64-entity-ids")]
const CONSTANT_PRIME: u64 = 0xA4DD1D198BA829AF;
#[cfg(not(feature="u64-entity-ids"))]
const CONSTANT_PRIME: u32 = 0xAF05BB09;

pub fn hash(key: EntityId) -> EntityHash {
    key.wrapping_mul(CONSTANT_PRIME)
}