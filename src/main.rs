extern crate rand;
extern crate json;
extern crate serde_json;
extern crate serde;

// use serde::{Deserialize, Serialize};
use serde::{Serialize, Deserialize};
// use serde_json::{Value, json};
use rand::Rng;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize, Debug)]
struct LangRNG {
    language_taken: Vec<String>,
}

fn main(){
    let history = Path::new("history.json");

    let mut language_rng = LangRNG{
        language_taken: vec![]
    };
    
    // history.json exist
    if history.exists() {
        let json_string = read_json(&history);

        let json_string = match json_string { 
            Ok(file) => {
                println!("File Found! Reading from history.json");
                file
            },

            Err(error) => {
                panic!("Error: {}", error);
            },
        };

        let val : LangRNG = serde_json::from_str(&json_string).unwrap();
        let new_language = generate_language();
        let is_taken = match val.language_taken.iter().position(|r| r == &new_language){
            None => -1,
            Some(val) => {
                val
            },
        };

        // let is_taken = match is_taken {
        //     None => println!("Error"),
        //     Some(val) => {
        //         val
        //     },
        // };
        println!("{:?}", is_taken);
    }
    else {
        match create_json(&mut language_rng, &history){
            Ok(_) => println!("File Created"),
            Err(error) => panic!("Error: {}", error),
        };
    }
}

fn read_json(history : &Path) -> Result< String , Box<dyn Error>> {
    let file_stream = File::open(history)?;
    let mut buffer_reader = BufReader::new(file_stream);
    let mut buffer = String::new();
    buffer_reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn create_json(json_object : &mut LangRNG, history : &Path) -> Result<(), Box<dyn Error>>{
    json_object.language_taken.push(generate_language());
    let json_stringify = serde_json::to_string(json_object).unwrap();
    let mut buffer = File::create(history)?;
    buffer.write_all(json_stringify.as_bytes())?;
    Ok(())
}

fn generate_language() -> String {
    let mut rng = rand::thread_rng();
    let array = [
        "C#".to_string(),
        "Golang".to_string(),
        "Java".to_string(),
        "Javascript".to_string(),
        "Kotlin".to_string(),
        "Python".to_string(),
        "Rustlang".to_string(),
    ]; 
    let index = rng.gen_range(0,array.len());
    return String::from(&array[index]);
}
