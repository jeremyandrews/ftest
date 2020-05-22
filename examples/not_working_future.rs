#[derive(Debug)]
struct MyStruct {
    a: usize,
    b: usize,
    c: String,
    // ...
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(example());
}

async fn my_function(my_struct: &mut MyStruct) -> () {
    println!("in my_function: {:?}", my_struct);
    // Change stuff.
    my_struct.a += 1;
    my_struct.b += 2;
    my_struct.c = "a different string".to_string();
}

struct StructWithFuture {
    // What should be stored here?
    stored_future: fn(&mut MyStruct),
    // ...
}

async fn example() {
    let mut my_struct = MyStruct {
        a: 1,
        b: 2,
        c: "a string".to_string(),
    };
    let struct_with_future = StructWithFuture {
        stored_future: my_function,
        // my_function,
        //                    ^^^^^^^^^^^ expected `()`, found opaque type
        // = note: expected fn pointer `for<'r> fn(&'r mut MyStruct)`
        //         found fn item `for<'_> fn(&mut MyStruct) -> impl std::future::Future {my_function}`
    };
    // Invoke the stored function.
    let function = struct_with_future.stored_future;
    // Await the future (but it's not currently defined as a future)
    function(&mut my_struct).await;
    // the trait bound `(): std::future::Future` is not satisfied

    // Confirm stuff changed.
    println!("after my_function: {:?}", my_struct);
}
