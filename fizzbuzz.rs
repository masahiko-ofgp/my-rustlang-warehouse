// Using if stmt
fn fizzbuzz(n: i32) {
    if (n % 15) == 0 {
        println!("FizzBuzz");
    } else if (n % 5) == 0 {
        println!("Buzz");
    } else if (n % 3) == 0 {
        println!("Fizz");
    } else {
        println!("{}", n);
    }
}

// Using match exp
fn fizzbuzz2(n: i32) {
    match ((n % 3), (n % 5)) {
        (0, 0) => { println!("FizzBuzz"); },
        (_, 0) => { println!("Buzz"); },
        (0, _) => { println!("Fizz"); },
        _ => { println!("{}", n); }
    }
}

fn main() {
    for i in 0 ..= 30 {
        fizzbuzz2(i);
    }
}
