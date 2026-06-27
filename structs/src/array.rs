use std::{
    alloc::{GlobalAlloc, Layout},
    ops::{Index, IndexMut},
    ptr::{NonNull, null_mut},
};

#[derive(Default, Hash)]
pub struct Array<T, Allocator: GlobalAlloc = std::alloc::System> {
    alloca: Allocator,
    len: usize,
    capacity: usize,
    data: *mut T,
    layout: Option<Layout>,
}

fn exp_of_2(mut val: usize) -> bool {
    if val == 0 {
        true
    } else if val % 2 == 1 {
        false
    } else {
        while val != 1 {
            if val % 2 == 1 {
                return false;
            }
            val /= 2;
        }
        true
    }
}

impl<T> Array<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            alloca: std::alloc::System,
            len: 0,
            capacity: 0,
            data: null_mut(),
            layout: None,
        }
    }
    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity >= 4, "Capacity must be >= 4");
        assert!(exp_of_2(capacity), "Capacity must be exponent of 2");
        Self {
            alloca: std::alloc::System,
            len: 0,
            capacity,
            data: null_mut(),
            layout: None,
        }
    }

    #[allow(unused)]
    pub fn push(&mut self, value: T) {
        let prev = self.layout;
        let mut layout_upd = false;
        if self.len == 0 {
            self.capacity = 4;
            self.layout = Some(Layout::array::<T>(4).unwrap());
            layout_upd = true;
        } else if self.len == self.capacity {
            self.capacity *= 2;
            self.layout = Some(Layout::array::<T>(self.capacity).unwrap());
            layout_upd = true;
        }
        unsafe {
            if layout_upd {
                let data = self.alloca.alloc_zeroed(self.layout.unwrap()) as *mut T;
                data.copy_from(self.data, self.len);
                if !self.data.is_null() {
                    self.alloca.dealloc(self.data as *mut u8, prev.unwrap());
                }
                self.data = data;
            }
            self.data.offset(self.len as isize).write(value);
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
        if self.capacity == 4 {
            unsafe {
                let size = std::mem::size_of::<T>();
                let ptr = self.data.offset(self.len as isize - 1) as *mut u8;
                for i in 0..size {
                    ptr.offset(i as isize).write(0);
                }
            }
        } else {
            if self.len - 1 == self.capacity / 2 {
                self.capacity /= 2;
                self.layout = Some(Layout::array::<T>(self.capacity).unwrap());
            }
            unsafe {
                let data = self.alloca.alloc_zeroed(self.layout.unwrap()) as *mut T;
                data.copy_from(self.data, self.len - 1);
                if !self.data.is_null() {
                    self.alloca.dealloc(self.data as *mut u8, prev.unwrap());
                }
                self.data = data;
                self.len -= 1;
            }
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

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.data.is_null() {
            panic!("Array is empty");
        } else if self.len <= index {
            panic!(
                "Index out of array range, array len: {}, index: {}",
                self.len, index
            );
        }
        unsafe { self.data.offset(index as isize).as_mut().unwrap() }
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
            layout: None,
        }
    }
    #[allow(unused)]
    pub fn with_capacity_in(alloca: A, capacity: usize) -> Self {
        assert!(capacity >= 4, "Capacity must be >= 4");
        assert!(exp_of_2(capacity), "Capacity must be exponent of 2");
        Self {
            alloca,
            len: 0,
            capacity,
            data: null_mut(),
            layout: None,
        }
    }
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        let alloca = self.alloca.clone();
        let layout = self.layout;
        let data = unsafe {
            if layout.is_some() {
                alloca.alloc(layout.unwrap()) as *mut T
            } else {
                null_mut()
            }
        };
        if layout.is_some() {
            unsafe {
                data.copy_from(self.data, self.len);
            }
        }
        Self {
            alloca,
            layout,
            data,
            len: self.len,
            capacity: self.capacity,
        }
    }
}

pub struct IterArray<T, A: GlobalAlloc = std::alloc::System> {
    pub alloc: A,
    pub ptr: Option<NonNull<T>>,
    pub len: usize,
    pub pos: usize,
}

impl<T> Iterator for IterArray<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.len || self.ptr.is_none() {
            if self.ptr.is_some() {
                unsafe {
                    self.alloc.dealloc(
                        self.ptr.unwrap().as_ptr() as *mut u8,
                        Layout::array::<T>(self.len).unwrap(),
                    );
                    self.ptr = None;
                };
            }
            None
        } else {
            Some(unsafe {
                let out = self.ptr.unwrap().offset(self.pos as isize).read();
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
        let ptr = if self.layout.is_some() {
            let data = unsafe { self.alloca.clone().alloc(self.layout.unwrap()) as *mut T };
            unsafe {
                data.copy_from(self.data, self.len);
            }
            NonNull::new(data)
        } else {
            None
        };
        Self::IntoIter {
            alloc: self.alloca,
            ptr: ptr,
            len: self.len,
            pos: 0,
        }
    }
}

impl<T> FromIterator<T> for Array<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = Self::new();
        for i in iter {
            out.push(i);
        }
        out
    }
}

impl<T, A: GlobalAlloc> Drop for Array<T, A> {
    fn drop(&mut self) {
        if self.layout.is_some() {
            unsafe {
                self.alloca
                    .dealloc(self.data as *mut u8, self.layout.unwrap());
            }
        }
    }
}

impl<T> Extend<T> for Array<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.push(i);
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Array<T> {
    fn from(value: [T; N]) -> Self {
        let mut ve = Self::new();
        for i in value {
            ve.push(i);
        }
        ve
    }
}
impl<T: Clone, const N: usize> From<&[T; N]> for Array<T> {
    fn from(value: &[T; N]) -> Self {
        let mut ve = Self::new();
        for i in value {
            ve.push(i.clone());
        }
        ve
    }
}
