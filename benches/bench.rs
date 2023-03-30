#![feature(test)]
#![feature(new_uninit)]

extern crate test;

use test::{Bencher, black_box};
use static_mem_pool::*;

#[bench]
fn static_mem_pool(b: &mut Bencher) {
    static mut POOL: StaticMemPool<4096, 16> = StaticMemPool::new();

    b.iter(|| unsafe {
        black_box(POOL.borrow().unwrap());
    });
}

#[bench]
fn r#box(b: &mut Bencher) {
    //

    b.iter(|| {
        black_box(Box::<[u8; 4096]>::new_uninit());
    });
}