use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::mem;
use std::os::unix::prelude::FileExt;
use std::{error::Error, fs::File};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    steps: u32,
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
    bson_file_example()?;
    bson_vec_example()?;
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

fn bson_file_example() -> Result<(), Box<dyn Error>> {
    let mut f = File::create("db.bson")?;

    for i in 1..=1000 {
        let a = Move {
            steps: i,
            dir: Direction::Left,
        };
        let mut a_bson = bson::to_vec(&a)?;
        let data_len = a_bson.len() as u32;
        let mut data_frame =
            Vec::<u8>::with_capacity(mem::size_of_val(&data_len) + data_len as usize);
        data_frame = data_len.to_be_bytes().to_vec();
        data_frame.append(&mut a_bson);
        f.write_all(&data_frame)?;
    }

    f.sync_all()?;

    let mut f = File::open("db.bson")?;
    let mut cursor = 0;
    let mut moves = Vec::<Move>::with_capacity(1000);
    loop {
        // read the data length from first 4 bytes
        let mut data_len_buf = [0u8; mem::size_of::<u32>()];
        let mut num_bytes_read = f.read_at(&mut data_len_buf, cursor)?;
        if num_bytes_read == 0 {
            break;
        }
        cursor += mem::size_of::<u32>() as u64;

        // read data
        let data_len = u32::from_be_bytes(data_len_buf);
        let mut data_buf = vec![0; data_len as usize];
        let num_bytes_read = f.read_at(&mut data_buf, cursor)?;
        if num_bytes_read == 0 {
            break;
        }
        cursor += data_len as u64;

        // reconstruct data
        let a: Move = bson::from_slice(&data_buf.to_vec())?;
        moves.push(a);
    }

    println!("moves.len() = {}", &moves.len());
    println!("moves[..10] = {:?}", &moves[..10]);

    Ok(())
}

fn bson_vec_example() -> Result<(), Box<dyn Error>> {
    let mut moves_buf = Vec::<u8>::new();
    for i in 1..=1000 {
        let a = Move {
            steps: i,
            dir: Direction::Left,
        };
        let mut a_bson = bson::to_vec(&a)?;
        let data_len = a_bson.len() as u32;
        let mut data_frame =
            Vec::<u8>::with_capacity(mem::size_of_val(&data_len) + data_len as usize);
        data_frame = data_len.to_be_bytes().to_vec();
        data_frame.append(&mut a_bson);
        moves_buf.append(&mut data_frame);
    }

    println!(
        "BSON VEC SERIALIZATION Moves[..10] = {:?}",
        &moves_buf[..10]
    );

    let mut cursor = 0;
    let mut moves = Vec::<Move>::with_capacity(1000);
    while cursor < moves_buf.len() {
        // read the data length from first 4 bytes
        let data_len_buf = &moves_buf[cursor..(cursor + mem::size_of::<u32>())];
        let data_len = u32::from_be_bytes(data_len_buf.try_into()?);
        cursor += mem::size_of::<u32>();
        let data = &moves_buf[cursor..(cursor + data_len as usize)];
        cursor += data_len as usize;
        let a: Move = bson::from_slice(&data.to_vec())?;
        moves.push(a);
    }

    println!("BSON VEC DESERIALIZATION moves.len() = {}", &moves.len());
    println!("BSON VEC DESERIALIZATION moves[..10] = {:?}", &moves[..10]);

    Ok(())
}
