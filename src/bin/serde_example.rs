use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{error::Error, fs::File};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    steps: u8,
    dir: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    Left,
    Up,
    Right,
    Bottom,
}

fn main() -> Result<(), Box<dyn Error>> {
    json_example()?;
    ron_example()?;
    Ok(())
}

fn json_example() -> Result<(), Box<dyn Error>> {
    let a = Move {
        steps: 4,
        dir: Direction::Right,
    };

    // serialize to json string
    let ser_a = serde_json::to_vec(&a)?;
    println!("Serialized: {:?}", ser_a);

    // write serialized data to file
    let mut f = File::create("move.json")?;
    f.write_all(&ser_a)?;
    f.sync_all()?;
    println!("file {:?}", f);

    let mut f = File::open("move.json")?;
    let mut f_contents = vec![];
    f.read_to_end(&mut f_contents)?;
    println!("contents = {:?}", f_contents);

    let deser_a: Move = serde_json::from_slice(&f_contents)?;
    println!("Deserialized: {:?}", deser_a);

    Ok(())
}

fn ron_example() -> Result<(), Box<dyn Error>> {
    let a = Move {
        steps: 4,
        dir: Direction::Right,
    };

    // serialize to json string
    let ser_a = ron::to_string(&a)?;
    println!("Serialized: {:?}", ser_a);

    let de_str: Move = ron::from_str(&ser_a)?;
    println!("Deserialized: {:?}", de_str);

    Ok(())
}
