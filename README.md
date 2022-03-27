# Rust-Struct-Reification
A macro to reify structs and their fields for type-safe runtime invocation and reflective access to fields and attributes

## Example:

```rust
use crate::reify;

reify!{
    struct TestStruct {
        #[some_attr=5]
        pub field1: u64,
        #[macro_attr(Value)]
        field2: Vec<String>,
        field3: Box<u16>,
    }
}

impl TestStruct {
    pub fn new() -> TestStruct {
        return TestStruct {
            field1: 42,
            field2: vec![String::from("something"), String::from("else")],
            field3: Box::new(64),
        };
    }
}

fn main() {
    println!("{:?}", TestStruct::get_field_attribute_map());
    // Prints: { "field1": "some_attr=5", "field2": "macro_attr(Value)", "field3": "" }
    println!("{:?}", TestStruct::get_field_attribute("field1"));
    // Prints: Ok(Some("some_attr=5"))
    println!("{:?}", TestStruct::get_field_attribute("field3"));
    // Prints: Ok(None)
    println!("{:?}", TestStruct::get_field_attribute("field4"));
    // Prints: Err(TypedAttributeRetrievalError{ message: "..." })

    let test_struct: TestStruct = TestStruct::new();
    println!("{:?}", test_struct::get_field("field1"));
    // Prints: Ok(Any { .. })
    println!("{:?}", test_struct::get_field_typed::<Vec<String>>("field2"))
    // Prints: Ok({ "something", "else" })
}

```