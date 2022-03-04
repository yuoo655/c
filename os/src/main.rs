#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
// #![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]
#![allow(unused)]

use alloc::alloc::dealloc;
// use std::println;
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod config;
mod task;
mod timer;
mod mm;
mod fs;
mod drivers;
mod loader;
mod lkm;
mod scheduler;
mod sync;
mod net;



global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


use core::sync::atomic::{AtomicBool, Ordering};
use core::hint::{spin_loop, self};


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub fn rust_main(hart_id: usize, device_tree_paddr: usize) -> ! {
    
    if hart_id == 0{
        clear_bss();
        mm::init();
        println!("[kernel] Hello, world!");
        println!("[kernel] device tree @ {:#x}", device_tree_paddr);
        
        
        mm::remap_test();
        trap::init();
        trap::enable_timer_interrupt();
        timer::set_next_trigger();
        
        
        net::net_test(device_tree_paddr);

        // info!("loader list app");
        // fs::list_apps();
        // debug!("trying to add user test");
        // // test_for_kernel(0);
        // task::add_initproc();
        // task::add_user_test();


        send_ipi();
        AP_CAN_INIT.store(true, Ordering::Relaxed);

    }else{
        init_other_cpu();
    }

    println_hart!("Hello", hart_id);
    
    
    println_hart!("run user task", hart_id);
    task::run_tasks();
    
    panic!("Unreachable in rust_main!");
}

pub fn init_other_cpu(){

    let hart_id = hart_id();

    if hart_id != 0 {

        while !AP_CAN_INIT.load(Ordering::Relaxed) {
            hint::spin_loop();
        }

        others_main();
        
        unsafe {
            let satp: usize;
            let sp: usize;
            asm!("csrr {}, satp", out(reg) satp);
            println_hart!("init done satp: {:#x}", hart_id, satp);
        }
    }
}

pub fn others_main(){
    mm::init_kernel_space();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
}






pub fn send_ipi(){
    let hart_id = hart_id();
    for i in 1..4 {
        debug!("[hart {}] Start {}", hart_id, i);
        let mask: usize = 1 << i;
        sbi::send_ipi(&mask as *const _ as usize);
    }
}


pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

pub fn test_for_kernel(base: usize){
    let init_environment_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_environment");
    println!("init_environment at {:#x?}", init_environment_addr);
    

    let init_cpu_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_cpu_test");
    println!("init_cpu at {:#x?}", init_cpu_addr);

    let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
    println!("cpu_run at {:#x?}", cpu_run_addr);

    
    let add_user_task_with_priority_addr = lkm::get_symbol_addr_from_elf("basic_rt", "add_user_task_with_priority");
    println!("add_user_task at {:#x?}", add_user_task_with_priority_addr);
    
    use spin::Mutex;
    use woke::waker_ref;
    use core::future::Future;
    use core::pin::Pin;
    use alloc::boxed::Box;


    unsafe{
        
        let init_environment: fn() = core::mem::transmute(init_environment_addr as usize + base);
        
        let init_cpu: fn()= core::mem::transmute(init_cpu_addr as usize + base);
        
        // let add_user_task: fn() = core::mem::transmute(add_user_task_addr as usize + 0x87);
        let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize + base);



        println!("init_environment");
        init_environment();
        
        
        println!("init_cpu");
        init_cpu();

        async fn test(x: i32) {
            crate::println!("{}", x);
        }
        println!("test task addr :{:#x?}", test as usize);

        println!("add_task");
        let add_task_with_priority : fn(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>> , Option<usize>) -> () = unsafe {
            core::mem::transmute(add_user_task_with_priority_addr as usize + base)
        };

        add_task_with_priority(Box::pin(test(666)), Some(0));


        let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
        unsafe{
            let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize);
            println!("cpu_run");
            cpu_run();
        }
    }

}







use device_tree::util::SliceRead;
use device_tree::{DeviceTree, Node};
use virtio_drivers::*;





// fn init_dt(dtb: usize) {
//     info!("device tree @ {:#x}", dtb);
//     #[repr(C)]
//     struct DtbHeader {
//         be_magic: u32,
//         be_size: u32,
//     }
//     let header = unsafe { &*(dtb as *const DtbHeader) };
//     let magic = u32::from_be(header.be_magic);
//     const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;
//     assert_eq!(magic, DEVICE_TREE_MAGIC);
//     let size = u32::from_be(header.be_size);
//     let dtb_data = unsafe { core::slice::from_raw_parts(dtb as *const u8, size as usize) };
//     let dt = DeviceTree::load(dtb_data).expect("failed to parse device tree");
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

// fn virtio_probe(node: &Node) {
//     if let Some(reg) = node.prop_raw("reg") {
//         let paddr = reg.as_slice().read_be_u64(0).unwrap();
//         let size = reg.as_slice().read_be_u64(8).unwrap();
//         let vaddr = paddr;
//         info!("walk dt addr={:#x}, size={:#x}", paddr, size);
//         let header = unsafe { &mut *(vaddr as *mut VirtIOHeader) };
//         info!(
//             "Detected virtio device with vendor id {:#X}",
//             header.vendor_id()
//         );
//         info!("Device tree node {:?}", node);
//         match header.device_type() {
//             // DeviceType::Block => virtio_blk(header),
//             // DeviceType::GPU => virtio_gpu(header),
//             // DeviceType::Input => virtio_input(header),
//             DeviceType::Network => {
//                 debug!("virtio network");
//                 virtio_net(header)
//             }
//             t => warn!("Unrecognized virtio device: {:?}", t),
//         }
//     }
// }

// fn virtio_blk(header: &'static mut VirtIOHeader) {
//     let mut blk = VirtIOBlk::new(header).expect("failed to create blk driver");
//     let mut input = vec![0xffu8; 512];
//     let mut output = vec![0; 512];
//     for i in 0..32 {
//         for x in input.iter_mut() {
//             *x = i as u8;
//         }
//         blk.write_block(i, &mut input).expect("failed to write");
//         blk.read_block(i, &mut output).expect("failed to read");
//         assert_eq!(input, output);
//     }
//     info!("virtio-blk test finished");
// }

// fn virtio_gpu(header: &'static mut VirtIOHeader) {
//     let mut gpu = VirtIOGpu::new(header).expect("failed to create gpu driver");
//     let fb = gpu.setup_framebuffer().expect("failed to get fb");
//     for y in 0..768 {
//         for x in 0..1024 {
//             let idx = (y * 1024 + x) * 4;
//             fb[idx] = x as u8;
//             fb[idx + 1] = y as u8;
//             fb[idx + 2] = (x + y) as u8;
//         }
//     }
//     gpu.flush().expect("failed to flush");
//     info!("virtio-gpu test finished");
// }

// fn virtio_input(header: &'static mut VirtIOHeader) {
//     let mut event_buf = [0u64; 32];
//     let mut _input =
//         VirtIOInput::new(header, &mut event_buf).expect("failed to create input driver");
//     // loop {
//     //     _input.ack_interrupt().expect("failed to ack");
//     //     info!("mouse: {:?}", _input.mouse_xy());
//     // }
//     // TODO: handle external interrupt
// }

// fn virtio_net(header: &'static mut VirtIOHeader) {
//     let mut net = VirtIONet::new(header).expect("failed to create net driver");
//     let mut buf = [0u8; 0x100];
//     let len = net.recv(&mut buf).expect("failed to recv");
//     info!("recv: {:?}", &buf[..len]);
//     net.send(&buf[..len]).expect("failed to send");
//     info!("virtio-net test finished");
// }




