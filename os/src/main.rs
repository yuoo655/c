#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
// #![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]
#![allow(unused)]
// use std::println;

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


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));




fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    
    mm::init();
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();


    info!("loader list app");
    fs::list_apps();
    

    // scheduler::init();
    // scheduler::thread::init();


    // println!("[kernel] init scheduler mem");
    // unsafe {
        //     llvm_asm!("auipc ra, 0");
        //     llvm_asm!("jalr ra, $0" :: "r"(0x86000462 as usize));
        // }
        
    // lkm::init();
    task::add_user_test();
    
    // let entry = get_symbol_addr_from_elf("tiny_kernel", "init_payload_environment");

    
    
    // let init_payload_environment_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_payload_environment");
    // println!("init_payload_environment at {:#x?}", init_payload_environment_addr);
    

    // let init_cpu_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_cpu");
    // println!("init_cpu at {:#x?}", init_cpu_addr);

    // let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
    // println!("cpu_run at {:#x?}", cpu_run_addr);

    // let add_user_task_addr = lkm::get_symbol_addr_from_elf("basic_rt", "add_user_task");
    // println!("add_user_task at {:#x?}", add_user_task_addr);


    // unsafe{
    //     let init_payload_environment: fn() = core::mem::transmute(init_payload_environment_addr as usize + 0x86000000);
    // }

    // use spin::Mutex;
    // use woke::waker_ref;
    // use core::future::Future;
    // use core::pin::Pin;
    // use alloc::boxed::Box;


    // unsafe{
        
    //     let init_payload_environment: fn() = core::mem::transmute(init_payload_environment_addr as usize + 0x86000000);
        
    //     let init_cpu: fn() = core::mem::transmute(init_cpu_addr as usize + 0x86000000);
        
    //     // let add_user_task: fn() = core::mem::transmute(add_user_task_addr as usize + 0x86000000);
    //     let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize + 0x86000000);

    //     let add_task: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
    //         core::mem::transmute(add_user_task_addr as usize + 0x86000000)
    //     };

    //     async fn test(x: i32) {
    //         crate::println!("{}", x);
    //     }

    //     println!("add_task");
    //     add_task(Mutex::new(Box::pin(test(2))));

    //     println!("cpu_run");
    //     cpu_run();
    // }

    // let add_task: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
    //     core::mem::transmute(0x8600000 as usize)
    // };


    task::run_tasks();
    panic!("Unreachable in rust_main!");
}




// pub unsafe fn add_task(
//     &self,
//     hart_id: usize,
//     address_space_id: AddressSpaceId,
//     task_repr: usize,
// ) -> bool {
//     let f = self.shared_add_task;
//     f(self.shared_scheduler, hart_id, address_space_id, task_repr)
// }



// pub fn spawn(future: impl Future<Output = ()> + Send + Sync + 'static) {
//     let shared_payload = unsafe { task::shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
//     let asid = unsafe { task::shared::AddressSpaceId::from_raw(ADDRESS_SPACE_ID) };

//     let task = task::new_user(
//         future,
//         shared_payload.shared_scheduler,
//         shared_payload.shared_set_task_state,
//     );

//     unsafe {
//         shared_payload.add_task(0 /* todo */, asid, task.task_repr());
//     }
// }