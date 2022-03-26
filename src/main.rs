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
            field2: vec![String::from("something")],
            field3: Box::new(64),
        };
    }
}

fn main() {
    println!("{:?}", TestStruct::get_field_attribute_map());
    println!("{:?}", TestStruct::get_field_attribute("field1"));
    println!("{:?}", TestStruct::get_field_attribute("field3"));

    let test_struct: TestStruct = TestStruct::new();
    println!("{:?}", test_struct::get_field("field1"));
    println!("{:?}", test_struct::get_field_typed::<Vec<String>>("field2"))
}
