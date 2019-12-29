const END: &'static str = "\x1b[0m";

#[allow(dead_code)]
enum Fg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
fn colorize<'a>(fg: Fg, text: &'a str) {
    match fg {
        Fg::Black => println!("{}{}{}", "\x1b[00;30m", text, END),
        Fg::Red => println!("{}{}{}", "\x1b[00;31m", text, END),
        Fg::Green => println!("{}{}{}", "\x1b[00;32m", text, END),
        Fg::Yellow => println!("{}{}{}", "\x1b[00;33m", text, END),
        Fg::Blue => println!("{}{}{}", "\x1b[00;34m", text, END),
        Fg::Magenta => println!("{}{}{}", "\x1b[00;35m", text, END),
        Fg::Cyan => println!("{}{}{}", "\x1b[00;36m", text, END),
        Fg::White => println!("{}{}{}", "\x1b[00;37m", text, END),
    }
}

fn main() {
    colorize(Fg::Red, "Hello");
}
