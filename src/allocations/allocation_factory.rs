use crate::allocations::allocation::{Allocation, Builder as AllocationBuilder};

pub struct AllocationFactory;

impl AllocationFactory {
    pub fn stub() -> Allocation {
        Allocation::Stub
    }
    pub fn in_memory_builder() -> AllocationBuilder {
        AllocationBuilder::new()
    }
}
