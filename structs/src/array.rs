use std::alloc::{GlobalAlloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{ops::Index, ptr::null_mut};

#[derive(Clone)]
pub struct Array<T, Allocator: GlobalAlloc = std::alloc::System> {
    alloca: Allocator,
    len: usize,
    capacity: usize,
    data: *mut T,
    layout: Layout,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self {
            alloca: std::alloc::System,
            len: 0,
            capacity: 0,
            data: null_mut(),
            layout: Layout::new::<T>(),
        }
    }
    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            alloca: std::alloc::System,
            len: 0,
            capacity,
            data: null_mut(),
            layout: Layout::new::<T>(),
        }
    }

    #[allow(unused)]
    pub fn push(&mut self, value: T) {
        let prev = self.layout;
        if self.len == 0 {
            self.capacity = 4;
            self.layout = Layout::array::<T>(4).unwrap();
        } else if self.len == self.capacity {
            self.capacity *= 2;
            self.layout = Layout::array::<T>(self.capacity).unwrap();
        }
        unsafe {
            let data = self.alloca.alloc_zeroed(self.layout) as *mut T;
            data.copy_from(self.data, self.len);
            data.offset(self.len as isize).write(value);
            if !self.data.is_null() {
                self.alloca.dealloc(self.data as *mut u8, prev);
            }
            self.data = data;
            self.len += 1;
        }
    }
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.len
    }
    #[allow(unused)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[allow(unused)]
    pub fn pop(&mut self) -> T
    where
        T: Copy,
    {
        if self.len == 0 {
            panic!("Array is empty");
        }
        let prev = self.layout;
        let el = self[self.len - 1];
        if self.len - 1 == self.capacity / 2 {
            self.capacity /= 2;
            self.layout = Layout::array::<T>(self.capacity).unwrap();
        }
        unsafe {
            let data = self.alloca.alloc_zeroed(self.layout) as *mut T;
            data.copy_from(self.data, self.len - 1);
            if !self.data.is_null() {
                self.alloca.dealloc(self.data as *mut u8, prev);
            }
            self.data = data;
            self.len -= 1;
        }
        el
    }

    #[allow(unused)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.data.is_null() {
            panic!("Array is empty");
        } else if self.len <= index {
            panic!(
                "Index out of array range, array len: {}, index: {}",
                self.len, index
            );
        }
        unsafe { Some(self.data.offset(index as isize).as_ref().unwrap()) }
    }

    #[allow(unused)]
    pub fn first(&self) -> Option<T>
    where
        T: Copy,
    {
        if self.len == 0 { None } else { Some(self[0]) }
    }
    #[allow(unused)]
    pub fn last(&self) -> Option<T>
    where
        T: Copy,
    {
        if self.len == 0 {
            None
        } else {
            Some(self[self.len - 1])
        }
    }
}

pub struct IterArray<T> {
    pub phantom: PhantomData<T>,
    pub ptr: NonNull<T>,
    pub len: usize,
    pub pos: usize,
}

impl<T> Iterator for IterArray<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.len {
            None
        } else {
            Some(unsafe {
                let out = self.ptr.offset(self.pos as isize).read();
                self.pos += 1;
                out
            })
        }
    }
}

impl<T> IntoIterator for Array<T>
where
    T: Copy + Clone + std::fmt::Display,
{
    type Item = T;
    type IntoIter = IterArray<T>;
    fn into_iter(self) -> Self::IntoIter {
        let phantom = PhantomData::<T>::default();
        let ptr = NonNull::new(self.data).unwrap();
        Self::IntoIter {
            phantom,
            ptr,
            len: self.len,
            pos: 0,
        }
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if self.data.is_null() {
            panic!("Array is empty");
        } else if self.len <= index {
            panic!(
                "Index out of array range, array len: {}, index: {}",
                self.len, index
            );
        }
        unsafe { self.data.offset(index as isize).as_ref().unwrap() }
    }
}

impl<T, A: GlobalAlloc> Array<T, A> {
    #[allow(unused)]
    pub fn new_in(alloca: A) -> Self {
        Self {
            alloca,
            len: 0,
            capacity: 0,
            data: null_mut(),
            layout: Layout::new::<T>(),
        }
    }
    #[allow(unused)]
    pub fn with_capacity_in(alloca: A, capacity: usize) -> Self {
        Self {
            alloca,
            len: 0,
            capacity,
            data: null_mut(),
            layout: Layout::new::<T>(),
        }
    }
}
