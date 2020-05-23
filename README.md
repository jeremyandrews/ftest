This repo is to ask some questions about how to store
async futures to invoke later, for use with Goose:
   https://github.com/tag1consulting/goose

Currently Goose is not async, and instead stores function
pointers.

The definition is quite simple:
https://github.com/tag1consulting/goose/blob/master/src/goose.rs#L1240

And invoking the stored functions is simple as well:
https://github.com/tag1consulting/goose/blob/master/src/client.rs#L45


I've been trying to cleanly rework this to support async, without
making the creation of Goose load tests more complex.

The currently working code-base is simplified into the
`working_future` example (the original working PR against Goose
can be found at https://github.com/tag1consulting/goose/pull/13):
```
$ cargo run --example working_future
in my_function: MyStruct { a: 1, b: 2, c: "a string" }
after my_function: MyStruct { a: 2, b: 4, c: "a different string" }
```

This requires the use of a proof-of-concept crate called
`macro_rules_attribute` which I'd like to avoid. It also makes
the declaration of individual task functions a little more
complicated, though not terrible:
```
#[macro_rules_attribute(dyn_async!)]
async fn my_function<'fut>(my_struct: &'fut mut MyStruct) -> () {
```

Is there a simpler way to do this?
Can it be done without the `macro_rules_attribute` crate?

The simplest form of what I'm currently doing in Goose can be
seen in the `working_function` example:
```
$ cargo run --example working_function
in my_function: MyStruct { a: 1, b: 2, c: "a string" }
after my_function: MyStruct { a: 2, b: 4, c: "a different string" }
```

A working async version that does not include a mutable can
be seen in the `working_nonmutable` example (see the top of the
example code for a link to where this came from):
```
$ cargo run --example working_nonmutable
in my_function: MyStruct { a: 1, b: 2, c: "a string" }
```

A failed attempt to convert from a nonmutable to a mutable can
be seen in the `not_working_mutable` example:

```
$ cargo run --example not_working_mutable
 error[E0308]: mismatched types
   --> examples/not_working_mutable.rs:47:24
    |
 47 |         stored_future: my_function,
    |                        ^^^^^^^^^^^ expected concrete lifetime, found bound lifetime parameter
    |
    = note: expected fn pointer `for<'r> fn(&'r mut MyStruct) -> _`
                  found fn item `for<'r> fn(&'r mut MyStruct) -> impl std::future::Future {my_function}`
```

A failed attempt to convert the Goose way of doing it to async
can be seen in the `not_working_future` example:
```
 $ cargo run --example not_working_future
   error[E0308]: mismatched types
     --> examples/not_working_future.rs:35:24
      |
   35 |         stored_future: my_function,
      |                        ^^^^^^^^^^^ expected `()`, found opaque type
      |
      = note: expected fn pointer `for<'r> fn(&'r mut MyStruct)`
                    found fn item `for<'_> fn(&mut MyStruct) -> impl std::future::Future {my_function}`
   
   error[E0277]: the trait bound `(): std::future::Future` is not satisfied
     --> examples/not_working_future.rs:44:5
      |
   44 |     function(&mut my_struct).await;
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
```

I've also tried various ways to Pin and/or Box the future, but
have not managed to get it working.

---

Update (May 22):

Thanks to feedback in https://www.reddit.com/r/rust/comments/goh2be/how_to_store_an_async_future_to_a_function_with_a/ I was able to get this working.

```
$ cargo run --example working_box
in my_function: MyStruct { a: 1, b: 2, c: "a string" }
after my_function: MyStruct { a: 2, b: 4, c: "a different string" }
```
