use std::convert::TryInto;

pub fn type_case_with_try_into() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("{:?} is less than {:?}", a, b_);
    }
}