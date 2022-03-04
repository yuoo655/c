qemu-system-riscv64 -M virt -device virtio-net-device,?

virtio-net-device options:
  any_layout=<bool>      - on/off (default: true)
  bootindex=<int32>
  csum=<bool>            - on/off (default: true)
  ctrl_guest_offloads=<bool> - on/off (default: true)
  ctrl_mac_addr=<bool>   - on/off (default: true)
  ctrl_rx=<bool>         - on/off (default: true)
  ctrl_rx_extra=<bool>   - on/off (default: true)
  ctrl_vlan=<bool>       - on/off (default: true)
  ctrl_vq=<bool>         - on/off (default: true)
  duplex=<str>
  event_idx=<bool>       - on/off (default: true)
  failover=<bool>        -  (default: false)
  gso=<bool>             - on/off (default: true)
  guest_announce=<bool>  - on/off (default: true)
  guest_csum=<bool>      - on/off (default: true)
  guest_ecn=<bool>       - on/off (default: true)
  guest_rsc_ext=<bool>   - on/off (default: false)
  guest_tso4=<bool>      - on/off (default: true)
  guest_tso6=<bool>      - on/off (default: true)
  guest_ufo=<bool>       - on/off (default: true)
  hash=<bool>            - on/off (default: false)
  host_ecn=<bool>        - on/off (default: true)
  host_mtu=<uint16>      -  (default: 0)
  host_tso4=<bool>       - on/off (default: true)
  host_tso6=<bool>       - on/off (default: true)
  host_ufo=<bool>        - on/off (default: true)
  indirect_desc=<bool>   - on/off (default: true)
  iommu_platform=<bool>  - on/off (default: false)
  mac=<str>              - Ethernet 6-byte MAC Address, example: 52:54:00:12:34:56
  mq=<bool>              - on/off (default: false)
  mrg_rxbuf=<bool>       - on/off (default: true)
  netdev=<str>           - ID of a netdev to use as a backend
  notify_on_empty=<bool> - on/off (default: true)
  packed=<bool>          - on/off (default: false)
  rsc_interval=<uint32>  -  (default: 300000)
  rss=<bool>             - on/off (default: false)
  rx_queue_size=<uint16> -  (default: 256)
  speed=<int32>          -  (default: -1)
  status=<bool>          - on/off (default: true)
  tx=<str>
  tx_queue_size=<uint16> -  (default: 256)
  use-disabled-flag=<bool> -  (default: true)
  use-started=<bool>     -  (default: true)
  x-disable-legacy-check=<bool> -  (default: false)
  x-mtu-bypass-backend=<bool> -  (default: true)
  x-txburst=<int32>      -  (default: 256)
  x-txtimer=<uint32>     -  (default: 150000)





rust_main

memory::init(dtb);

crate::drivers::init(dtb);


pub fn init(dtb: usize) {
    device_tree::init(dtb);
}


use core::slice;

use device_tree::{DeviceTree, Node};

use super::bus::virtio_mmio::virtio_probe;

const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;

fn walk_dt_node(dt: &Node) {
    if let Ok(compatible) = dt.prop_str("compatible") {
        // TODO: query this from table
        if compatible == "virtio,mmio" {
            virtio_probe(dt);
        }
    }
    for child in dt.children.iter() {
        walk_dt_node(child);
    }
}

struct DtbHeader {
    magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe {&*(dtb as*const DtbHeader)};
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        let dtb_data = unsafe { slice::from_raw_parts(dtb as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(dtb_data) {
            walk_dt_node(&dt.root);
        }
    }
}







```rust
// virtio_mmio.rs
use core::mem::size_of;

use bitflags::*;
use device_tree::Node;
use device_tree::util::SliceRead;
use log::*;
use rcore_memory::paging::PageTable;
use volatile::{ReadOnly, Volatile, WriteOnly};

use crate::memory::active_table;

use super::super::gpu::virtio_gpu;
use super::super::net::virtio_net;

// virtio 4.2.4 Legacy interface
#[repr(packed)]
#[derive(Debug)]
pub struct VirtIOHeader {
    magic: ReadOnly<u32>, // 0x000
    version: ReadOnly<u32>, // 0x004
    device_id: ReadOnly<u32>, // 0x008
    vendor_id: ReadOnly<u32>, // 0x00c
    pub device_features: ReadOnly<u32>, // 0x010
    pub device_features_sel: WriteOnly<u32>, // 0x014
    __r1: [ReadOnly<u32>; 2], 
    pub driver_features: WriteOnly<u32>, // 0x020
    pub driver_features_sel: WriteOnly<u32>, // 0x024
    pub guest_page_size: WriteOnly<u32>, // 0x028
    __r2: ReadOnly<u32>,
    pub queue_sel: WriteOnly<u32>, // 0x030
    pub queue_num_max: ReadOnly<u32>, // 0x034
    pub queue_num: WriteOnly<u32>, // 0x038
    pub queue_align: WriteOnly<u32>, // 0x03c
    pub queue_pfn: Volatile<u32>, // 0x040
    queue_ready: Volatile<u32>, // new interface only
    __r3: [ReadOnly<u32>; 2],
    pub queue_notify: WriteOnly<u32>, // 0x050
    __r4: [ReadOnly<u32>; 3],
    pub interrupt_status: ReadOnly<u32>, // 0x060
    pub interrupt_ack: WriteOnly<u32>, // 0x064
    __r5: [ReadOnly<u32>; 2],
    pub status: Volatile<u32>, // 0x070
    __r6: [ReadOnly<u32>; 3],
    queue_desc_low: WriteOnly<u32>, // new interface only since here
    queue_desc_high: WriteOnly<u32>,
    __r7: [ReadOnly<u32>; 2],
    queue_avail_low: WriteOnly<u32>,
    queue_avail_high: WriteOnly<u32>,
    __r8: [ReadOnly<u32>; 2],
    queue_used_low: WriteOnly<u32>,
    queue_used_high: WriteOnly<u32>,
    __r9: [ReadOnly<u32>; 21],
    config_generation: ReadOnly<u32>
}

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

pub fn virtio_probe(node: &Node) {
    if let Some(reg) = node.prop_raw("reg") {
        let from = reg.as_slice().read_be_u64(0).unwrap();
        let size = reg.as_slice().read_be_u64(8).unwrap();
        // assuming one page
        active_table().map(from as usize, from as usize);
        let mut header = unsafe { &mut *(from as *mut VirtIOHeader) };
        let magic = header.magic.read();
        let version = header.version.read();
        let device_id = header.device_id.read();
        // only support legacy device
        if magic == 0x74726976 && version == 1 && device_id != 0 { // "virt" magic
            info!("Detected virtio net device with vendor id {:#X}", header.vendor_id.read());
            info!("Device tree node {:?}", node);
            // virtio 3.1.1 Device Initialization
            header.status.write(0);
            header.status.write(VirtIODeviceStatus::ACKNOWLEDGE.bits());
            if device_id == 1 { // net device
                virtio_net::virtio_net_init(node);
            } else if device_id == 16 { // gpu device
                virtio_gpu::virtio_gpu_init(node);
            } else {
                println!("Unrecognized virtio device {}", device_id);
            }
        } else {
            active_table().unmap(from as usize);
        }
    }
}
```




Read and parse dtb upon boot

Implement virtio net device detection

Negotiate feature bits for virtio-net

Read MAC address from virtio-net device and detect virtqueues

Receiving from virtio net device is working for the first time

Implement driver interface and interrupt handling routines

Rearrange drivers into upper directory

Implement initial support for processing arp request and reply packet

Refactor MAC address and IPv4 address into structs, and implement ICMP echo reply

Fix typos

Implement initial support for virtio gpu driver

Complete first working version of virtio gpu driver with mandelbrot example

Use smoltcp and implement a udp and tcp server on top of it

Cleanup virtio net code


```rust
# 0  <os::net::virtio_net::VirtIONetTxToken as smoltcp::phy::TxToken>::consume (self=..., _timestamp=..., len=42, f=...) at src\net/virtio_net.rs:407
# 1  0x000000008021ba90 in smoltcp::iface::ethernet::InterfaceInner::dispatch_ethernet (self=0x802b8190, tx_token=..., timestamp=..., buffer_len=28, f=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:1519
# 2  0x000000008021d0c8 in smoltcp::iface::ethernet::InterfaceInner::dispatch (self=0x802b8190, tx_token=..., timestamp=..., packet=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:1439
# 3  0x000000008021eadc in smoltcp::iface::ethernet::Interface<DeviceT>::socket_ingress::{{closure}}::{{closure}} (response=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:573
# 4  0x00000000802232ba in core::result::Result<T,E>::and_then (self=..., op=...) at /rustc/6d820866a27b1949e237be79b9c8c0145fe728b7/library/core/src/result.rs:704
# 5  0x000000008021ea74 in smoltcp::iface::ethernet::Interface<DeviceT>::socket_ingress::{{closure}} (frame=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:566
# 6  0x0000000080208dac in <os::net::virtio_net::VirtIONetRxToken as smoltcp::phy::RxToken>::consume (self=..., timestamp=..., f=...) at src\net/virtio_net.rs:385
# 7  0x000000008021e996 in smoltcp::iface::ethernet::Interface<DeviceT>::socket_ingress (self=0x802b8188, sockets=0x802b8a00, timestamp=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:565
# 8  0x000000008021ee64 in smoltcp::iface::ethernet::Interface<DeviceT>::poll (self=0x802b8188, sockets=0x802b8a00, timestamp=...) at C:\Users\l\.cargo\git\checkouts\smoltcp-40eb31ddad590270\5bd87c7\src\iface/ethernet.rs:502
# 9  0x00000000802035f8 in os::net::server (_arg=1) at src\net/mod.rs:297
# 10 0x0000000080202984 in os::net::net_test (dtb=2279604224) at src\net/mod.rs:215
# 11 0x0000000080231872 in os::rust_main (hart_id=0, device_tree_paddr=2279604224) at src\main.rs:76
# 12 0x000000008020001a in stext ()
```