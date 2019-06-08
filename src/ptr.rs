use std::alloc::{alloc, dealloc, Layout};
use std::clone::Clone;
use std::fmt::{Display, Error, Formatter};

// This keeps track of how many unfreed objects are on the heap
static mut HEAP_UNFREED: i32 = 0;

// This object abstracts pointers, and keeps track of objects
// Allocated and freed from the the heap using this object.
// Detects whether or not its been freed, and panics if it has
// and is being read.
// It is not necessarily better than a raw pointer, just more convenient
#[derive(Clone, Copy)]
pub struct Ptr<T> {
    layout: Layout,
    mem: *mut T,
    is_freed: bool
}


// Create a Ptr from a raw pointer
impl<T> From<*mut T> for Ptr<T> {
    fn from(mem: *mut T) -> Self {
        let layout = Layout::new::<T>();
        Self {
            layout, mem,
            is_freed: false
        }
    }
}

impl<T> Ptr<T> {
    // Create a new pointer to a T and instantiate it with value t
    pub fn new(t: T) -> Self {
        unsafe {
            // Increment the number of pointers on the heap
            HEAP_UNFREED += 1;
            // Get the layout of T for alloc
            // and allocate memory on the heap
            let layout = Layout::new::<T>();
            let mem = alloc(layout) as *mut T;
            // Initialize mem
            *mem = t;

            // Create object
            Self {
                layout, mem: mem,
                is_freed: false
            }
        }
    }

    // Is the memory safe to read/write, meaning has it not been freed
    pub fn is_safe(&self) -> bool {
        !self.is_freed
    }

    // If memory is safe to read/write, set it to value t, else panic
    pub fn set(&self, t: T) {
        unsafe {
            if self.is_safe() {
                *self.mem = t
            } else {
                panic!("TRIED TO SET DEALLOCATED MEM, EVERYTHING IS BROKEN");
            }
        }
    }

    // If memory is safe to read/write, get the pointer to our T, else panic
    // This is done instead of returning T to make Ptr always clonable
    pub fn get(&self) -> *mut T {
        if self.is_safe() {
            // Clone the value of our pointer and return
            self.mem.clone()
        } else {
            panic!("TRIED TO GET DEALLOCATED MEM, EVERYTHING IS BROKEN");
        }
    }

    // If memory is safe to read/write, dealloc, else panic
    pub fn dealloc(&mut self) {
        unsafe {
            HEAP_UNFREED -= 1;

            if self.is_safe() {
                // Cast to u8 pointer for dealloc
                dealloc(self.mem as *mut u8, self.layout);
                // Confirm we've freed memory
                self.is_freed = true;
            } else {
                panic!("DEALLOCATED MEM TWICE, EVERYTHING IS BROKEN");
            }
        }
    }
}



// Boring kind of debugging stuff
impl<T: Display> Display for Ptr<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unsafe {
            write!(f, "Ptr to '{}'", *self.get())
        }
    }
}


// Checks the status of heap at the end of
// main to tell you how many leaks you had
pub fn check_heap() {
    unsafe {
        if HEAP_UNFREED == 0 {
            println!("All blocks freed, no leaks!");
        } else {
            panic!("{} Ptr objects were not freed!", HEAP_UNFREED/2);
        }
    }
}