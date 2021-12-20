#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#[macro_use]
extern crate user_lib;
use user_lib::console::print;

use core::mem;

use user_lib::{exit};



#[no_mangle]
pub fn main() -> i32 {


    println!("[user2] Hello world from user mode program!");

    thread_init();
    


    // unsafe {
    //     let init_payload_environment: unsafe extern "C" fn() = unsafe {core::mem::transmute(0x8600048e as usize)};
    //     println!("calling init_payload_environment at {:?}", init_payload_environment);
    //     init_payload_environment();
    // }

    
    // let addr = test as usize;
    // println!("[user2] add thread to scheduler  entry addr {:#x} space_id {:#x}", addr, 2);
    // let add_to_thread_pool: unsafe extern "C" fn(usize, usize) = unsafe { core::mem::transmute(0x86001ea8 as usize) };
    // unsafe { add_to_thread_pool(addr, 2 as usize) };


    // yield_();
    // let run_: unsafe extern "C" fn() = unsafe { core::mem::transmute(0x86001caa as usize) };
    // unsafe { run_() };
    exit(0);
}


pub fn test(){
    println!("hello world! from --------------------- user2");
    exit(0);
}



use spin::Mutex;
use core::usize::MAX;
extern crate alloc;
use alloc::boxed::Box;
use user_lib::scheduler::task::*;
use user_lib::scheduler::thread::*;

pub fn thread_init() {
    println!("scheduler init");

    // 使用 Fifo Scheduler
    // let scheduler = FifoScheduler::new();
    let scheduler = RRScheduler::new(50);

    // 新建线程池
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));


    let entry = Processor::idle_main as usize;

    // 新建idle ，其入口为 Processor::idle_main
    let idle = Thread::new_box_thread(entry, &CPU as *const Processor as usize);


    CPU.init(idle, Box::new(thread_pool));

    // 新建一个thread_main加入线程池
    
    CPU.add_thread(
        {
            let thread = Thread::new_box_thread(thread_main as usize, 1);
            thread
        }
    );

    async fn foo(x:usize){
        println!("{:?}", x);
    }


    let mut queue = USER_TASK_QUEUE.lock();
    for i in 0..100 {
        queue.add_task(runtime::UserTask::spawn(Mutex::new(Box::pin( foo(i)))) );
    }

    drop(queue);
    
    println!("scheduler cpu run");
    CPU.run();
}
