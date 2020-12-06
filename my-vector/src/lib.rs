mod iterator;
mod raw_vec;

use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use iterator::{IntoIter, RawValIter, Drain};
use raw_vec::RawVec;

pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> MyVec<T> {
    fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }
    fn cap(&self) -> usize { self.buf.cap }

    pub fn new() -> Self {
        Self { buf: RawVec::new(), len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() { self.buf.grow(); }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            unsafe {
                Some(ptr::read(self.ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // NOTE: '<=' because it's valid to insert after everything
        // which would be equivalent to push.
        assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len { self.buf.grow(); }

        unsafe {
            if index < self.len {
                // ptr::copy(src, dest, len): "copy from source to dest len elems"
                ptr::copy(self.ptr().add(index),
                          self.ptr().add(index + 1),
                          self.len - index);
            }
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        // NOTE: '<' because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
                ptr::copy(self.ptr().add(index + 1),
                          self.ptr().add(index),
                          self.len - index);
            result
        }
    }
}

impl<T> MyVec<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);

            IntoIter {
                iter,
                _buf: buf,
            }
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawValIter::new(&self);
            self.len = 0;

            Drain {
                iter,
                vec: PhantomData,
            }
        }
    }

}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_vec_works() {
        let mut lst: MyVec<i32> = MyVec::new();
        lst.push(1);
        lst.push(3);
        lst.push(5);
        let to_real_vec = lst.into_iter().collect::<Vec<_>>();
        assert_eq!(vec![1,3,5], to_real_vec);
    }
}
