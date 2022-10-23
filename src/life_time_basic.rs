pub fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_fn_owner_ship() {
    let a: i32 = 100;
    let b: i32 = 200;
    println!("{} {}", a, b);
    let c = add_two(a, b);
    println!("{} {} {}", a, b, c);
    let d = add_two(a, b);
    println!("{} {} {}", a, b, c);
}


fn longest(x: &str, y: &str) -> &str {
    if(x.len() > y.len()) {
        x
    } else {
        y
    }
}
/*
    String vs str
    A Rust String is like a std::string; it owns the memory and does the dirty job of managing memory.
    A Rust &str is like a char* (but a little more sophisticated); it points us to the beginning of a chunk in the same way you can get a pointer to the contents of std::string

 */
pub fn run_find_longest() {
    let string1 = String::from("abcdefghijk");
    let string2 = "xyz";
    println!("The longest string is {}", longest(string1.as_str(), string2));
}