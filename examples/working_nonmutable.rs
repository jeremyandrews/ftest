// This is based on this page:
// https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance?noredirect=1

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

async fn my_function(my_struct: MyStruct) {
    println!("in my_function: {:?}", my_struct);
}

struct StructWithFuture<F>
where
    F: std::future::Future,
{
    stored_future: fn(MyStruct) -> F,
    // ...
}
 
impl<F> StructWithFuture<F>
where
    F: std::future::Future,
{
    async fn do_thing(self) {
        let my_struct = MyStruct {
            a: 1,
            b: 2,
            c: "a string".to_string(),
        };
        (self.stored_future)(my_struct).await;
    }
}

async fn example() {
    let s = StructWithFuture {
         stored_future: my_function,
    };
    s.do_thing().await;
}
