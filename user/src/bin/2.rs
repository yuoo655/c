#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#[macro_use]
extern crate user_lib;
use user_lib::console::print;

use core::mem;

use user_lib::{exit, get_symbol_addr};



#[no_mangle]
pub fn main() -> i32 {


    // let hart_id = hart_id();
    // println!("[hart {}] [user2] Hello world from user mode program!", hart_id);
    println!("[user2] Hello world from user mode program!");

    // thread_init();
    test_for_user();
    exit(0);
}


pub fn test_for_user(){

    // let base = 0x86000000 - 0x87000000;
    let init_environment_addr = get_symbol_addr("init_environment\0") as usize - 0x87000000 + 0x86000000;
    println!("init_environment at {:#x?}", init_environment_addr);
    

    let init_cpu_addr = get_symbol_addr("init_cpu_test\0") as usize - 0x87000000 + 0x86000000;
    println!("init_cpu at {:#x?}", init_cpu_addr);

    let cpu_run_addr = get_symbol_addr("cpu_run\0") as usize    - 0x87000000 + 0x86000000;
    println!("cpu_run at {:#x?}", cpu_run_addr);


    let add_user_task_with_priority_addr = get_symbol_addr("add_user_task_with_priority\0") as usize   - 0x87000000 + 0x86000000;
    println!("add_user_task at {:#x?}", add_user_task_with_priority_addr);

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

        println!("init_environment");
        init_environment();
        
        
        println!("init_cpu");
        init_cpu();


        async fn test(x: i32) {
            println!("[hart {}] [user2] {}", hart_id(),x);
        }
        println!("test task addr :{:#x?}", test as usize);
        println!("add_task");

        for i in 0..10{
            add_task_with_priority(Box::pin(test(i)), Some(0));
        }
        add_task_with_priority(Box::pin(test(666)), Some(0));

        // println!("cpu_run");
        cpu_run();
    }

}


pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
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
