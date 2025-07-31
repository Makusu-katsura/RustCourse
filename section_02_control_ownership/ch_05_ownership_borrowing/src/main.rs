// fn main() {
//     let s1 = String::from("Hello"); // String ถูกเก็บไว้ใน Heap
//     // let s2 = s1;

//     // println!("{}",s2)

//    takes_ownership(s1); // s1 ถูกย้ายเข้าสู่ฟังก์ชัน takes_ownership
//                         // ตอนนี้ s1 ไม่สามารถใช้งานได้แล้ว

//     // println!("{}", s1); // จะเกิดข้อผิดพลาด: s1 ไม่มีความเป็นเจ้าของอีกต่อไป

//     let x = 5;

//     makes_copy(x); // x ถูกส่งเข้าไปในฟังก์ชัน makes_copy
//                    // x ยังคงสามารถใช้งานได้หลังจากฟังก์ชันนี้จบ

//     println!("x is still accessible: {}", x); // x ยังคงสามารถใช้งานได้


// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // some_string หลุดออกจาก scope, หน่วยความจำถูกคืน

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// } 
// fn main() {
//     let s = String::from("Hello, Rust!"); // s เป็นเจ้าของ String บน Heap

//     let len = calculate_length(&s); // ส่ง s เป็น reference ให้กับฟังก์ชัน

//     print!("Length of '{}' is: {}", s, len); // s ยังคงสามารถใช้งานได้
// } 

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// 2. การยืมแบบเปลี่ยนแปลง (Mutable Borrowing
fn main() {

    let mut s = String::from("hello");

    change(&mut s); // ยืม s แบบ mutable

    println!("{}", s); // s ถูกแก้ไขแล้ว
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} // some_string หลุดออกจาก scope
