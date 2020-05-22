#[macro_use]
extern crate macro_rules_attribute;

use std::{pin::Pin, future::Future};

#[derive(Debug)]
struct MyStruct {
    a: usize,
    b: usize,
    c: String,
    // ...
}

type AsyncTaskCallbackFunction =
    fn (&'_ mut MyStruct) ->
        Pin<Box<dyn // owned trait object
            Future<Output = ()> // future API / pollable
            + Send // required by non-single-threaded executors
            + '_ // may capture `my_struct`, which is only valid for the `'_` lifetime
        >>
;

#[macro_export]
macro_rules! dyn_async {(
    $( #[$attr:meta] )* // includes doc strings
    $pub:vis
    async
    fn $fname:ident<$lt:lifetime> ( $($args:tt)* ) $(-> $Ret:ty)?
    {
        $($body:tt)*
    }
) => (
    $( #[$attr] )*
    #[allow(unused_parens)]
    $pub
    fn $fname<$lt> ( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
        dyn ::std::future::Future<Output = ($($Ret)?)>
            + ::std::marker::Send + $lt
    >>
    {
        ::std::boxed::Box::pin(async move { $($body)* })
    }
)}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(example());
}

#[macro_rules_attribute(dyn_async!)]
async fn my_function<'fut>(my_struct: &'fut mut MyStruct) -> () {
    println!("in my_function: {:?}", my_struct);
    // Change stuff.
    my_struct.a += 1;
    my_struct.b += 2;
    my_struct.c = "a different string".to_string();
}

struct StructWithFuture {
    stored_future: AsyncTaskCallbackFunction,
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
