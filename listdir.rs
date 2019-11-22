use std::path::Path;

fn main() {
    let path = Path::new(".");

    for entry in path.read_dir().expect("Failed") {
        if let Ok(entry) = entry {
            let ent = entry.path();

            if Path::is_dir(&ent.as_path()) {
                println!("{}{:?}{}", "\x1b[01;36m", ent, "\x1b[0m");
            } else {
                println!("{:?}", ent);
            }
        }
    }
}
