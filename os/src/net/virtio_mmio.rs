use core::mem::size_of;

use bitflags::*;
use device_tree::Node;
use device_tree::util::SliceRead;
use volatile::{ReadOnly, Volatile, WriteOnly};

bitflags! {
    pub struct VirtIODeviceStatus : u32 {
        const ACKNOWLEDGE = 1;
        const DRIVER = 2;
        const FAILED = 128;
        const FEATURES_OK = 8;
        const DRIVER_OK = 4;
        const DEVICE_NEEDS_RESET = 64;
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct VirtIOVirtqueueDesc {
    pub addr: Volatile<u64>,
    pub len: Volatile<u32>,
    pub flags: Volatile<u16>,
    pub next: Volatile<u16>
}

bitflags! {
    pub struct VirtIOVirtqueueFlag : u16 {
        const NEXT = 1;
        const WRITE = 2;
        const INDIRECT = 4;
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct VirtIOVirtqueueAvailableRing {
    pub flags: Volatile<u16>,
    pub idx: Volatile<u16>,
    pub ring: [Volatile<u16>; 32], // actual size: queue_size
    used_event: Volatile<u16>
}

#[repr(packed)]
#[derive(Debug)]
pub struct VirtIOVirtqueueUsedElem {
    id: Volatile<u32>,
    len: Volatile<u32>
}

#[repr(packed)]
#[derive(Debug)]
pub struct VirtIOVirtqueueUsedRing {
    pub flags: Volatile<u16>,
    pub idx: Volatile<u16>,
    pub ring: [VirtIOVirtqueueUsedElem; 32], // actual size: queue_size
    avail_event: Volatile<u16>
}

// virtio 2.4.2 Legacy Interfaces: A Note on Virtqueue Layout
pub fn virtqueue_size(num: usize, align: usize) -> usize {
    (((size_of::<VirtIOVirtqueueDesc>() * num + size_of::<u16>() * (3 + num)) + align) & !(align-1)) +
        (((size_of::<u16>() * 3 + size_of::<VirtIOVirtqueueUsedElem>() * num) + align) & !(align-1))
}

pub fn virtqueue_used_elem_offset(num: usize, align: usize) -> usize {
    ((size_of::<VirtIOVirtqueueDesc>() * num + size_of::<u16>() * (3 + num)) + align) & !(align-1)
}



