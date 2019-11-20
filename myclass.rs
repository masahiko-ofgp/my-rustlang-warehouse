trait Name {
    fn get_name(&self) -> String;
}
trait Email {
    fn get_email(&self) -> String;
}

#[derive(Debug)]
struct Person {
    name: String,
    email: String,
}

impl Name for Person {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl Email for Person {
    fn get_email(&self) -> String {
        self.email.to_string()
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Name for Dog {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

fn main() {
    let tom = Person{
        name: "Tom".to_string(),
        email: "tom@tom.com".to_string(),
    };
    let hachi = Dog{ name: "Hachi".to_string() };

    println!("{}", tom.get_name());
    println!("{}", tom.get_email());
    println!("{}", hachi.get_name());
}
