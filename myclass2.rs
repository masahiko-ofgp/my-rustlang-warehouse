trait Device {
    fn new() -> Self
        where Self: Sized;

    fn set_software<'a>(&mut self, cd: &'a str);

    fn play(&self);

    fn stop(&self);
}

#[derive(Debug)]
struct CDPlayer { cd: Option<String> }

impl Device for CDPlayer {
    fn new() -> Self {
        CDPlayer { cd: None }
    }
    fn set_software<'a>(&mut self, cd: &'a str) {
        self.cd = Some(cd.to_string());
    }
    fn play(&self) {
        println!("CD: {:?}", &self.cd);
    }
    fn stop(&self) {
        println!("CDPlayer stopped.");
    }
}

impl CDPlayer {
    fn shuffle(&mut self) {
        println!("Shuffle");
    }
}

#[derive(Debug)]
struct DVDPlayer { dvd: Option<String> }

impl Device for DVDPlayer {
    fn new() -> Self {
        DVDPlayer { dvd: None }
    }
    fn set_software<'a>(&mut self, dvd: &'a str) {
        self.dvd = Some(dvd.to_string());
    }
    fn play(&self) {
        println!("DVD: {:?}", &self.dvd);
    }
    fn stop(&self) {
        println!("DVDPlayer stopped.");
    }
}

impl DVDPlayer {
    fn chapter_menu(&mut self) {
        println!("Display chapter menu.");
    }
}

fn main() {
    let mut cdp = CDPlayer::new();
    let mut dvdp = DVDPlayer::new();

    println!("{:?}", &cdp.cd);
    println!("{:?}", &dvdp.dvd);

    cdp.set_software("Buena Vista Social Club");
    cdp.play();
    cdp.shuffle();
    cdp.stop();

    dvdp.set_software("Forest Gump");
    dvdp.play();
    dvdp.chapter_menu();
    dvdp.stop();
}


