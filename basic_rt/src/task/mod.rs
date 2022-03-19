
// use super::runtime::*;

pub mod runtime;
use runtime::*;
use runtime::UserTaskQueue;
use alloc::sync::Arc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::VecDeque;

use core::task::{Context, Poll};
use core::mem;
use spin::Mutex;
use woke::waker_ref;
use lazy_static::*;
use crate::println;



// use crate::println;

lazy_static! {
    pub static ref USER_TASK_QUEUE: Arc<Mutex<Box<UserTaskQueue>>> =
        Arc::new(
            Mutex::new(
                Box::new(
                    UserTaskQueue::new(),
                )
            )
        );
}

pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}


#[no_mangle]
pub fn thread_main() {

    println!("thread_main-------------");
    loop {
        let x = check_bitmap_should_yield();
        
        let mut queue = USER_TASK_QUEUE.lock();
        let task = queue.peek_task();
        println!("thread_main running, have task {:?}", !task.is_none());

        match task {
            // have any task
            Some(task) => {
                let mywaker = task.clone();
                let waker = waker_ref(&mywaker);
                let mut context = Context::from_waker(&*waker);

                let r = task.reactor.clone();
                let mut r = r.lock();

                if r.is_ready(task.id) {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // 任务完成
                            r.finish_task(task.id);
                        }
                        Poll::Pending => {
                            r.add_task(task.id);
                        }
                    }
                } else if r.contains_task(task.id) {
                    r.add_task(task.id);
                } else {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // 任务完成
                            // println!("task completed");
                        }
                        Poll::Pending => {
                            r.register(task.id);
                        }
                    }
                }
            }
            None => {
                println!("no task");
                // let mut queue = USER_TASK_QUEUE.lock();
                // if queue.is_all_empty(){
                //     crate::sys_exit(0);
                // }
                crate::sys_exit(0);
                break;

            }
                
        }

        // crate::sys_exit(0);
    }
}

#[no_mangle]
pub fn add_user_task(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>){
    let mut queue = USER_TASK_QUEUE.lock();
    let task = UserTask::spawn(Mutex::new(future));
    queue.add_task(task , Some(0));
    drop(queue);
}


#[no_mangle]
pub fn add_user_task_with_priority(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>, priority: usize){
    let mut queue = USER_TASK_QUEUE.lock();
    let task = UserTask::spawn(Mutex::new(future));
    queue.add_task(task , Some(priority));
    drop(queue);
}

