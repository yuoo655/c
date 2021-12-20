pub mod task;
pub mod thread;

use spin::Mutex;
use core::usize::MAX;

// use crate::runtime::*;
// use crate::runtime;

use task::*;
use task::runtime::*;

use thread::*;
use crate::println;


extern crate alloc;
use alloc::boxed::Box;




async fn foo(x:usize){
    println!("{:?}", x);
}

pub fn init(){
    let mut queue = USER_TASK_QUEUE.lock();
    for i in 0..100 {
        queue.add_task(UserTask::spawn(Mutex::new(Box::pin( foo(i) ))) );
        // if i % 10_000_000 == 0 {
        //     println!("count {:?}", i);
        // }
    }
    drop(queue);
    task::run();
}


pub fn thread_init() {

    println!("scheduler init");

    // 使用 Fifo Scheduler
    let scheduler = crate::scheduler::thread::scheduler::FifoScheduler::new();
    // 新建线程池
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    // 新建idle ，其入口为 Processor::idle_main
    let idle = Thread::new_box_thread(Processor::idle_main as usize, &CPU as *const Processor as usize);

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


    let mut queue = crate::scheduler::task::USER_TASK_QUEUE.lock();
    for i in 0..100 {
        queue.add_task(crate::scheduler::task::runtime::UserTask::spawn(Mutex::new(Box::pin( foo(i) ))) );
    }
    drop(queue);

    println!("scheduler cpu run");
    CPU.run();
}