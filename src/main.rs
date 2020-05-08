extern crate rand;
extern crate json;
extern crate serde_json;
extern crate serde;

use serde::{Serialize, Deserialize};
use rand::Rng;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct LangRNG {
    language_taken: Vec<String>,
}

fn main(){
    let history = Path::new("history.json");

    // Define vector
    let mut language_array = vec![
        "C#".to_string(),
        "Golang".to_string(),
        "Java".to_string(),
        "Javascript".to_string(),
        "Kotlin".to_string(),
        "Python".to_string(),
        "Rustlang".to_string(),
    ];

    let mut json_stringify;
    
    // history.json exist
    if history.exists() {
        let json_string = read_json(&history);
        
        // match need all possibilities to return the same type
        let json_string = match json_string { 
            Ok(file) => {
                println!("File Found! Reading from history.json");
                file
            },

            Err(error) => {
                panic!("Error: {}", error);
            },
        };

        // Parse JSON to Object
        let mut val : LangRNG = serde_json::from_str(&json_string).unwrap();

        filter_array(&val, &mut language_array);
        
        // If all language are in history.json, then start fresh
        if language_array.len() == 0{
            val.language_taken = vec![];
            language_array = vec![
                "C#".to_string(),
                "Golang".to_string(),
                "Java".to_string(),
                "Javascript".to_string(),
                "Kotlin".to_string(),
                "Python".to_string(),
                "Rustlang".to_string(),
            ];
        }

        let new_language = generate_language(&mut language_array);
        println!("Language of the day is: {}", new_language);
        val.language_taken.push(new_language);

        // JSON Stringify
        json_stringify = serde_json::to_string(&mut val).unwrap();
    }
    else {
        // Instantiate Object
        let mut language_rng = LangRNG{
            language_taken: vec![]
        };

        // Since there is no such file, RNG a new language without filtering
        language_rng.language_taken.push(generate_language(&mut language_array));
        json_stringify = serde_json::to_string(&mut language_rng).unwrap();
    }

    match write_json(&mut json_stringify, &history){
        Ok(_) => println!("File Created"),
        Err(error) => panic!("Error: {}", error),
    };
}

// Functiom sigature with return type Result can use ? as exception short hand
fn write_json(json_stringify : &mut String , history : &Path) -> Result<(), Box<dyn Error>>{
    let mut buffer = File::create(history)?;
    
    // Write using byte stream
    buffer.write_all(json_stringify.as_bytes())?;
    Ok(())
}

// Borrow (use reference) can leave variable ownership at parent scope
fn read_json(history : &Path) -> Result< String , Box<dyn Error>> {
    let file_stream = File::open(history)?;
    
    // More efficient Reader Buffer
    let mut buffer_reader = BufReader::new(file_stream);
    let mut buffer = String::new();
    buffer_reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn filter_array(json_object : &LangRNG , language_array : &mut Vec<String>){
    // Loop with iterator
    for item in json_object.language_taken.iter(){
        // Find index of given item, then remove that item
        language_array.remove(language_array.iter().position(|r| r == item).unwrap());
    }
}

fn generate_language(language_array : &mut Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0,language_array.len());
    return String::from(&language_array[index]);
}
