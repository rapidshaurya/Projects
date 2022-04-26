use std::process;
use std::fs;
use std::io;
use std::collections::HashMap;
pub struct Todo {
    // use rust built in HashMap to store key - val pairs
    pub map: HashMap<String, String>,
}

impl Todo {
    
    pub fn new(filename: &str) -> Result<Todo, std::io::Error> {
        // open db.json
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(filename)?;
    // serialize json as HashMap
    
    match serde_json::from_reader(f) {
        Ok(map) => Ok(Todo { map }),
        Err(e) if e.is_eof() => Ok(Todo {
            map: HashMap::new(),
        }),
        Err(e) => panic!("An error occurred: {}", e),
    }
    }

    pub fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value.
        self.map.insert(key.trim().to_string(), "☐".to_string());
    }

    pub fn save(self, filename: &str) -> Result<(), std::io::Error> {
        // open file
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;
    // write to file with serde
    serde_json::to_writer_pretty(f, &self.map)?;
    Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key.trim()) {
            Some(v) => Some(*v = "✅".to_string()),
            None => None,
        }
    }
    pub fn display(self,filename: &str) {
        let contents = fs::read_to_string(filename).expect("File Not exist");
         for _line in contents.lines() {
                println!("{}",_line);
         }
    }
}

pub fn reset() {
    let _x= fs::remove_file("data.txt");
    let _x = fs::remove_file("user1.json");
    let _x =fs::remove_file("user2.json");
    let _x =fs::remove_file("user3.json");
    let _x =fs::remove_file("user4.json");
    println!("Rest Completed");
}
pub fn welcome() -> String{

    println!("Welcome To To-do list");
    println!("(Press 1)\tNew User");
    println!("(Press 2)\tExisting User");
    
    let mut option = String::new();
         io::stdin().read_line(&mut option).expect("Invalid Option");
    let trimmed = option.trim();
    if !(trimmed =="1" || trimmed == "2") {
        println!("Invalid Option");
        process::exit(1);
    }
    return trimmed.to_string();
}