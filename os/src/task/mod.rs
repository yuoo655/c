mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;
mod pool;

use crate::fs::{open_file, OpenFlags};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use alloc::{sync::Arc, string::ToString};
pub use pool::{add_task, fetch_task, prioritize_task};
use lazy_static::*;
pub use context::TaskContext;

// pub use task::SPACE;

pub use processor::{
    run_tasks,
    current_task,
    current_user_token,
    current_trap_cx,
    take_current_task,
    schedule,
    hart_id,
};
pub use pid::{PidHandle, pid_alloc, KernelStack};

use spin::Mutex;

lazy_static! {
    pub static ref WAIT_LOCK: Mutex<()> = Mutex::new(());
}



pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = task_inner.get_task_cx_ptr();
    // let task_cx_ptr = task_inner.gets_task_cx_ptr();
    drop(task_inner);

    // jump to scheduling cycle
    // add_task(task);
    schedule(task_cx_ptr);
}




// pub fn switch_to_spaceid(space_id:usize){
    
//     let idle = 0 as usize;
//     let target_context_ptr = SPACE.lock().get_context_ptr(space_id);

//     unsafe {
//         __switch(
//             &idle  as *const usize,
//             &target_context_ptr as *const usize,
//         );
//     }
// }

pub fn exit_current_and_run_next(exit_code: i32) {
    
    // take from Processor
    let task = take_current_task().unwrap();

    // **** hold current PCB lock
    let mut inner = task.acquire_inner_lock();

    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;

    // for child in inner.children.iter() {
    //     child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
    //     initproc_inner.children.push(child.clone());
    // }

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    
    // **** release current PCB lock
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();

    warn!("exit_current_and_run_next schedule");
    schedule(&mut _unused as *mut TaskContext);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 0 as usize)
    });
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}


pub fn add_user_test(){

    // for i in 1..9 {
    //     let task = Arc::new({
    //         let inode = open_file(i.to_string().as_str(), OpenFlags::RDONLY).unwrap();
    //         let v = inode.read_all();
    //         TaskControlBlock::new(v.as_slice(), 0 as usize)
    //     });
    //     add_task(task.clone());
    //     drop(task);
    // }

    let task1 = Arc::new({
        let inode = open_file("1", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task2 = Arc::new({
        let inode = open_file("2", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task3 = Arc::new({
        let inode = open_file("3", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task4 = Arc::new({
        let inode = open_file("4", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task5 = Arc::new({
        let inode = open_file("5", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task6 = Arc::new({
        let inode = open_file("6", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task7 = Arc::new({
        let inode = open_file("7", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });
    let task8 = Arc::new({
        let inode = open_file("8", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice(), 1 as usize)
    });


    add_task(task1.clone());    
    add_task(task2.clone());    
    add_task(task3.clone());    
    add_task(task4.clone());    
    add_task(task5.clone());    
    add_task(task6.clone());    
    add_task(task7.clone());    
    add_task(task8.clone());    
}