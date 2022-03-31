use bit_field::BitField;
use spin::Mutex;
use alloc::boxed::Box;
use alloc::sync::Arc;


#[derive(Clone, Copy)]
pub struct  BitMap(pub usize);

impl BitMap {
    pub fn new() -> BitMap{
        let mut bitmap: &mut BitMap;
        unsafe{
            // bitmap = &mut *(0x87810000 as *mut BitMap);
            bitmap = &mut *(0x87410000 as *mut BitMap);
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

    pub fn get_priority(&mut self, id: usize) -> usize {
        for i in 0..4 {
            if self.0.get_bit(i){
                return i;
            }
        }
        4
    }
    pub fn get_sys_bitmap() -> BitMap{
        let mut bitmap: BitMap;
        unsafe{
            bitmap = *(0x20000 as *mut BitMap);
        }
        bitmap
    }

    pub fn inner(&mut self) -> &mut usize {
        &mut self.0
    }

    pub fn update_bitmap(&mut self){

        //kernel_bitmap va = pa = 0x87410000 
        //user_bitmap  0x87400000 + PAGE_SIZE*pid  (pid start from 1)  0x87401000..0x87404000

        for i in 1..5 {
            // let start_addr = 0x87400000 as usize;
            let user_bitmap = unsafe { &*( (0x87400000 + PAGE_SIZE*i) as *const BitMap) };

            self.0  = self.0 | user_bitmap.0;
        }

        debug!("hard [{}] update bitmap", crate::hart_id());
    }
}


use crate::config::PAGE_SIZE;
use lazy_static::*;

#[no_mangle]
lazy_static! {
    pub static ref KERNEL_BITMAP: Arc<Mutex<Box<BitMap>>> = Arc::new(Mutex::new(Box::new(BitMap::new())));
}

// pub fn update_bitmap(){
//     let mut bitmap = KERNEL_BITMAP.inner().lock();

//     for i in 0..7 {
//         let start_addr = 0x8900_0000 as usize;
//         let user_bitmap = unsafe { &*( (start_addr + PAGE_SIZE *i) as *const BitMap) };

//         bitmap = bitmap.as_bytes || user_bitmap.as_bytes;
//     }

//     debug!("update bitmap ---------------------------");
// }