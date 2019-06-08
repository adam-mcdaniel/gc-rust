mod ptr;
mod garbage;

// Call this at the end of the function, just before you return
// Call this on all values binded during the functions lifetime,
// Such as parameters for the function, and functions defined 
// with let or let mut in the function. Values must be mutable
// to collect.
#[macro_export]
macro_rules! collect {
    ($($garbage:ident),*) => (
        $($garbage.collect());*
    )
}
pub use ptr::check_heap;
pub use garbage::{Garbage};