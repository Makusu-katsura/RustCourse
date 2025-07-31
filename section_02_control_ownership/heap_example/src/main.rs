// ตัวอย่างโค้ด: การทำงานบน Heap เพิ่มเติม
fn main() {
    // สร้างข้อมูลบน Heap
    // b1 คือ Box<i32> ซึ่งเป็น pointer และถูกเก็บไว้บน Stack
    // ส่วนค่า `5` จริงๆ ถูกเก็บไว้ใน Heap
    let b1 = Box::new(5);

    println!("b1 = {}", b1);

    // ลองสร้างข้อมูลที่ซับซ้อนขึ้นมา
    let game = create_game();

    // ตัวแปร game (struct Game) อยู่บน Stack
    // แต่ field ที่ชื่อ `name` (String) มีข้อมูลจริงๆ ("Minecraft") อยู่บน Heap
    println!("Playing {} with score {}", game.name, game.score);

} // เมื่อจบ main, game จะถูก drop -> String ภายในจะคืนหน่วยความจำบน Heap
  // b1 ก็จะถูก drop -> คืนหน่วยความจำของเลข 5 บน Heap

struct Game {
    name: String, // String จัดการข้อมูลบน Heap
    score: u32,   // u32 อยู่บน Stack (เป็นส่วนหนึ่งของ struct)
}

fn create_game() -> Game {
    let game_name = String::from("Minecraft"); // สร้าง String บน Heap
    Game {
        name: game_name,
        score: 100,
    }
}