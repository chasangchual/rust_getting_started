pub fn greet_to_world() {
    println!("Hello World !!\n\n");
    let greet_in_korean = "안녕하세요, 반갑습니다.";
    let greet_in_english = "Hello, good to see you";

    let greets = [greet_in_korean, greet_in_english];
    for greet in greets.iter() {
        println!("{}", &greet)
    }
}