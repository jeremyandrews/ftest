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

async fn my_function<'r>(my_struct: &'r mut MyStruct) {
    my_struct.a += 1;
    println!("in my_function: {:?}", my_struct);
}

struct StructWithFuture<F>
where
    F: std::future::Future,
{
    stored_future: fn(&mut MyStruct) -> F,
    // ...
}
 
impl<F> StructWithFuture<F>
where
    F: std::future::Future,
{
    async fn do_thing(self) {
        let mut my_struct = MyStruct {
            a: 1,
            b: 2,
            c: "a string".to_string(),
        };
        (self.stored_future)(&mut my_struct).await;
    }
}

async fn example() {
    let s = StructWithFuture {
        // expected concrete lifetime, found bound lifetime parameter
        //
        // = note: expected fn pointer `for<'r> fn(&'r mut MyStruct) -> _`
        //         found fn item `for<'r> fn(&'r mut MyStruct) -> impl std::future::Future {my_function}`
        stored_future: my_function,
    };
    s.do_thing().await;
}
