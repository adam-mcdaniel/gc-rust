use std::fmt::Display;
use gc_rust::{Garbage, collect, check_heap};


fn println<T>(mut a: Garbage<T>) where T: Display {
    unsafe {
        println!("{}", *a.decay());
    }
    collect!(a);
}


fn add(mut a: Garbage<i32>, mut b: Garbage<i32>) -> Garbage<i32> {
    let result = a.unwrap() + b.unwrap();
    collect!(a, b);
    return Garbage::new(result);
}


fn main() {
    // let mut a = Garbage::new(27);
    // let mut b = Garbage::new(5);
    // print!("a: ");
    // println(a.get());
    // print!("b: ");
    // println(b.get());

    // let mut c = add(a.get(), b.get());
    // print!("c: ");
    // println(c.get());
    // // unsafe {
    // //     println(a.weak_get());
    // // }
    // collect!(a, b, c);
    fn inc(mut i: Garbage<i32>) -> Garbage<i32> {
        let result = i.unwrap() + 1;
        collect!(i);
        return Garbage::new(result);
    }

    fn dec(mut i: Garbage<i32>) -> Garbage<i32> {
        let result = i.unwrap() - 1;
        collect!(i);
        return Garbage::new(result);
    }


    let mut inc_gc: Garbage<fn(Garbage<i32>) -> Garbage<i32>> = Garbage::new(inc);
    let mut dec_gc: Garbage<fn(Garbage<i32>) -> Garbage<i32>> = Garbage::new(dec);
    // let mut incinc: Garbage<Arc<dyn Fn(Garbage<i32>) -> Garbage<i32>>>;



    let mut a = Garbage::new(2);
    let mut b = Garbage::new(5);

    println((inc_gc.unwrap())( a.get() ));
    // println((inc_gc.unwrap())(a.get()));
    println(
        add(
            a.get(), b.get()
        )
    );

    collect!(a, b, inc_gc, dec_gc);

    check_heap();
}