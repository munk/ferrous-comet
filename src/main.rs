use std::fs::File;
use std::io::prelude::*;

fn parse_line(line: str) -> Vec<str> {
    return line.split(',').collect()
}

fn main() {
    println!("Asteroid data");

    let mut file = match File::open("/Users/john.downs/Downloads/results.csv") {
        Ok(f) => f,
        Err(_e) => {
            panic!("There was a problem open results.csv");
        },
    };

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("something went wrong reading the file");

    let mut lines = data.lines();
    let next_line = lines.next().unwrap();
    println!("{:?}", parse_line(next_line));
    //println!("{:?}", headers);
    //println!("Data:\n{}", data);

}
