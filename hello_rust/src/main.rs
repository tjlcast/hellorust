use foo::{Another, my_struct::MyStruct};

mod foo;

fn main() {
 let _ms = MyStruct{};
 let _ano = Another{};

 let _my_struct = MyStruct::new();
 let _another = Another::new();

 my_print();

 println!("The sum of {} {} is: {}", 1, 2, add_sum(1, 2));
}

fn my_print() {
    println!("Hello World");
}

fn add_sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
