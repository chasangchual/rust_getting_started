pub fn conv_i32_to_i8() {
    let a: i32 = 300;
    let b = a as i8;
    println!("{:b} {:x} {:?}", a, a, a);
    println!("{:b} {:x} {:?}", b, b, b);
}