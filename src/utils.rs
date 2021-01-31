use core::ops::{BitAnd, Not};
use num_traits::Num;

pub fn align_up<T: Num + Not<Output = T> + BitAnd<Output = T> + Copy>(addr: T, align: T) -> T {
    align_down(addr + (align - T::one()), align)
}

pub fn align_down<T: Num + Not<Output = T> + BitAnd<Output = T> + Copy>(addr: T, align: T) -> T {
    addr & !(align - T::one())
}

pub fn get_current_el() -> u32 {
    let current_el: u32;
    unsafe {
        asm!("mrs {el}, CurrentEL", el = out(reg) current_el, options(nostack));
    }

    current_el >> 2
}
