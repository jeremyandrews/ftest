use std::boxed::Box;
use std::pin::Pin;
use std::future::Future;

#[derive(Debug)]
struct MyStruct {
    a: usize,
    b: usize,
    c: String,
    // ...
}

struct StructWithFuture {
    stored_future: fn(&mut MyStruct) -> Pin<Box<dyn Future<Output = ()>>>
    // ...
}

async fn my_function<'r>(my_struct: &'r mut MyStruct) {
    println!("in my_function: {:?}", my_struct);
    // Change stuff.
    my_struct.a += 1;
    my_struct.b += 2;
    my_struct.c = "a different string".to_string();
}
fn my_function_async<'r>(my_struct: &'r mut MyStruct) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(my_function(my_struct))
    // 26 |     Box::pin(my_function(my_struct))
    //    |              ^^^^^^^^^^^^^^^^^^^^^^
    //    |
    // note: first, the lifetime cannot outlive the lifetime `'r` as defined on the function body at 25:22...
    //   --> examples/not_working_box.rs:25:22
    //    |
    // 25 | fn my_function_async<'r>(my_struct: &'r mut MyStruct) -> Pin<Box<dyn Future<Output = ()> + 'static>> {
    //    |                      ^^
    // note: ...so that reference does not outlive borrowed content
    //   --> examples/not_working_box.rs:26:26
    //    |
    // 26 |     Box::pin(my_function(my_struct))
    //    |                          ^^^^^^^^^
    //    = note: but, the lifetime must be valid for the static lifetime...
    // note: ...so that the expression is assignable
    //   --> examples/not_working_box.rs:26:5
    //    |
    // 26 |     Box::pin(my_function(my_struct))
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    = note: expected  `std::pin::Pin<std::boxed::Box<(dyn std::future::Future<Output = ()> + 'static)>>`
    //               found  `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = ()>>>`
}


fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(example());
}

async fn example() {
    let struct_with_function = StructWithFuture {
        stored_future: my_function_async,
    };
    let mut my_struct = MyStruct {
        a: 1,
        b: 2,
        c: "a string".to_string(),
    };
    // Invoke the stored function.
    let function = struct_with_function.stored_future;
    function(&mut my_struct);
    // Confirm stuff changed.
    println!("after my_function: {:?}", my_struct);
}