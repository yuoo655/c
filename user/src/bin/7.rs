#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#[macro_use]
extern crate user_lib;
use user_lib::*;

extern crate alloc;


#[no_mangle]
pub fn main() -> i32 {

    println!("[user7] Hello world from user mode program!");

    test_for_user();

    println!("[user7] end");

    0
}

pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}


pub fn test_for_user(){

    // let base = 0x86000000 - 0x87000000;
    let init_environment_addr = get_symbol_addr("init_environment\0") as usize - 0x1000000;
    let init_cpu_addr = get_symbol_addr("init_cpu_test\0") as usize - 0x1000000;
    let cpu_run_addr = get_symbol_addr("cpu_run\0") as usize    - 0x1000000;
    let add_user_task_with_priority_addr = get_symbol_addr("add_user_task_with_priority\0") as usize   - 0x1000000;
    // println!("init_environment at {:#x?}", init_environment_addr);
    // println!("init_cpu at {:#x?}", init_cpu_addr);
    // println!("cpu_run at {:#x?}", cpu_run_addr);
    // println!("add_user_task at {:#x?}", add_user_task_with_priority_addr);

    use spin::Mutex;
    use woke::waker_ref;
    use core::future::Future;
    use core::pin::Pin;
    use alloc::boxed::Box;


    unsafe{
        
        let init_environment: fn() = core::mem::transmute(init_environment_addr as usize );
        
        let init_cpu: fn()= core::mem::transmute(init_cpu_addr as usize);
        
        let cpu_run: fn() = core::mem::transmute(cpu_run_addr as usize);

        let add_task_with_priority : fn(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>> , Option<usize>) -> () = unsafe {
            core::mem::transmute(add_user_task_with_priority_addr as usize)
        };

        // println!("init_environment");
        init_environment();
        
        // println!("init_cpu");
        init_cpu();

        async fn test(x: i32) {
            println!("hart[{}] [user7] {}", hart_id(),x);
        }
        // println!("test task addr :{:#x?}", test as usize);
        // println!("add_task");

        for i in 0..10{
            add_task_with_priority(Box::pin(test(i)), Some(0));
        }
        add_task_with_priority(Box::pin(test(666)), Some(0));

        // println!("cpu_run");
        cpu_run();
    }

}