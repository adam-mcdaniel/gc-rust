use std::alloc::{alloc, dealloc, Layout};
use std::clone::Clone;

#[derive(Clone, Copy)]
struct Ptr<T> {
    layout: Layout,
    mem: Option<*mut T>
}

impl<T> Ptr<T> where T: Clone {
    pub fn new(t: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let mem = alloc(layout) as *mut T;
            *mem = t;
            Self {
                layout, mem: Some(mem)
            }
        }
    }

    pub fn set(&self, t: T) {
        unsafe {
            match self.mem {
                Some(mem) => *mem = t,
                None => {
                    panic!("TRIED TO SET DEALLOCATED MEM, EVERYTHING IS BROKEN");
                }
            }
        }
    }

    pub fn get(&self) -> T {
        unsafe {
            match self.mem {
                Some(mem) => (*mem).clone(),
                None => {
                    panic!("TRIED TO GET DEALLOCATED MEM, EVERYTHING IS BROKEN");
                }
            }
        }
    }

    pub fn dealloc(&self) {
        unsafe {
            match self.mem {
                Some(mem) => dealloc(mem as *mut u8, self.layout),
                None => {
                    panic!("DEALLOCATED MEM TWICE, EVERYTHING IS BROKEN");
                }
            }
        }
    }
}

// let layout = Layout::new::<u16>();


#[derive(Copy)]
struct Gc<'a, T> {
    data: &'a T,
    count: Ptr<usize>
}

impl<'a, T> Clone for Gc<'a, T> {
    fn clone(&self) -> Self {
        let count = self.count.get();
        self.count.set(count + 1);
        Self::new(self.data.clone())
    }
}

impl<'a, T> Gc<'a, T> {
    pub fn new(data: &'a T) -> Self {
        Self {
            data: data,
            count: Ptr::new(0 as usize)
        }
    }
}



fn main() {

}