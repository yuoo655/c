#![no_std]

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::VecDeque;

use core::task::{Context, Poll};
use core::mem;
use spin::Mutex;
use woke::waker_ref;
use lazy_static::*;

use bit_field::BitField;

use crate::println;


#[no_mangle]
lazy_static! {
    pub static ref BITMAP: Arc<Mutex<Box<BitMap>>> = Arc::new(Mutex::new(Box::new(BitMap::new())));
}

#[derive(Clone, Copy)]
pub struct  BitMap(usize);

impl BitMap {
    pub fn new() -> BitMap{
        let mut bitmap: &mut BitMap;
        unsafe{
            bitmap = &mut *(0x87420000 as *mut BitMap);
        }
        bitmap.0 = 0;
        *bitmap.deref()
    }
    pub fn set(&mut self, id: usize, value:bool) {
        self.0.set_bit(id, value);
    }
    pub fn get(&mut self, id: usize) -> bool {
        self.0.get_bit(id)
    }

    pub fn get_priority(&self) -> usize {
        for i in 0..7 {
            if self.0.get_bit(i){
                return i;
            }
        }
        
        7
    }
    pub fn get_sys_bitmap() -> BitMap{
        let mut bitmap: BitMap;
        unsafe{
            bitmap = *(0x87410000 as *mut BitMap);
        }
        bitmap
    }
}



pub fn check_bitmap_should_yield() -> bool{
    // user bitmap  0x87420000
    // kernel bitmap 0x87410000
    unsafe{
        let kernel_bitmap = *(0x87410000 as *mut BitMap);

        let kernel_highest_priority = kernel_bitmap.get_priority();

        let user_highest_priority = BITMAP.lock().get_priority();

        if kernel_highest_priority > user_highest_priority{
            return true
        }
    }
    false
}

#[no_mangle]
lazy_static! {
    pub static ref REACTOR: Arc<Mutex<Box<Reactor>>> = Reactor::new();
}



#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct TaskId(usize);

impl TaskId {
    pub(crate) fn generate() -> TaskId {
        // 任务编号计数器，任务编号自增
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > usize::MAX / 2 {
            // TODO: 不让系统 Panic
            panic!("too many tasks!")
        }
        TaskId(id)
    }
}



//Task包装协程
pub struct UserTask{
    // 任务编号
    pub id: TaskId,
    // future
    pub future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>, 
    // reactor
    pub reactor: Arc<Mutex<Box<Reactor>>>,
}

impl UserTask{
    //创建一个协程
    pub fn spawn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> Self{
        UserTask{
            id: TaskId::generate(),
            future: future,
            reactor: REACTOR.clone(),
        }
    }

    pub fn do_wake(self: &Arc<Self>) {
        // todo!()
    }
}


impl Future for UserTask {
    type Output = usize;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock();
        if r.is_ready(self.id) {
            Poll::Ready(self.id.0)
        } else if r.contains_task(self.id) {
            r.add_task(self.id);
            Poll::Pending
        } else {
        let mut f = self.future.lock();
        match f.as_mut().poll(cx) {
            Poll::Ready(_) => {
                Poll::Ready(0)
            },
            Poll::Pending => {
                r.register(self.id); // fixme
                Poll::Pending
            }
        }
    
        }
    }
}

pub struct UserTaskQueue {
    pub queue: Vec<VecDeque<Arc<UserTask>>>,
}



//用户协程队列
impl UserTaskQueue {
    pub fn new() -> Self {
        let queue = (0..4).map(|_| VecDeque::new() ).collect::<Vec<VecDeque<Arc<UserTask>>>>();
        UserTaskQueue {
            queue,
        }
    }

    pub fn add_task(&mut self, task: UserTask, priority: Option<usize>) {
        let p = priority.unwrap_or(0);
        self.queue[p].push_front(Arc::new(task));
    }

    pub fn add_arc_task(&mut self, task: Arc<UserTask>, priority: usize) {
        self.queue[priority].push_back(task);
    }

    pub fn peek_task(&mut self) -> Option<Arc<UserTask>> {

        let mut ret = None;

        for i in 0..self.queue.len() {
            if self.queue[i].len() !=0 {
                BITMAP.lock().set(i, true);
                let x =  self.queue[i].pop_front();
                return x
            }
        }
        return ret
    }
    
    pub fn is_all_empty(&self) -> bool {

        for i in 0..self.queue.len() {
            if !self.queue[i].is_empty() {
                return false
            }
        }
        return  true;
    }

}



pub enum TaskState {
    Ready,
    NotReady,
    Finish,
}

pub struct Reactor {
    tasks: BTreeMap<TaskId, TaskState>,
}

impl Reactor {
    pub(crate) fn new() -> Arc<Mutex<Box<Self>>> {
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            tasks: BTreeMap::new(),
        })));
        reactor
    }

    pub(crate) fn wake(&mut self, id: TaskId) {
        let state = self.tasks.get_mut(&id).unwrap();
        match mem::replace(state, TaskState::Ready) {
            TaskState::NotReady => (),
            TaskState::Finish => panic!("Called 'wake' twice on task: {:?}", id),
            _ => unreachable!()
        }
    }

    pub(crate) fn register(&mut self, id: TaskId) {
        if self.tasks.insert(id, TaskState::NotReady).is_some() {
            panic!("Tried to insert a task with id: '{:?}', twice!", id);
        }
    }

    pub(crate) fn is_ready(&self, id: TaskId) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub(crate) fn get_task(&self, task_id: TaskId) -> Option<&TaskState> {
        self.tasks.get(&task_id)
    }

    pub(crate) fn get_task_mut(&mut self, task_id: TaskId) -> Option<&mut TaskState> {
        self.tasks.get_mut(&task_id)
    }

    pub(crate) fn add_task(&mut self, task_id: TaskId) -> Option<TaskState> {
        self.tasks.insert(task_id, TaskState::NotReady)
    }

    pub(crate) fn contains_task(&self, task_id: TaskId) -> bool {
        self.tasks.contains_key(&task_id)
    }

    pub(crate) fn is_finish(&self, task_id: TaskId) -> bool {
        self.tasks.get(&task_id).map(|state| match state {
            TaskState::Finish => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub(crate) fn finish_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id, TaskState::Finish);
    }

    pub(crate) fn remove_task(&mut self, task_id: TaskId) -> Option<TaskState>{
        self.tasks.remove(&task_id)
    }
}




impl woke::Woke for UserTask {
    fn wake_by_ref(task: &Arc<Self>) {
        task.do_wake()
    }
}

impl Drop for UserTask {
    fn drop(&mut self) {
        let r = self.reactor.clone();
        let mut r = r.lock();
        r.remove_task(self.id);
    }
}




//传递用户协程队列
pub fn diliver_to_kernel(){
    //to do
}


//检查kernel提供给用户的调度信息
pub fn check_kernel_clue(){
    //to do
    // println!("checking clue.");
}








