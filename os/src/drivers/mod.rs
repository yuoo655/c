mod block;
pub use block::BLOCK_DEVICE;





//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------  

use alloc::string::String;
use alloc::vec::Vec;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};


use spin::RwLock;
use smoltcp::socket::*;
use alloc::vec;
use core::fmt::Write;
// use crate::net::SOCKETS;
use smoltcp::wire::*;
use lazy_static::*;
use alloc::sync::Arc;

use spin::Mutex;

use alloc::format;

use smoltcp::phy::{self, DeviceCapabilities};
use smoltcp::time::Instant;
use smoltcp::Result;
use smoltcp::iface::*;
use virtio_drivers::{VirtIOHeader, VirtIONet};



use alloc::{collections::BTreeMap};


use crate::config::PAGE_SIZE;


// //----------------------------------------------------------------------------------------------------------------------------------------------------------------------------  


// #[derive(Clone)]
// pub struct VirtIONetDriver(Arc<Mutex<VirtIONet<'static>>>);


// //----------------------------------------------------------------------------------------------------------------------------------------------------------------------------  



// #[derive(Debug, Eq, PartialEq)]
// pub enum DeviceType {
//     Net,
//     Gpu,
//     Input,
//     Block,
//     Rtc,
//     Serial,
//     Intc,
// }


// // pub trait Driver: Send + Sync {
// //     // if interrupt belongs to this driver, handle it and return true
// //     // return false otherwise
// //     // irq number is provided when available
// //     // driver should skip handling when irq number is mismatched
// //     fn try_handle_interrupt(&self, irq: Option<usize>) -> bool;

// //     // return the correspondent device type, see DeviceType
// //     fn device_type(&self) -> DeviceType;

// //     // get unique identifier for this device
// //     // should be different for each instance
// //     fn get_id(&self) -> String;

// //     // trait casting
// //     fn as_net(&self) -> Option<&dyn NetDriver> {
// //         None
// //     }

// //     // fn as_block(&self) -> Option<&dyn BlockDriver> {
// //     //     None
// //     // }

// //     // fn as_rtc(&self) -> Option<&dyn RtcDriver> {
// //     //     None
// //     // }
// // }

// pub trait NetDriver: Driver {
//     // get mac address for this device
//     fn get_mac(&self) -> EthernetAddress {
//         unimplemented!("not a net driver")
//     }

//     // get interface name for this device
//     fn get_ifname(&self) -> String {
//         unimplemented!("not a net driver")
//     }

//     // get ip addresses
//     fn get_ip_addresses(&self) -> Vec<IpCidr> {
//         unimplemented!("not a net driver")
//     }

//     // get ipv4 address
//     fn ipv4_address(&self) -> Option<Ipv4Address> {
//         unimplemented!("not a net driver")
//     }

//     // manually trigger a poll, use it after sending packets
//     fn poll(&self) {
//         unimplemented!("not a net driver")
//     }

//     // send an ethernet frame, only use it when necessary
//     fn send(&self, _data: &[u8]) -> Option<usize> {
//         unimplemented!("not a net driver")
//     }

//     // get mac address from ip address in arp table
//     fn get_arp(&self, _ip: IpAddress) -> Option<EthernetAddress> {
//         unimplemented!("not a net driver")
//     }
// }







// impl NetDriver for VirtIONetDriver {
//     fn get_mac(&self) -> EthernetAddress {
//         EthernetAddress(self.0.lock().mac())
//     }

//     fn get_ifname(&self) -> String {
//         format!("virtio{:?}", self.0.lock().mac())
//     }

//     fn ipv4_address(&self) -> Option<Ipv4Address> {
//         unimplemented!()
//     }

//     fn poll(&self) {
//         unimplemented!()
//     }
// }


// impl Driver for VirtIONetDriver {
//     fn try_handle_interrupt(&self, _irq: Option<usize>) -> bool {
//         self.0.lock().ack_interrupt()
//     }

//     fn device_type(&self) -> DeviceType {
//         DeviceType::Net
//     }

//     fn get_id(&self) -> String {
//         format!("virtio_net")
//     }

//     fn as_net(&self) -> Option<&dyn NetDriver> {
//         Some(self)
//     }

//     // fn as_block(&self) -> Option<&dyn BlockDriver> {
//     //     None
//     // }
// }




// lazy_static! {
//     /// Global SocketSet in smoltcp.
//     ///
//     /// Because smoltcp is a single thread network stack,
//     /// every socket operation needs to lock this.
//     pub static ref SOCKETS: Mutex<SocketSet<'static, 'static, 'static>> =
//         Mutex::new(SocketSet::new(vec![]));
// }


// lazy_static! {
//     // NOTE: RwLock only write when initializing drivers
//     pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
//     pub static ref NET_DRIVERS: RwLock<Vec<Arc<dyn NetDriver>>> = RwLock::new(Vec::new());
//     // pub static ref RTC_DRIVERS: RwLock<Vec<Arc<dyn RtcDriver>>> = RwLock::new(Vec::new());
//     // pub static ref SERIAL_DRIVERS: RwLock<Vec<Arc<dyn SerialDriver>>> = RwLock::new(Vec::new());
//     // pub static ref IRQ_MANAGER: RwLock<irq::IrqManager> = RwLock::new(irq::IrqManager::new(true));
// }


// pub extern "C" fn server(_arg: usize) {
//     println!("server");

//     if NET_DRIVERS.read().len() < 1 {
//         loop {
//             // println!("net drivers read len: {:?}", NET_DRIVERS.read().len());

//             //thread::yield_now();
//         }
//     }else{
//         let mac_addr = NET_DRIVERS.read().get(0).unwrap().get_mac();
//         info!("mac addr: {:?}", mac_addr);

//     }


//     // println!("net drivers read len: {:?}", NET_DRIVERS.read().len());

//     let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 64]);
//     let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 128]);
//     let udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);

//     let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
//     let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
//     let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

//     let tcp2_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
//     let tcp2_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
//     let tcp2_socket = TcpSocket::new(tcp2_rx_buffer, tcp2_tx_buffer);

//     let mut sockets = SOCKETS.lock();
//     let udp_handle = sockets.add(udp_socket);
//     let tcp_handle = sockets.add(tcp_socket);
//     let tcp2_handle = sockets.add(tcp2_socket);
//     drop(sockets);

//     loop {

//         {
//             let mut sockets = SOCKETS.lock();

//             // udp server
//             {
//                 let mut socket = sockets.get::<UdpSocket>(udp_handle);
//                 if !socket.is_open() {
//                     socket.bind(6969).unwrap();
//                 }

//                 let client = match socket.recv() {
//                     Ok((data, endpoint)) => {
//                         Some(endpoint)
//                     }
//                     Err(_) => None
//                 };
//                 if let Some(endpoint) = client {
//                     let hello = b"hello\n";
//                     socket.send_slice(hello, endpoint).unwrap();
//                 }
//             }

//             // simple http server
//             {
//                 let mut socket = sockets.get::<TcpSocket>(tcp_handle);
//                 if !socket.is_open() {
//                     socket.listen(80).unwrap();
//                 }

//                 if socket.can_send() {
//                     write!(socket, "HTTP/1.1 200 OK\r\nServer: rCore\r\nContent-Length: 13\r\nContent-Type: text/html\r\nConnection: Closed\r\n\r\nHello, world!\r\n").unwrap();
//                     socket.close();
//                 }
//             }
//         }

//         // thread::yield_now();
//     }
// }














// impl phy::Device<'_> for VirtIONetDriver {
//     type RxToken = VirtIONetDriver;
//     type TxToken = VirtIONetDriver;

//     fn receive(&mut self) -> Option<(Self::RxToken, Self::TxToken)> {
//         let net = self.0.lock();
//         if net.can_recv() {
//             Some((self.clone(), self.clone()))
//         } else {
//             None
//         }
//     }

//     fn transmit(&mut self) -> Option<Self::TxToken> {
//         let net = self.0.lock();
//         if net.can_send() {
//             Some(self.clone())
//         } else {
//             None
//         }
//     }

//     fn capabilities(&self) -> DeviceCapabilities {
//         let mut caps = DeviceCapabilities::default();
//         caps.max_transmission_unit = 1536;
//         caps.max_burst_size = Some(1);
//         caps
//     }
// }

// impl phy::RxToken for VirtIONetDriver {
//     fn consume<R, F>(self, _timestamp: Instant, f: F) -> Result<R>
//     where
//         F: FnOnce(&mut [u8]) -> Result<R>,
//     {
//         let mut buffer = [0u8; 2000];
//         let mut driver = self.0.lock();
//         let len = driver.recv(&mut buffer).expect("failed to recv packet");
//         f(&mut buffer[..len])
//     }
// }

// impl phy::TxToken for VirtIONetDriver {
//     fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
//     where
//         F: FnOnce(&mut [u8]) -> Result<R>,
//     {
//         let mut buffer = [0u8; 2000];
//         let result = f(&mut buffer[..len]);
//         let mut driver = self.0.lock();
//         driver.send(&buffer).expect("failed to send packet");
//         result
//     }
// }

// pub fn init(header: &'static mut VirtIOHeader) {
//     let net = VirtIONet::new(header).expect("failed to create net driver");
//     let driver = Arc::new(VirtIONetDriver(Arc::new(Mutex::new(net))));

//     DRIVERS.write().push(driver.clone());
//     // IRQ_MANAGER.write().register_all(driver.clone());
//     NET_DRIVERS.write().push(driver);
// }





// use device_tree::util::SliceRead;
// use device_tree::{DeviceTree, Node};
// use virtio_drivers::*;

// #[no_mangle]
// pub fn init_dt(dtb: usize) {
//     info!("device tree @ {:#x}", dtb);
//     #[repr(C)]
//     struct DtbHeader {
//         be_magic: u32,
//         be_size: u32,
//     }
//     let header = unsafe { &*(dtb as *const DtbHeader) };
//     // let magic = u32::from_be(header.be_magic);
//     // const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;
//     // assert_eq!(magic, DEVICE_TREE_MAGIC);
//     let size = u32::from_be(header.be_size);
//     let dtb_data = unsafe { core::slice::from_raw_parts(dtb as *const u8, size as usize) };
//     let dt = DeviceTree::load(dtb_data).expect("failed to parse device tree");

//     println!("init_dt done!");
//     walk_dt_node(&dt.root);
// }


// fn walk_dt_node(dt: &Node) {
//     if let Ok(compatible) = dt.prop_str("compatible") {
//         if compatible == "virtio,mmio" {
//             virtio_probe(dt);
//         }
//     }
//     for child in dt.children.iter() {
//         walk_dt_node(child);
//     }
// }



// // pub fn test_net(dtb: usize){
// //     init_dt(dtb);
// // }

// pub fn virtio_probe(node: &Node) {
//     // println!("virtio_probe");
//     let reg = match node.prop_raw("reg") {
//         Some(reg) => reg,
//         _ => return,
//     };
//     let paddr = reg.as_slice().read_be_u64(0).unwrap();
//     let vaddr = paddr as usize;
//     let size = reg.as_slice().read_be_u64(8).unwrap();
//     // assuming one page
//     assert_eq!(size as usize, PAGE_SIZE);


//     info!("walk dt pa addr={:#x}  va addr={:#x}, size={:#x}", paddr, vaddr, size);
//     let header = unsafe { &mut *(vaddr as *mut VirtIOHeader) };
//     if !header.verify() {
//         // only support legacy device
//         return;
//     }
//     info!(
//         "Detected virtio device with vendor id: {:#X}",
//         header.vendor_id()
//     );
//     info!("Device tree node {:?}", node);
//     match header.device_type() {
//         virtio_drivers::DeviceType::Network => {
//             debug!("header virtio network device");
//             virtio_net_init(header)
//         }
//         // DeviceType::Block => virtio_blk::init(header),
//         // DeviceType::GPU => virtio_gpu::init(header),
//         // DeviceType::Input => virtio_input::init(header),
//         // DeviceType::Console => virtio_console::init(node, header),
//         t => warn!("Unrecognized virtio device: {:?}", t),
//     }
// }




// pub fn virtio_net_init(header: &'static mut VirtIOHeader) {
//     println!("virtio_net_init");
//     let net = VirtIONet::new(header).expect("failed to create net driver");
//     let driver = Arc::new(VirtIONetDriver(Arc::new(Mutex::new(net))));

//     DRIVERS.write().push(driver.clone());
//     // IRQ_MANAGER.write().register_all(driver.clone());
//     NET_DRIVERS.write().push(driver);
// }



// lazy_static! {
//     /// Compatible lookup
//     pub static ref DEVICE_TREE_REGISTRY: RwLock<BTreeMap<&'static str, fn(&Node)>> =
//         RwLock::new(BTreeMap::new());
//     /// Interrupt controller lookup
//     pub static ref DEVICE_TREE_INTC: RwLock<BTreeMap<u32, Arc<dyn IntcDriver>>> =
//         RwLock::new(BTreeMap::new());
// }
// pub trait IntcDriver: Driver {
//     /// Register interrupt controller local irq
//     fn register_local_irq(&self, irq: usize, driver: Arc<dyn Driver>);
// }


// pub fn virtio_mmio_driver_init() {
//     DEVICE_TREE_REGISTRY
//         .write()
//         .insert("virtio,mmio", virtio_probe);
// }


