use crate::party;
use colored::*;
use std::io; //Gadjet mais ça rajoute des couleurs ajouter colored = "2" dans Cargo.toml

pub fn choice(mut robot: &mut Vec<party::Robot>) -> party::Terrain {
    //Tant que l'utilisateur n'aura pas entré une réponse valide, on fera une boucle
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.as_str().trim() {
                    //.trim() pour enlever les \n
                    "Y" => {
                        println!(
                            "\n{}\n         {}{}{}",
                            "Generation du monde aléatoire en cours 🌍 ...".green(),
                            "└[∵┌]".cyan(),
                            "└[ ∵ ]┘".yellow(),
                            "[┐∵]┘".magenta()
                        );
                        return party::random_game::random_world(&mut robot);
                    }
                    "N" => {
                        println!(
                            "\n{}\n         {}{}{}",
                            "Generation du monde aléatoire en cours 🌍 ...".green(),
                            "└[∵┌]".red(),
                            "└[ ∵ ]┘".blue(),
                            "[┐∵]┘".green()
                        );
                        return party::normal_game::file(&mut robot);
                    }
                    _ => println!("Y/N ?"),
                }
            }
            Err(error) => println!("????: {}", error),
        }
    }
}

pub fn initial_final(robot: &mut Vec<party::Robot>, position: String) {
    for i in 0..robot.len() {
        let id = robot[i].id;
        //On le parse pour mettre des jolies couleurs
        let s = id.to_string();
        println!(
            "{} du Robot<{}>  x : {} y : {}",
            position.magenta(),
            s.magenta(),
            robot[i].x,
            robot[i].y
        );
    }
    println!("\n");
}

pub fn display(robot: &mut Vec<party::Robot>, terrain: &mut party::Terrain) {
    println!("Terrain {{ {} }}", terrain);
    println!("Robots {{");
    for i in 0..robot.len() {
        println!("  {{ {}, }}", robot[i]);
    }
    println!("}}");
}
