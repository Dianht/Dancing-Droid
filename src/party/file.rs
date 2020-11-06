use crate::party;
use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn file(mut robot: &mut Vec<party::Robot>) -> party::Terrain {
    let path = Path::new("../two_robots.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Impossible d'ouvrir {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(why) => panic!("Impossible de lire {}: {}", display, why),
    }
    let c: Vec<&str> = s.split(|c| c == '\n' || c == ' ').collect();

    let mut m: Vec<&str> = Vec::new();
    let mut id: i32 = 0;

    let mut terrain = party::Terrain { x: 0, y: 0 };

    let _number = match c[0].parse::<i32>() {
        Ok(number) => terrain.x = number,
        Err(_) => (),
    };
    let _number = match c[1].parse::<i32>() {
        Ok(number) => terrain.y = number,
        Err(_) => (),
    };

    //on  considere que à partir de c[3] on a les robots
    for i in 3..c.len() {
        //lorsque que que c[i] == "" on considere qu'on passe à un autre robot
        if c[i] == "" {
            //Un robot doit recevoir 4 String (x,y,orientation,instruction)
            if m.len() == 4 {
                party::normal_game::create_robot(&mut robot, &mut m, id, 'N');
                id += 1;
                m.clear();
            }
            //Si le vecteur n'a reçu que 3 string (x,y,orientation),cela signifie qu'il n'a pas reçu d'instruction
            else if m.len() == 3 {
                m.push(c[i]);
                let s = id.to_string();
                println!("🚨 Le robot<{}> ne contient pas d'instruction, les instructions seront générés aléatoirement 🎲 ...",s.red());
                //On envoie le char "O"(oui) qui va dire au programme que le robot n'a pas d'instruction
                party::normal_game::create_robot(&mut robot, &mut m, id, 'O');
                id += 1;
                m.clear();
            } else {
                break;
            }
        } else {
            m.push(c[i]);
        }
    }

    return terrain;
}
