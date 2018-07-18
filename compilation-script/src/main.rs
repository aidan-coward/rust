extern crate glob;

use glob::glob;
use std::process::Command;
use std::process;
use std::path::Path;

fn main() {
    
    let to_update = Command::new("pacman")
                            .arg("-Qqu")
                            .output()
                            .expect("failed to gather list available package updates");
                           
    let to_update = String::from_utf8(to_update.stdout).expect("to_update.stdout is not a String");

    let mut to_update_vec: Vec<String> = Vec::new();

    //println!("to_update is {:?}", to_update);

    if to_update == "" {
        println!("nothing to compile");
        process::exit(0);
    } else {
        for line in to_update.lines() {
            //println!("{}", line);
            to_update_vec.push(line.to_string());
        }
    }
    
    let all_compiled = Command::new("ls")
                             .arg("/home/aidan/abs")
                             .output()
                             .expect("failed to read contents of directory /home/aidan/abs");

    let all_compiled= String::from_utf8(all_compiled.stdout).expect("all_compiled.stdout is not a String");

    let mut all_compiled_vec: Vec<String> = Vec::new();

    if all_compiled == "" {
        println!("nothing to compile");
    } else {
        for line in all_compiled.lines() {
            //println!("{}", line);
            all_compiled_vec.push(line.to_string());
        }
    }

    let mut to_compile_vec: Vec<String> = Vec::new();

    for x in 0..(all_compiled_vec.len()) {
        for y in 0..(to_update_vec.len()) {
            if all_compiled_vec[x] == to_update_vec[y] {
                println!("{} is to be built", all_compiled_vec[x]);
                to_compile_vec.push(all_compiled_vec[x].clone());

            }
        }
    }

    println!("currently updating upstream repositories");

    let update_upstream = Command::new("asp")
                                  .arg("update")
                                  .output()
                                  .expect("failed to update upstream repositories");

    println!("output of 'asp update': {}", String::from_utf8(update_upstream.stdout).expect("all_compiled.stdout is not a String"));

    println!("removing old packages");

    let root_dir = Path::new("/home/aidan/abs");

    for target_index in 0..to_compile_vec.len() {

        let mut target_build_directory = 
            format!("{}{}{}", "/home/aidan/abs/", to_compile_vec[target_index].clone(), "/trunk");

        let mut target_directory= 
            format!("{}{}", "/home/aidan/abs/", to_compile_vec[target_index].clone());

        //env::set_current_dir(&target_build_directory);
        //let cd = Command::new("cd") 
        //                 .arg(target_build_directory)
        //                 .output();
        //                 //.spawn();

        //let mut current_directory = Command::new("pwd")
        //                         .current_dir(&target_build_directory)
        //                         .output()
        //                         .expect("could not read current directory");

        println!("currently updating {} package directory", to_compile_vec[target_index]);

        let git_pull = Command::new("git")
                               .arg("pull")
                               .current_dir(&target_directory)
                               .output();

        let git_fetch = Command::new("git")
                                .arg("fetch")
                                .arg("--all")
                                .current_dir(&target_directory)
                                .output()
                                .expect("the command 'git fetch --all' failed");


        println!("output of 'git fetch --all': {}", String::from_utf8(git_fetch.stdout).expect("git_fetch.stdout is not a String"));

        let git_reset = Command::new("git")
                                .arg("reset")
                                .arg("--hard")
                                .current_dir(&target_directory)
                                .output()
                                .expect("the command 'git reset --hard' failed");

        println!("output of 'asp reset --hard': {}", String::from_utf8(git_reset.stdout).expect("git_reset.stdout is not a String"));


                println!("currently building {} package", to_compile_vec[target_index]);

        //let add_groups_pkgbuild = Command::new("echo")
        //                                  .arg("makepkg=('groups')")
        //                                  .arg(">>")
        //                                  .arg("PKGBUILD")
        //                                  .current_dir(&target_build_directory)
        //                                  .output();



        let edit_pkgbuild = Command::new("urxvt")
                                        .arg("-e")
                                        .arg("vim")
                                        .arg("PKGBUILD")
                                        .current_dir(&target_build_directory)
                                        .output();
            
        
        let make_package = Command::new("urxvt")
                                   .arg("-e")
                                   .arg("makepkg")
                                   .arg("-s")
                                   .arg("--skippgpcheck")
                                   .current_dir(&target_build_directory)
                                   .output();

    for entry in glob("/home/aidan/abs/*.pkg.tar").expect("failed to expand wildcard for installation of compiled packages") {
        Command::new("pacman")
                .arg("-U")
                .arg(entry.expect("failed to parse package to install"))
                .current_dir(root_dir);
    }
    }
}

