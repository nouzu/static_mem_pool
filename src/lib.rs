#![no_std]

#![feature(const_for)]
#![feature(const_iter)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_intoiterator_identity)]

use core::mem::transmute;
use core::ptr::NonNull;

pub struct StaticMemPool<const L: usize, const N: usize>(const_collections::vec::Vec<NonNull<[u8; L]>, N>);

impl<const L: usize, const N: usize> StaticMemPool<L, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        let mut vec = const_collections::vec::Vec::new();

        for _ in 0..N {
            unsafe { vec.push_unchecked(transmute(const_collections::alloc::alloc_array::<u8, L>())); }
        }

        Self(vec)
    }

    #[inline(always)]
    pub fn borrow(&mut self) -> Option<Block<L, N>> {
        if self.0.is_empty() { None } else { Some(unsafe { self.borrow_unchecked() }) }
    }

    #[inline(always)]
    pub unsafe fn borrow_unchecked(&mut self) -> Block<L, N> {
        Block { pool: NonNull::new_unchecked(self as _), data: self.0.pop_unchecked() }
    }
}

pub struct Block<const L: usize, const N: usize> {
    pool: NonNull<StaticMemPool<L, N>>,
    data: NonNull<[u8; L]>
}

impl<const L: usize, const N: usize> AsRef<[u8; L]> for Block<L, N> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; L] {
        unsafe { self.data.as_ref() }
    }
}

impl<const L: usize, const N: usize> AsMut<[u8; L]> for Block<L, N> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8; L] {
        unsafe { self.data.as_mut() }
    }
}

impl<const L: usize, const N: usize> Drop for Block<L, N> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { self.pool.as_mut().0.push_unchecked(self.data) }
    }
}

#[cfg(test)]
mod tests {
    static mut POOL: crate::StaticMemPool<4096, 2> = crate::StaticMemPool::new();

    #[allow(unused_variables)]
    #[test]
    fn test() {
        unsafe {
            let block1 = POOL.borrow();
            assert!(matches!(Some(..), block1));
            let block2 = POOL.borrow();
            assert!(matches!(Some(..), block2));
            let block3 = POOL.borrow();
            assert!(matches!(None::<[u8; 4096]>, block3));
            drop(block1);
            let block3 = POOL.borrow();
            assert!(matches!(Some(..), block2));
        }
    }
}