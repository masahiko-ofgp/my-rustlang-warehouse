fn hello() {
    println!("Hello, world!!");
}

fn hello2<'a>(name: &'a str) {
    println!("Hello, {}!!", name);
}

fn main() {
    hello();

    let tom = "Tom".to_string();
    hello2(&tom);
}
