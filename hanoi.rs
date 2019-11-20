fn hanoi(n: u8, a: char, b: char, c: char) {
    if n > 0 {
        hanoi(n - 1, a, c, b);
        println!("{} => {}", a, c);
        hanoi(n - 1, b, a, c);
    }
}

fn main() {
    hanoi(3_u8, 'A', 'B', 'C');
}
