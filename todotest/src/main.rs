mod login;
use crate::login::Login;
use todotest::welcome;
use todotest::{reset, Todo};
use std::process;
use std::fs;
use std::io;


fn main() {
    let trimmed = welcome();
    let mut _fileno = 0;
    if trimmed.trim() == "1" {
         let mut logres = Login::new().expect("Failed to create new user");
        println!("Enter New Username");
        let mut username =String::new();
        io::stdin()
                .read_line(&mut username)
                .expect("Invalid Option");
        println!("Enter New Password");
        let mut password =String::new();
        io::stdin()
                .read_line(&mut password)
                .expect("Invalid Option");
        if password.trim() == "" {
            println!("Invalid Password");
        }
         logres.insert(username.trim().to_string(), password.trim().to_string());
    }
        println!("Enter Username and Password");
        let mut username = String::new();
        let mut password = String::new();
        io::stdin()
                .read_line(&mut username)
                .expect("Invalid Option");
        io::stdin()
                .read_line(&mut password)
                .expect("Invalid Option");
        let check = format!("{}\t{}", username.trim(), password.trim());

        let contents = fs::read_to_string("data.txt").expect("file to open user");
         let mut f=0;
         let mut count=1; 
         for line in contents.lines() {
                if line.contains(&check.trim()) {
                    f=1;
                    break;
                }
                count=count+1;
         }
        if f ==0 {
        println!("Invalid Password");
        process::exit(1);
                 }
        _fileno=count;
    let mut filename="";
    if _fileno == 1 {
        filename="user1.json";
    }
    else if _fileno == 2 {
        filename="user2.json";
    }else if _fileno == 3 {
        filename="user3.json";
    }
    else if _fileno == 4 {
        filename="user4.json";
    }


    println!("Enter Action  [add, display, remove, complete]");
    let mut action = String::new();
         io::stdin().read_line(&mut action).expect("Please specify an action");
         let action = action.trim();
    
    let mut item = String::new();
    if action == "add" || action == "complete"{
         println!("Enter Item");
         io::stdin().read_line(&mut item).expect("Please specify an item");
    }
    let mut todo = Todo::new(&filename).expect("Initialisation of db failed");
    
    if action == "add" {
        todo.insert(item);
        match todo.save(&filename) {
            Ok(_) => println!("File Saved...."),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save(&filename) {
                Ok(_) => println!("File Saved...."),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "remove" {
        fs::remove_file(&filename)
      .expect("File delete failed");
       println!("File deleted successfully!");
    }
      else if action == "display" {
          todo.display(&filename);
      }
    println!("[Press 1]\tReset to-do list for all user\n[Press exit]\tExit");
    let mut res = String::new();
         io::stdin().read_line(&mut res).expect("Please specify an item");
    if res.trim() == "1" {
        reset();
    }
    
    //     else if trimmed == "9"{
    //         reset();
    //     }
     
}

