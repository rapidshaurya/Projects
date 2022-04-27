mod login;
use crate::login::Login;
use todotest::welcome;
use todotest::{reset, Todo};
use std::process;
use std::fs;
use std::io;


fn main() {
    let trimmed = welcome();
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


    println!("Enter Action  [add, display, complete]");
    let mut action = String::new();
         io::stdin().read_line(&mut action).expect("Please specify an action");
         let action = action.trim();
    
    let mut item = String::new();
    if action == "add" || action == "complete"{
         println!("Enter Item");
         io::stdin().read_line(&mut item).expect("Please specify an item");
    }
    let mut todo = Todo::new().expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(username,item);
        match todo.save() {
            Ok(_) => println!("File Saved...."),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(username,&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("File Saved...."),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "display" {
          todo.display(username);
      }
      else {
          println!("Invalid  Option!!");
          process::exit(1);
      }
    println!("[Press 1]\tReset to-do list for all user\n[Press Enter]\tExit");
    let mut res = String::new();
         io::stdin().read_line(&mut res).expect("Please specify an item");
    if res.trim() == "1" {
        reset();
    }
    
    //     else if trimmed == "9"{
    //         reset();
    //     }
     
}

