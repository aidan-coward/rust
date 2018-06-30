use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    
    Command::new("sudo")
            .arg("pacman")
            .arg("-Syy");
    
    let to_update = Command::new("pacman")
                            .arg("-Qqu")
                            .output()
                            .expect("failed to gather list available package updates");
                           
    println!("to_update is {:?}", to_update.stdout);
}

