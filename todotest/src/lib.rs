use std::process;
use std::fs;
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
}

pub fn check (password: &str, i: &str) {
    let contents = fs::read_to_string("password.txt").expect("file to open user");
    let mut f=0;
    let mut count=1; 
    for line in contents.lines() {
        if i.trim() == count.to_string().trim() {
        if line.contains(&password) {
            f=1;
        }
        
        }
        count=count+1;
    }
    if f ==0 {
        println!("Invalid Password");
        process::exit(1);
    }

}
pub fn reset() {
    
    let mut _password = vec!["0000", "0000", "0000", "0000"];
    let mut _usernames = vec!["Admin1"];
    let mut pass = String::new();
    pass.push_str(_password[0]);
             pass.push_str("\n");
             pass.push_str(_password[1]);
             pass.push_str("\n");
             pass.push_str(_password[2]);
             pass.push_str("\n");
             pass.push_str(_password[3]);
    std::fs::write("password.txt",pass).expect("Unable to write file");


    let mut user = String::new();
         user.push_str(_usernames[0]);
         std::fs::write("db.txt",user).expect("Unable to write file");

    let _x = fs::remove_file("user1.json");
    let _x =fs::remove_file("user2.json");
    let _x =fs::remove_file("user3.json");
    let _x =fs::remove_file("user4.json");
    let _x =println!("Rest Completed");
}