#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![allow(unused)]
// #![feature(const_generics)]

pub mod thread;
pub mod console;
pub mod lang_items;
// pub mod runtime;
// pub mod scheduler;
pub mod syscall;
pub mod init;
pub mod task;
pub mod sbi;

pub use thread::*;
pub use runtime::*;
pub use task::*;
pub use scheduler::*;

extern crate alloc;


#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {

    let mut space_id :usize;
    // unsafe {
    //     HEAP.lock()
    //         .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    // }
    // unsafe{asm!("mv {}, tp", out(reg) space_id, options(nomem, nostack));}

    // println!(" space_id : {:#x}", space_id);
    exit( main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    crate::thread::init_cpu_test();
    crate::thread::thread_init();
    panic!("Cannot find main!");
}

use syscall::*;




