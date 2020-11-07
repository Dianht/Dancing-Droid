use crate::party;
use colored::*;
use std::io; //Gadjet mais ça rajoute des couleurs ajouter colored = "2" dans Cargo.toml
use std::{thread, time};

pub fn choice(mut robot: &mut Vec<party::Robot>) -> party::Terrain {
    let ten_millis = time::Duration::from_millis(2000);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.as_str().trim() {
                "Y" => {
                    println!(
                        "\n{}\n         {}{}{}",
                        "Generation du monde aléatoire en cours 🌍 ...\n".green(),
                        "└[∵┌]".cyan().bold(),
                        "└[ ∵ ]┘".yellow().bold(),
                        "[┐∵]┘".magenta().bold()
                    );
                    thread::sleep(ten_millis);
                    return party::random_game::random_world(&mut robot);
                }
                "N" => {
                    println!(
                        "\n{}\n         {}{}{}",
                        "Generation du monde aléatoire en cours 🌍 ...".green(),
                        "└[∵┌]".red().bold(),
                        "└[ ∵ ]┘".blue().bold(),
                        "[┐∵]┘".green().bold()
                    );
                    thread::sleep(ten_millis);

                    return party::file::file(&mut robot);
                }
                _ => println!("Y/N ?"),
            },
            Err(error) => println!("????: {}", error),
        }
    }
}

pub fn initial_final(robot: &mut Vec<party::Robot>, position: String) {
    for i in 0..robot.len() {
        let id = robot[i].id;
        //On le parse pour mettre des jolies couleurs
        let s = id.to_string();
        if robot[i].instruction.is_empty() {
            println!(
                "Position {} du Robot<{}> -> chez lui (s'est pris une cuite)",
                position,
                s.green()
            )
        } else {
            println!(
                "Position {} du Robot<{}> -> x : {} y : {}",
                position,
                s.magenta().bold(),
                robot[i].x,
                robot[i].y
            );
        }
    }
}

pub fn display(robot: &mut Vec<party::Robot>, terrain: &mut party::Terrain) {
    println!("Terrain {{ {} }}", terrain);
    println!("Robots {{");
    for i in 0..robot.len() {
        println!("  {{ {}, }}", robot[i]);
    }
    println!("}}");
}
