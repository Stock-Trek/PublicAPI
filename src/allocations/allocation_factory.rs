use crate::allocations::allocation::{Allocation, InMemoryAllocationBuilder};

pub struct AllocationFactory;

impl AllocationFactory {
    pub fn stub() -> Allocation {
        Allocation::Stub
    }
    pub fn in_memory_builder() -> InMemoryAllocationBuilder {
        InMemoryAllocationBuilder::new()
    }
}
