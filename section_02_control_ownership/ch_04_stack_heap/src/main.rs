fn main() {
    let s1 = String::from("Hello"); // String ถูกเก็บไว้ใน Heap
    let s2 = String::from("World"); // String ถูกเก็บไว้ใน Heap

    println!("The original strings are: {}, {}", s1, s2); // s1 และ s2 ยังคงอยู่บน Stack
    
    // Calling the concatenate function
    let result = concatenate(s1, s2); // ค่า s1 และ s2 ถูก "move" ไปให้ฟังก์ชัน concatenate

    println!("The concatenated string is: {}", result); // result เป็น String ใหม่ที่ถูกสร้างขึ้น

    // s1 และ s2 ถูกย้ายไปยังฟังก์ชัน concatenate แล้ว ดังนั้นไม่สามารถใช้งานได้อีก
    // println!("s1: {}, s2: {}", s1, s2); // จะเกิดข้อผิดพลาดที่นี่
    // แต่ข้อมูลใน Heap ยังคงอยู่จนกว่าจะไม่มีตัวแปรใด
    // อ้างอิงถึงมันอีกต่อไป
} // ตัวแปร result ถูกลบออกจาก Stack แต่ข้อมูลใน Heap ยังคงอยู่

fn concatenate(a: String, b: String) -> String {
    let result = format!("{} {}", a, b); // a, b, และ result อยู่บน Stack ของฟังก์ชัน concatenate
    result // คืนค่า result ซึ่งเป็น String ใหม่
} // ตัวแปร result, b, a ถูกลบออกจาก Stack