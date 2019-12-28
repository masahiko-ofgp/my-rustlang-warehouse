trait Device {
    // Default method
    fn play<'a>(software: &'a str) {
        println!("PLAY: {}", software);
    }
    // Default method
    fn stop() {
        println!("STOP");
    }
}

#[derive(Debug)]
struct CD;
impl Device for CD {
    // Override
    fn stop() {
        println!("CD STOP");
    }
}

#[derive(Debug)]
struct DVD;
impl Device for DVD {
    // No override
}

fn main() {
    CD::play("Buena Vista Social Club");
    CD::stop();

    DVD::play("Forrest Gump");
    DVD::stop();
}
