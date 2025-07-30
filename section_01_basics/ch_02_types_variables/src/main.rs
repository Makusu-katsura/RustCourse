fn main() {
    println!("-------------------------");

    /*
    * Statically Typed และการอนุมานชนิดข้อมูล (Type Inference)
    */
    // เราสามารถประกาศชนิดข้อมูลอย่างชัดเจนได้
    // `i32` คือ signed integer ขนาด 32 บิต
    let a: i32 = -100;
    //a = 100.5; // จะเกิดข้อผิดพลาด เพราะ `a` ถูกประกาศเป็น i32

    // หรือจะปล่อยให้คอมไพเลอร์อนุมานชนิดข้อมูลให้ก็ได้
    // Rust จะอนุมานว่า `b` เป็น i32 (ชนิดเริ่มต้นสำหรับ integer)
    let b = -200;
    // b = 200.5; // จะเกิดข้อผิดพลาด เพราะ `b` ถูกอนุมานเป็น i32

    println!("integer with explicit type: {}", a);
    println!("integer with inferred type: {}", b);

    // Floating-point (จำนวนทศนิยม)
    // f64 คือ float ขนาด 64 บิต มีความแม่นยำ 15-17 หลัก
    let temp: f64 = 3.1498998998998798799899898999;
    // f32 คือ float ขนาด 32 บิต มีความแม่นยำ 6-9 หลัก
    let presure: f32 = 10.983939398933;

    println!("floating-point number: {}", temp);
    println!("floating-point number: {}", presure);

    /*
    * Signed และ Unsigned Integers
    */
    // Signed integer สามารถติดลบได้
    let balance: i32 = -500;

    println!("Signed integer: {}", balance);

    // Unsigned integer เก็บได้แค่ค่าบวกและศูนย์
    // u8 สามารถเก็บค่าได้ตั้งแต่ 0 ถึง 255
    let age: u8 = 0;

    println!("Unsigned integer: {}", age);
    
    /*
    * ชนิดข้อมูลและขนาดในหน่วยความจำ (Bits)
    */
    // i8 ใช้ 8 บิต สามารถเก็บค่าได้ตั้งแต่ -128 ถึง 127
    let small_signed_number: i8 = 127;

    // u16 ใช้ 16 บิต สามารถเก็บค่าได้ตั้งแต่ 0 ถึง 65,535
    let medium_unsigned_number: u16 = 65_535;

    println!("Small signed number: {}", small_signed_number);
    println!("Medium unsigned number: {}", medium_unsigned_number);

    println!("-------------------------");

    /*
    * Boolean Type
    */
    // Boolean type มีแค่สองค่า: true และ false
    let is_active: bool = true;
    let is_logged_in: bool = false;
    println!("Is active: {}", is_active);
    println!("Is logged in: {}", is_logged_in);

    /*
    * Character Type
    */
    // Character type ใช้สำหรับเก็บตัวอักษร Unicode ขนาด 4 ไบต์
    // สามารถเก็บตัวอักษรพิเศษหรือสัญลักษณ์ได้
    let letter: char = 'R';
    let emoji: char = '😊';
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);

    println!("-------------------------");

    /*
    * Character Type String
    */
    // String type ใช้สำหรับเก็บข้อความหรือชุดอักขระ
    let greeting: String = "Hello, world!".to_string();
    println!("Greeting: {}", greeting);

    /*
    * ตัวแปร (Variables)
    */
    // ประกาศตัวแปรชื่อ 'x' และกำหนดค่าให้เป็น 5
    let x = 5;
    println!("The value of x is: {}", x);

    /*
    * Interpolation (การแทรกค่าในสตริง)
    */

    let name = "Alice";
    let age = 30;

    // แทรกค่าของตัวแปร 'name' และ 'age' ลงในสตริง
    println!("My name is {} and I am {} years old.", name, age);
    // {0} จะอ้างอิงถึง "Alice" (อาร์กิวเมนต์ตัวแรก)
    // {1} จะอ้างอิงถึง "Bob" (อาร์กิวเมนต์ตัวที่สอง)
    println!("{0} is a friend of {1}. And {1} is also a friend of {0}.", "Alice", "Bob");


   // 'mutable_var' สามารถเปลี่ยนแปลงค่าได้เพราะมี 'mut'
    let mut mutable_var = 15;
    println!("Original value: {}", mutable_var);

    mutable_var = 25; // เปลี่ยนค่าได้
    mutable_var += 5; // เพิ่มค่าอีก 5
    mutable_var *= 2; // คูณด้วย 2
    mutable_var -= 10; // ลบ 10
    mutable_var /= 5; // หารด้วย 5

    println!("Mutable variable value: {}", mutable_var);

    /*
    * Constants (ค่าคงที่)
    */
    // ประกาศค่าคงที่ ต้องระบุชนิดข้อมูล (type annotation)
    const MAX_POINTS: u32 = 100_000;
    const MIN_POINTS: u32 = 0;

    // ค่าคงที่ไม่สามารถเปลี่ยนแปลงได้
    // MAX_POINTS = 200_000; // บรรทัดนี้จะทำให้เกิด error ตอนคอมไพล์
    println!("The maximum points are: {}", MAX_POINTS);
    println!("The minimum points are: {}", MIN_POINTS);

    /*
    * Variable Shadowing (การบดบังตัวแปร)
    */
    let x = 5;
    println!("The value of x is: {}", x); // แสดงผล 5

    // 'x' ตัวใหม่นี้จะ "บดบัง" (shadow) 'x' ตัวเดิม
    // เราสามารถเปลี่ยนชนิดข้อมูลได้ด้วย
    let x = "hello"; 
    println!("The value of x is now: {}", x); // แสดงผล "hello"

    /*
    * Scopes and Blocks (ขอบเขตและบล็อก)
    */

    let outer_var = "I'm outside";

    { // เริ่ม scope ใหม่
        let inner_var = "I'm inside";
        println!("{}", outer_var); // เข้าถึงตัวแปรภายนอกได้
        println!("{}", inner_var); // เข้าถึงตัวแปรภายในได้
    } // จบ scope

    println!("{}", outer_var);
    // println!("{}", inner_var); // บรรทัดนี้จะ error เพราะ 'inner_var' อยู่นอก scope แล้ว
    println!("-------------------------");
    // unused_function();
     // การจัดการข้อผิดพลาด
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
}

/*
* Compiler Directives like allow (แอตทริบิวต์ของคอมไพเลอร์)
*/
// #[allow(dead_code)] จะบอกคอมไพเลอร์ให้เมินเฉยต่อโค้ดที่ไม่ได้ถูกเรียกใช้
// #[allow(unused_variables)] จะบอกให้เมินเฉยต่อตัวแปรที่ประกาศแต่ไม่ได้ใช้
#[allow(dead_code)]
#[allow(unused_variables)]
fn unused_function() {
    #[allow(unused_variables)]
    let unused_var = 42; // ตัวแปรที่ไม่ได้ใช้งาน
    println!("This function is not used, but Rust won't complain.");
}

/*
* Rust Error Codes (การจัดการข้อผิดพลาด)
*/
// Rust เน้นการจัดการข้อผิดพลาดผ่าน Result<T, E> และ Option<T> enum แทนการใช้ error code แบบดั้งเดิม
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        // คืนค่า Error
        Err("Cannot divide by zero!")
    } else {
        // คืนค่าที่สำเร็จ
        Ok(numerator / denominator)
    }
}