use crate::ptr::Ptr;


// What's cool about garbage is that it
// allows closures that use this type to be copied,
// Even for Garbage types that are on the heap!
// Pretty convenient huh?
#[derive(Copy)]
pub struct Garbage<T> {
    data: Ptr<T>,
    count: Ptr<i32>
}

// This implements the reference counter for garbage
// When this object is cloned, it increments the counter!
// When the garbage is collected, and the counter is decremented,
// If the counter is 0, all data the garbage points to is freed.
impl<T> Clone for Garbage<T> {
    fn clone(&self) -> Self {
        let count = unsafe {
            *self.count.get()
        };

        self.count.set(count + 1);
        unsafe {
            self.weak_get()
        }
    }
}

// If our T implements clone,
// provide a safe way to access T as a value
impl<T> Garbage<T> where T: Clone {
    pub fn unwrap(&self) -> T {
        unsafe {
            let ptr: *mut T = self.decay();
            (*ptr).clone()
        }
    }
}

#[allow(dead_code)]
impl<T> Garbage<T> {
    // Create a new instance of garbage instantiated with T
    pub fn new(data: T) -> Self {
        Self {
            data: Ptr::new(data),
            count: Ptr::new(1)
        }
    }

    // Get the pointer to the T stored in Garbage
    pub fn decay(&self) -> *mut T {
        self.data.get()
    }

    // Create a copy of self without incrementing counter.
    // This is very unsafe, and should only be done internally.
    unsafe fn weak_get(&self) -> Self {
        Self {
            data: Ptr::from(self.data.get()),
            count: self.count.clone()
        }
    }

    // Get a copy of this Garbage and increment the counter.
    // This copy will also need to be collected.
    pub fn get(&self) -> Self {
        self.clone()
    }

    // Set the value of the data stored in garbage
    pub fn set(&mut self, t: T) {
        self.data.set(t);
    }

    // Get the reference count
    pub fn count(&self) -> i32 {
        unsafe {
            *self.count.get()
        }
    }

    // Collect the garbage.
    pub fn collect(&mut self) {
        let count = unsafe {
            *self.count.get()
        };

        if count > 0 {
            self.count.set(count - 1);
        } else {
            panic!("TRIED TO COLLECT DEALLOC'D GARBAGE, EVERYTHINGS BROKEN");
        }


        let new_count = unsafe {
            *self.count.get()
        };

        if new_count == 0 {
            self.count.dealloc();
            self.data.dealloc();
            println!("DEALLOC'D GARBAGE");
        }
    }
}