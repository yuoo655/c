
// use super::runtime::*;

pub mod runtime;
use runtime::*;

use alloc::sync::Arc;
use alloc::boxed::Box;
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

lazy_static! {
    pub static ref USER_TASK_QUEUE: Arc<Mutex<Box<UserTaskQueue>>> =
        Arc::new(
            Mutex::new(
                Box::new(
                    UserTaskQueue {
                        queue: VecDeque::new()
                    }
                )
            )
        );
}


#[no_mangle]
pub fn thread_main() {

    // println!("thread_main-------------------");
    loop {
        let mut queue = USER_TASK_QUEUE.lock();
        let task = queue.peek_task();
        // println!("running, queue len: {}, task: {:?}", queue.queue.len(), task.is_none());

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
                            // // 任务完成
                            // println!("task completed");
                        }
                        Poll::Pending => {
                            r.register(task.id);
                        }
                    }
                }
            }
            None => {
                let mut queue_len = USER_TASK_QUEUE.lock().queue.len();

                if queue_len == 0 {
                    println!("queue len 0 no task, exit");
                    crate::sys_exit(0);
                }

                return;
                // crate::scheduler::thread::CPU.exit(0);
            }
                
        }

        // crate::sys_exit(0);
    }
}


#[no_mangle]
pub fn add_user_task(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>){
    let mut queue = USER_TASK_QUEUE.lock();
    let task = UserTask::spawn(future);
    queue.add_task(task);
    drop(queue);
}


#[no_mangle]
pub fn add_user_task_1(future: Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>){
    let mut queue = USER_TASK_QUEUE.lock();
    let task = UserTask::spawn(Mutex::new(future));
    queue.add_task(task);
    drop(queue);
}