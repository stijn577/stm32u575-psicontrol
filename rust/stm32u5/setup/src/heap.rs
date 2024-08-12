#[global_allocator]
static HEAP: alloc_cortex_m::CortexMHeap = alloc_cortex_m::CortexMHeap::empty();

pub(crate) fn heap_init(size: usize) {
    let start = cortex_m_rt::heap_start() as usize;
    unsafe { HEAP.init(start, size) }
}

pub fn heap_usage() -> (usize, usize) {
    (HEAP.used(), HEAP.free())
}
