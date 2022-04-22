use std::collections::HashMap;
pub struct Todo {
    // use rust built in HashMap to store key - val pairs
    pub map: HashMap<String, bool>,
}

impl Todo {
    
    pub fn new() -> Result<Todo, std::io::Error> {
        // open db.json
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?;
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
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        // open db.json
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("db.json")?;
    // write to file with serde
    serde_json::to_writer_pretty(f, &self.map)?;
    Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}