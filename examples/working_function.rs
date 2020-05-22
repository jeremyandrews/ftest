#[derive(Debug)]
struct MyStruct {
    a: usize,
    b: usize,
    c: String,
    // ...
}

fn main() {
    example();
}

fn my_function(my_struct: &mut MyStruct) {
    println!("in my_function: {:?}", my_struct);
    // Change stuff.
    my_struct.a += 1;
    my_struct.b += 2;
    my_struct.c = "a different string".to_string();
}

struct StructWithFunction {
    stored_function: fn(&mut MyStruct),
    // ...
}

fn example() {
    let struct_with_function = StructWithFunction {
        stored_function: my_function,
    };
    let mut my_struct = MyStruct {
        a: 1,
        b: 2,
        c: "a string".to_string(),
    };
    // Invoke the stored function.
    let function = struct_with_function.stored_function;
    function(&mut my_struct);
    // Confirm stuff changed.
    println!("after my_function: {:?}", my_struct);
}
