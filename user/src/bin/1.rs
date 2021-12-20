#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#[macro_use]
extern crate user_lib;
use user_lib::console::print;

use core::mem;

use user_lib::*;



extern crate alloc;
use spin::Mutex;
use woke::waker_ref;
use core::future::Future;
use core::pin::Pin;
use alloc::boxed::Box;


#[no_mangle]
pub fn main() -> i32 {



    println!("[user1] Hello world from user mode program!");


    // let init_payload_environment_addr = 0 as usize;
    // let init_cpu_addr= 0x5bc4 as usize;
    // let cpu_run_addr = 0x5d20 as usize;
    // let add_user_task_addr = 0x1738a as usize;


    // unsafe{
        
    //     let init_payload_environment: fn() = core::mem::transmute(init_payload_environment_addr as usize + 0x86000000);
        
    //     let init_cpu: fn() = core::mem::transmute(init_cpu_addr as usize + 0x86000000);
        
    //     let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize + 0x86000000);

    //     let add_task: fn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> () = unsafe {
    //         core::mem::transmute(add_user_task_addr as usize + 0x86000000)
    //     };

    //     println!("init_payload_environment");
    //     init_payload_environment();

    //     println!("init_cpu");
    //     init_cpu();

    //     async fn test(x: i32){         
    //         crate::print!("{}", x);
    //     }

    //     println!("add_task");
    //     add_task(Mutex::new(Box::pin(test(2))));



    //     println!("cpu_run");
    //     cpu_run();
    // }


    // task::run_tasks();


    // unsafe {
    //     let init_payload_environment: unsafe extern "C" fn() = unsafe {core::mem::transmute(0x8600048e as usize)};
    //     println!("calling init_payload_environment = {:?}", init_payload_environment);
    //     init_payload_environment();
    // }


    // let addr = test as usize;
    // println!("[user1] add thread to scheduler  entry addr {:#x} space_id {:#x}", addr, 1);
    // let add_to_thread_pool: unsafe extern "C" fn(usize, usize) -> ()= unsafe { core::mem::transmute(0x86001ea8 as usize) };
    // unsafe { add_to_thread_pool(addr, 1 as usize) };

    // yield_();
    // let run_: unsafe extern "C" fn() = unsafe { core::mem::transmute(0x86001caa as usize) };
    // println!("[user1] run_");
    // unsafe {run_()};

    println!("[user1] end");

    0
}


// pub fn test(){
//     println!("hello world! from --------------------- user1");
//     exit(0);
// }

// fn execute_unexecutable_test() {
//     println!("execute_unexecutable_test");
//     unsafe {
//         llvm_asm!("jr $0" :: "r"(0x87007000 as usize) :: "volatile");
//     }
//     println!("pass");
// }