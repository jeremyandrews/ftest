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
    //stored_future: fn(&mut MyStruct) -> Pin<Box<dyn Future<Output = ()>>>
    stored_future: for<'r> fn(&'r mut MyStruct) -> Pin<Box<dyn Future<Output = ()> + 'r>>
    // ...
}

async fn my_function<'r>(my_struct: &'r mut MyStruct) {
    println!("in my_function: {:?}", my_struct);
    // Change stuff.
    my_struct.a += 1;
    my_struct.b += 2;
    my_struct.c = "a different string".to_string();
}
fn my_function_async<'r>(my_struct: &'r mut MyStruct) -> Pin<Box<dyn Future<Output = ()> + 'r>> {
    Box::pin(my_function(my_struct))
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
    function(&mut my_struct).await;
    // Confirm stuff changed.
    println!("after my_function: {:?}", my_struct);
}