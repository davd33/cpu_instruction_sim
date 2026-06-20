use std::arch::x86_64::{__cpuid, _rdtsc};

pub fn rdtsc() -> u64 {
    unsafe {
        __cpuid(0);
        _rdtsc()
    }
}

pub struct CyclesStat {
    pub(crate) cycles: u64,
    pub(crate) label: String,
}

impl CyclesStat {
    pub(crate) fn new(label: &str, cycles: u64) -> Self {
        CyclesStat {
            label: label.into(),
            cycles
        }
    }
}