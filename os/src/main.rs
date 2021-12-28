#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
// #![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]
#![allow(unused)]

use alloc::alloc::dealloc;
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
    // lkm::init();



    // test1();
    // scheduler::thread_init();
    // test_for_kernel(0);


    task::add_user_test();

    println!("run user task");
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

pub fn test1(){
    unsafe{
        let init_environment_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_environment");
        let init_environment: fn() = core::mem::transmute(init_environment_addr as usize);
        init_environment();
    }
    let thread_init_addr = lkm::get_symbol_addr_from_elf("basic_rt", "thread_init");
    unsafe{
        let thread_init: fn() = core::mem::transmute(thread_init_addr as usize);
        println!("cpu_run");
        thread_init();
        println!("cpu_run done");
    }
    
}


pub fn test_for_kernel(base: usize){
    let init_environment_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_environment");
    println!("init_environment at {:#x?}", init_environment_addr);
    

    let init_cpu_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_cpu_test");
    println!("init_cpu at {:#x?}", init_cpu_addr);

    let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
    println!("cpu_run at {:#x?}", cpu_run_addr);

    let add_user_task_addr = lkm::get_symbol_addr_from_elf("basic_rt", "add_user_task");
    println!("add_user_task at {:#x?}", add_user_task_addr);

    let add_user_task_1_addr = lkm::get_symbol_addr_from_elf("basic_rt", "add_user_task_1");
    println!("add_user_task at {:#x?}", add_user_task_1_addr);

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

        let add_task: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
            core::mem::transmute(add_user_task_addr as usize + base)
        };

        let add_task_1: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
            core::mem::transmute(add_user_task_1_addr as usize + base)
        };

        println!("init_environment");
        init_environment();
        
        
        println!("init_cpu");
        init_cpu();

        async fn test(x: i32) {
            crate::println!("{}", x);
        }
        println!("test task addr :{:#x?}", test as usize);

        println!("add_task");
        let add_task_1 : fn(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>) -> () = unsafe {
            core::mem::transmute(add_user_task_1_addr as usize + base)
        };

        add_task_1(Box::pin(test(666)));


        let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
        unsafe{
            let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize);
            println!("cpu_run");
            cpu_run();
        }
    }

}


pub fn test_odd(base: usize){
    let init_environment_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_environment");
    println!("init_environment at {:#x?}", init_environment_addr);
    

    let init_cpu_addr = lkm::get_symbol_addr_from_elf("basic_rt", "init_cpu_test");
    println!("init_cpu at {:#x?}", init_cpu_addr);

    let cpu_run_addr = lkm::get_symbol_addr_from_elf("basic_rt", "cpu_run");
    println!("cpu_run at {:#x?}", cpu_run_addr);

    let add_user_task_addr = lkm::get_symbol_addr_from_elf("basic_rt", "add_user_task");
    println!("add_user_task at {:#x?}", add_user_task_addr);

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


        let add_task: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
            core::mem::transmute(add_user_task_addr as usize + base)
        };
        async fn test(x: i32) {
            println!("{}", x);
        }
        println!("test addr :{:#x?}", test as usize);
        add_task(Mutex::new(Box::pin(test(2))));


        init_environment();
        println!("init_environment done");
        
        llvm_asm!("sfence.vma" :::: "volatile");
        
        println!("init_cpu");
        init_cpu();
        println!("init_cpu done");


        // println!("add_task done");

    }
}




