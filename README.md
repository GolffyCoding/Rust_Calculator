# Rust Calculator

Rust Calculator เป็นแอปพลิเคชันเครื่องคิดเลขแบบกราฟิกที่สร้างขึ้นโดยใช้ `eframe` และ `egui` เพื่อให้มี UI ที่เรียบง่ายและสวยงาม พร้อมเอฟเฟกต์สไตล์นีออน

## คุณสมบัติ
- อินเทอร์เฟซที่ออกแบบอย่างเรียบง่ายด้วย `egui`
- รองรับการป้อนสมการทางคณิตศาสตร์ เช่น `sqrt(25) + 3`
- ปุ่มกดคำนวณพร้อมเอฟเฟกต์สไตล์นีออน
- แสดงผลลัพธ์ของสมการที่คำนวณได้

## วิธีติดตั้งและรันโปรแกรม

### ติดตั้ง Rust และ Cargo
หากคุณยังไม่มี Rust และ Cargo ติดตั้งบนเครื่องของคุณ สามารถติดตั้งได้โดยใช้คำสั่ง:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### โคลนโปรเจ็กต์
```sh
git clone https://github.com/username/rust-calculator.git
cd rust-calculator
```

### คอมไพล์และรันโปรแกรม
```sh
cargo run
```

## การใช้งาน
1. เปิดแอปพลิเคชัน
2. ป้อนสมการที่ต้องการคำนวณลงในช่องอินพุต
3. กดปุ่ม "✨ Calculate"
4. ผลลัพธ์จะแสดงที่ด้านล่างของแอป

## เทคโนโลยีที่ใช้
- **Rust** - ภาษาโปรแกรมหลัก
- **eframe/egui** - ใช้สำหรับการสร้าง GUI
- **meval** - ใช้สำหรับคำนวณค่าของสมการที่ผู้ใช้ป้อน

## ตัวอย่างโค้ดหลัก
```rust
fn eval_expr(expr: &str) -> String {
    let parsed = meval::eval_str(expr);
    match parsed {
        Ok(value) => format!("{:.6}", value),
        Err(_) => "❌ Error: Invalid expression".to_string(),
    }
}
```

![image](https://github.com/user-attachments/assets/d29ab6ab-bb4e-4152-bab3-2982fa57c358)
