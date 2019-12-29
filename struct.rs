use std::default::Default;

#[derive(Debug)]
struct Person {
    name: String,
    email: String,
}
impl Default for Person {
    fn default() -> Self {
        Person {
            name: "anonymous".to_string(),
            email: "xxxxx@xxx.com".to_string(),
        }
    }
}
impl Person {
    fn new<'a>(name: &'a str, email: &'a str) -> Self {
        Person {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

fn main() {
    let p = Person::default();
    assert_eq!(&p.name, "anonymous");
    assert_eq!(&p.email, "xxxxx@xxx.com");

    let tom = Person::new("Tom", "tom@tom.com");
    assert_eq!(&tom.name, "Tom");
    assert_eq!(&tom.email, "tom@tom.com");
}
