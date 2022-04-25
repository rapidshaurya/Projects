use todotest::{check, reset, Todo};
use std::process;
use std::fs;
use std::io;

fn main() {
    println!("Welcome To To-do list");
    let contents = fs::read_to_string("db.txt").expect("file to open user");
    
    let mut i=1; 
    for line in contents.lines() {
        println!("User {}\n {}\n Press {}:", i, line, i);
        i=i+1;
    }
    if i<=4 {
        println!("Press {} for new user",i);
    }
    
    let mut option = String::new();
         io::stdin().read_line(&mut option).expect("Invalid Option");
    let trimmed = option.trim();
    println!("{}", trimmed );
    if !(trimmed =="1" || trimmed == "2" || trimmed =="3" || trimmed == "4" ) {
        println!("Invalid Option");
        process::exit(1);
    }
    let mut filename="";
    if trimmed == "1" {
        filename="user1.json";
    }
    else if trimmed == "2" {
        filename="user2.json";
    } if trimmed == "3" {
        filename="user3.json";
    }
    else if trimmed == "4" {
        filename="user4.json";
    }


    println!("Enter Password");
    let mut password = String::new();
         io::stdin().read_line(&mut password).expect("Please specify an password");
    if password.trim() == ""{
        println!("Invalid Password");
        process::exit(1);
    }
    check(&password.trim(), &option);

    println!("Enter Action");
    let mut action = String::new();
         io::stdin().read_line(&mut action).expect("Please specify an action");
    println!("Enter Item");
    let mut item = String::new();
         io::stdin().read_line(&mut item).expect("Please specify an item");

   
    let mut todo = Todo::new(&filename).expect("Initialisation of db failed");
    
    if action.trim() == "add" {
        todo.insert(item);
        match todo.save(&filename) {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action.trim() == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save(&filename) {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action.trim() == "remove" {
        fs::remove_file(&filename)
      .expect("File delete failed");
       println!("File deleted successfully!");
    }


    println!("Do you want to change username or password");
    println!(" Press 1 for username
    \n Press 2 for password
    \nPress any other key for exit
    \nPress 9 for reset ALL");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to readline");
    let trimmed = guess.trim();
    if trimmed == "1"
        { let mut usernames = vec!["", "", "", ""];
        println!("Enter UserName: ");
        let mut user = String::new();
         io::stdin().read_line(&mut user).expect("failed to readline");
         let mut x=0;
         let contents = fs::read_to_string("db.txt").expect("file to open user");
         for line in contents.lines() {
             usernames[x] = line;
             x=x+1;
         }
         if option.trim() == "1" {
         usernames[0]=&user.trim(); }
         else if option.trim() == "2" {
            usernames[1]=&user.trim(); }
            else if option.trim() == "3" {
                usernames[2]=&user.trim(); }
                else if option.trim() == "4" {
                   usernames[3]=&user.trim(); }

         let mut user = String::new();
         user.push_str(usernames[0]);
         user.push_str("\n");
         user.push_str(usernames[1]);
         user.push_str("\n");
         user.push_str(usernames[2]);
         user.push_str("\n");
         user.push_str(usernames[3]);



         std::fs::write("db.txt",user).expect("Unable to write file");
         println!("New Username Saved");
        }
        else if trimmed == "2" {
         
            let mut password = vec!["", "", "", ""];
            println!("Enter Password: ");
            let mut user = String::new();
            io::stdin().read_line(&mut user).expect("failed to readline");
            let mut x=0;
            let contents = fs::read_to_string("password.txt").expect("file to open user");
            for line in contents.lines() {
             password[x] = line;
             x=x+1;
            }
            
            if option.trim() == "1" {
                password[0]=&user.trim(); }
                else if option.trim() == "2" {
                    password[1]=&user.trim(); }
                   else if option.trim() == "3" {
                    password[2]=&user.trim(); }
                       else if option.trim() == "4" {
                        password[3]=&user.trim(); }

             let mut pass = String::new();
             pass.push_str(password[0]);
             pass.push_str("\n");
             pass.push_str(password[1]);
             pass.push_str("\n");
             pass.push_str(password[2]);
             pass.push_str("\n");
             pass.push_str(password[3]);

             std::fs::write("password.txt",pass).expect("Unable to write file");
             println!("Password Saved");
        }
        else if trimmed == "9"{
            reset();
        }
     
}

