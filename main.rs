extern crate colored;

use std::io;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
use std::fs;

fn printhelp() {
    println!(" ");
    println!("Commands:");
    println!("Help - Displays this text.");
    println!("New - Creates a new person, requires: name, phone number, and address. If one of these values is unknown, put in 'Unknown'.");
    println!("View - Allows you to view a person by inputting the name.");
    println!("Delete - Removes a person from the database by inputting the name.");
    println!("List - Lists all people saved to the database.");
    println!("Clearlist - Deletes all saved people, requires confirmation");
}

fn handlenew() {
    println!(" ");

    // VARIABLE DECLARATION
    let mut nam = String::new();
    let mut pho = String::new();
    let mut add = String::new();

    // INFO SET
    println!("Input name: ");
    io::stdin().read_line(&mut nam).expect("Error while reading line");
    println!("Input phone number: ");
    io::stdin().read_line(&mut pho).expect("Error while reading line");
    println!("Input address: ");
    io::stdin().read_line(&mut add).expect("Error while reading line");

    // FILE SET
    let mut pathinit = nam.trim_end().to_string(); // trim end, set back to string
    pathinit.push_str(".txt"); // append .txt to end of string
    let path = &*pathinit; // change back into &str

    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {panic!("Error while creating file : {:?}", error);}
    };

    write!(output, "Name: {}", nam.clone()).expect("Error while writing name");
    write!(output, "Phone: {}", pho.clone()).expect("Error while writing phone");
    write!(output, "Address: {}", add.clone()).expect("Error while writing address");

    println!("\nAdded {}", nam);
}

fn handleview() {
    println!(" ");

    let mut name = String::new();
    println!("Input name for search: ");
    io::stdin().read_line(&mut name).expect("Error while reading name input");

    let mut pathinit = name.trim_end().to_string(); // trim end, set back to string
    pathinit.push_str(".txt"); // append .txt to end of string
    let path = &*pathinit; // change back into &str

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    println!(" ");
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}

fn handleremove() {
    println!(" ");

    let mut name = String::new();
    println!("Input name for deletion: ");
    io::stdin().read_line(&mut name).expect("Error while reading name input");

    let mut pathinit = name.trim_end().to_string(); // trim end, set back to string
    pathinit.push_str(".txt"); // append .txt to end of string
    let path = &*pathinit; // change back into &str

    fs::remove_file(path).expect("Error while removing file");

    println!("\nRemoved {}", name);
}

fn handlelist() {
    println!(" ");

    println!("List of people: ");
    println!("----------------------");
    for file in fs::read_dir("").unwrap() {
        let file = file.unwrap().path();
        if file.extension().unwrap().to_str().unwrap() == "txt" {
            println!("{}", file.file_name().unwrap().to_str().unwrap().trim_end_matches(".txt"));
        }
    }
}

fn handleclearlist() {
    println!(" ");

    let mut uip = String::new();
    println!("Are you sure? Y/N:");
    io::stdin().read_line(&mut uip).expect("Error while reading confirmation");
    let uip = uip.trim_end();

    fn clearall() {
        for file in fs::read_dir("").unwrap() {
            let file = file.unwrap().path();
            if file.extension().unwrap().to_str().unwrap() == "txt" {
                fs::remove_file(file.as_path()).expect("Error while removing file");
            }
        }
    }

    if uip.to_ascii_lowercase() == "y" {
        clearall();
        println!("\nCleared all data");
    } else {
        println!("Canceled request to clear");
    }
}


fn main() {
    println!("Simple Database - Marktime\n-----------------");

    // MAIN LOOP
    loop {
        println!(" ");
        let mut uip = String::new();
        println!("Input a command, type help for help: ");
        io::stdin().read_line(&mut uip).expect("Error while reading line");

        let help = "help".to_string();
        let new = "new".to_string();
        let view = "view".to_string();
        let remove = "delete".to_string();
        let list = "list".to_string();
        let clearlist = "clearlist".to_string();

        if uip.trim_end().to_ascii_lowercase() == help {
            printhelp();
        } else if uip.trim_end().to_ascii_lowercase() == new {
            handlenew();
        } else if uip.trim_end().to_ascii_lowercase() == view {
            handleview();
        } else if uip.trim_end().to_ascii_lowercase() == remove {
            handleremove();            
        } else if uip.trim_end().to_ascii_lowercase() == list {
            handlelist();
        } else if uip.trim_end().to_ascii_lowercase() == clearlist{
            handleclearlist();
        } else {
            println!("Command entered not valid");
        }
    }
}
