struct MyStruct {
    a: i32,
    b: i32
}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {}, value b: {})", self.a, self.b)
    }
}
fn main(){
// Integer Type in Rust
let num : i32 = 5;
println!("The value of num is {}", num);
// Float Type in Rust
let num : f32 = 5.5;
println!("The value of num is {}", num);
// // Boolean Type in Rust
let num : bool = true;
println!("The value of num is {}", num);
// // Character Type in Rust
let num : char = 'a';
println!("The value of num is {}", num);
// // String Type in Rust
let num : String = "Hello World".to_string();
println!("The value of num is {}", num);
// // Array Type in Rust
let num : [i32; 5] = [1,2,3,4,5];
for i in 0..num.len() {
    println!("The value of num is {}", num[i]);
}
for elem in num.iter() {
    println!("The value of num is {}", elem);    
}
// println!("The value of num is {}", num);
// // Tuple Type in Rust
let num : (i32, f32, bool, char, String) = (1,2.5,true,'a',"Hello World".to_string());
println!("The value of num is {}", num.2);
// // Enum Type in Rust
// enum Color {
//     Red,
//     Green,
//     Blue
// }
// println!("The value of num is {}", num);
// // Struct Type in Rust

let test = MyStruct { a: 0, b: 0 };

    println!("Used Display: {}", test);  

}