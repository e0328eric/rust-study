fn main() {
    let x:u8 = 255;
    let y:u8 = 2;
    let z = x.overflowing_add(y).0;
    println!("{}", z);
}
