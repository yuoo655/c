use core::mem::size_of;

use bitflags::*;
use device_tree::Node;
use device_tree::util::SliceRead;
use volatile::{ReadOnly, Volatile, WriteOnly};

// use crate::memory::active_table;

// use super::net::virtio_net;

// // virtio 4.2.4 Legacy interface
// #[repr(packed)]
// #[derive(Debug)]
// pub struct VirtIOHeader {
//     magic: ReadOnly<u32>, // 0x000
//     version: ReadOnly<u32>, // 0x004
//     device_id: ReadOnly<u32>, // 0x008
//     vendor_id: ReadOnly<u32>, // 0x00c
//     pub device_features: ReadOnly<u32>, // 0x010
//     pub device_features_sel: WriteOnly<u32>, // 0x014
//     __r1: [ReadOnly<u32>; 2], 
//     pub driver_features: WriteOnly<u32>, // 0x020
//     pub driver_features_sel: WriteOnly<u32>, // 0x024
//     pub guest_page_size: WriteOnly<u32>, // 0x028
//     __r2: ReadOnly<u32>,
//     pub queue_sel: WriteOnly<u32>, // 0x030
//     pub queue_num_max: ReadOnly<u32>, // 0x034
//     pub queue_num: WriteOnly<u32>, // 0x038
//     pub queue_align: WriteOnly<u32>, // 0x03c
//     pub queue_pfn: Volatile<u32>, // 0x040
//     queue_ready: Volatile<u32>, // new interface only
//     __r3: [ReadOnly<u32>; 2],
//     pub queue_notify: WriteOnly<u32>, // 0x050
//     __r4: [ReadOnly<u32>; 3],
//     pub interrupt_status: ReadOnly<u32>, // 0x060
//     pub interrupt_ack: WriteOnly<u32>, // 0x064
//     __r5: [ReadOnly<u32>; 2],
//     pub status: Volatile<u32>, // 0x070
//     __r6: [ReadOnly<u32>; 3],
//     queue_desc_low: WriteOnly<u32>, // new interface only since here
//     queue_desc_high: WriteOnly<u32>,
//     __r7: [ReadOnly<u32>; 2],
//     queue_avail_low: WriteOnly<u32>,
//     queue_avail_high: WriteOnly<u32>,
//     __r8: [ReadOnly<u32>; 2],
//     queue_used_low: WriteOnly<u32>,
//     queue_used_high: WriteOnly<u32>,
//     __r9: [ReadOnly<u32>; 21],
//     config_generation: ReadOnly<u32>
// }

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



