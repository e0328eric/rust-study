use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use std::alloc::{alloc, realloc, dealloc, Layout};

fn oom() {
    std::process::exit(-9999);
}

pub struct RawVec<T> {
    pub ptr: NonNull<T>,
    pub cap: usize,
    _marker: PhantomData<T>,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        // !0 is usize::MAX. This branch should be stripped at compile time.
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };
        Self {
            ptr: NonNull::dangling(),
            cap,
            _marker: PhantomData,
        }
    }

    pub fn grow(&mut self) {
        // this is all pretty delicate, so let's say it's all unsafe
        unsafe {
            // current API requires us to specify size and alignment manually.
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            // since we set the capacity to usize::MAX when elem_size is 0,
            // getting to here necessarily means the MyVec is overflow.
            assert!(elem_size != 0, "capacity overflow");

            let layout = Layout::from_size_align(elem_size, align).expect("Error Occured!");
            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                // as an invariant, we can assume that 'self.cap < isize::MAX',
                // so this doesn't need to be checked.
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;

                // check that the new allocation doesn't exceed 'isize::MAX' at all
                // regardless of the actual size of the capacity. This combines the
                // 'new_cap <= isize::MAX' and 'new_num_bytes <= usize::MAX' checks
                // the address space with a single MyVec of i16's on 32-bit though.
                assert!(old_num_bytes <= (isize::MAX as usize) / 2, "capacity overflow");

                let new_num_bytes = old_num_bytes * 2;
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_num_bytes);
                (new_cap, ptr)
            };

            // If allocate or reallocate fail,, we'll get null back
            if ptr.is_null() { oom(); }
            self.ptr = NonNull::new(ptr as *mut _).expect("It must can make a NonNull pointer");
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            let align = mem::align_of::<T>();

            let layout = Layout::from_size_align(elem_size, align).expect("Error Occured!");
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
    }
}
