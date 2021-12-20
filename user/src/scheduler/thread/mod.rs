pub mod context;
pub mod thread;
pub mod user_stack;
pub mod scheduler;
pub mod thread_pool;
pub mod processor;
pub mod fifo;

pub use processor::Processor;
pub use scheduler::FifoScheduler;
pub use scheduler::*;
pub use thread_pool::ThreadPool;
pub use thread::*;


use alloc::boxed::Box;
use alloc::{sync::Arc};
use spin::Mutex;
use lazy_static::*;



use super::task::thread_main;

use crate::println;

// use fifo::THREAD_MANAGER;

pub type Tid = usize;
pub static CPU : Processor = Processor::new();

pub fn init() {
    // // 使用 Fifo Scheduler
    // let scheduler = FifoScheduler::new();
    // // 新建线程池
    // let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    // // 新建idle ，其入口为 Processor::idle_main
    // let idle = Thread::new_box_thread(Processor::idle_main as usize, &CPU as *const Processor as usize);

    // // 初始化 CPU
    // CPU.init(idle, Box::new(thread_pool));


    println!("scheduler init");

    // 使用 Fifo Scheduler
    // let scheduler = FifoScheduler::new();
    let scheduler = RRScheduler::new(50);
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
    for i in 0..10 {
        queue.add_task(crate::scheduler::task::runtime::UserTask::spawn(Mutex::new(Box::pin( foo(i)))) );
    }

    drop(queue);
    
    println!("scheduler cpu run");
    CPU.run();
}


#[no_mangle]
pub fn init_thread_pool(){
    // 使用 Fifo Scheduler
    let scheduler = FifoScheduler::new();
    // 新建线程池
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    // 新建idle ，其入口为 Processor::idle_main
    let idle = Thread::new_box_thread(Processor::idle_main as usize, &CPU as *const Processor as usize);

    CPU.init(idle, Box::new(thread_pool));
}



#[no_mangle]
pub fn init_cpu(){
    // 使用 Fifo Scheduler
    let scheduler = FifoScheduler::new();
    // 新建线程池
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    // 新建idle ，其入口为 Processor::idle_main
    let idle = Thread::new_box_thread(Processor::idle_main as usize, &CPU as *const Processor as usize);

    CPU.init(idle, Box::new(thread_pool));

    CPU.add_thread(
        {
            let thread = Thread::new_box_thread(thread_main as usize, 1);
            thread
        }
    );
}

#[no_mangle]
pub fn cpu_run(){
    CPU.run();
}

// pub fn add_to_thread_pool(addr: usize, space_id:usize) {
//     THREAD_MANAGER.lock().add(
//         {
//             let thread = Thread::new_thread(addr, space_id);
//             thread
//         }
//     );
// }


