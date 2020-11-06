use crate::party;
use rand::Rng;
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
                create_robot(&mut robot, &mut m, id, 'N');
                id += 1;
                m.clear();
            }
            //Si le vecteur n'a reçu que 3 string (x,y,orientation),cela signifie qu'il n'a pas reçu d'instruction
            else if m.len() == 3 {
                m.push(c[i]);
                println!("🚨 Le robot<{}> ne contient pas d'instruction, les instructions seront générés aléatoirement 🎲 ...",id);
                //On envoie le char "O"(oui) qui va dire au programme que le robot n'a pas d'instruction
                create_robot(&mut robot, &mut m, id, 'O');
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
pub fn create_robot(robot: &mut Vec<party::Robot>, c: &mut Vec<&str>, id: i32, vide: char) {
    let mut robot_instruction: Vec<&party::Instruction> = Vec::new();
    let mut position: Vec<i32> = Vec::new();
    let mut orientation = party::Orientation::North; //Faut trouver un autre moyen pour l'initialiser

    for i in 0..2 {
        let _number = match c[i].parse::<i32>() {
            Ok(number) => position.push(number),
            Err(_) => (),
        };
    }
    match c[2] {
        "N" => orientation = party::Orientation::North,
        "E" => orientation = party::Orientation::Est,
        "W" => orientation = party::Orientation::West,
        "S" => orientation = party::Orientation::South,
        _ => println!("C'est une Orientation ???? {}", c[2]),
    }
    if vide == 'N' {
        let instruction: Vec<char> = c[3].chars().collect();
        for i in 0..instruction.len() {
            match instruction[i] {
                'F' => robot_instruction.push(&party::Instruction::F),
                'R' => robot_instruction.push(&party::Instruction::R),
                'L' => robot_instruction.push(&party::Instruction::L),
                _ => {
                    println!("C'est une instruction ???? {} ", instruction[i]);
                    break;
                }
            }
        }
    } else if vide == 'O' {
        let mut i: usize = 0;
        let mut rng = rand::thread_rng();
        let nbre_instruction = rng.gen_range(1, 10);
        while i != nbre_instruction {
            let aleatoire = rng.gen_range(0, 3);
            match aleatoire {
                0 => robot_instruction.push(&party::Instruction::F),
                1 => robot_instruction.push(&party::Instruction::R),
                2 => robot_instruction.push(&party::Instruction::L),
                _ => (),
            }
            i += 1;
        }
    }

    let y = position[0];
    let x = position[1];
    robot.push(party::Robot {
        id: id,
        x: x,
        y: y,
        orientation: orientation,
        instruction: robot_instruction,
    });
}

pub fn game(lim_x: i32, lim_y: i32, mut robot: &mut Vec<party::Robot>) {
    //On trouve le grand nombre d'instruction qu'a un robots
    let mut taille = robot[0].instruction.len();
    for i in 0..robot.len() - 1 {
        if taille > robot[i + 1].instruction.len() {
            taille = robot[i].instruction.len();
        } else {
            taille = robot[i + 1].instruction.len()
        }
    }
    // Nous permetra de stocker les valeurs des coordonées du robot avant l'instruction
    let mut tmp: (i32, i32);
    //Une boucle jusqu'à que le robot avec le + d'instruction n'aura plus d'instruction
    for x in 0..taille {
        for i in 0..robot.len() {
            //Une boucle qui fera jouer les robots un par un
            if x < robot[i].instruction.len() {
                //On ignorera les Robot qui n'ont plus d'instruction
                tmp = (robot[i].x, robot[i].y);
                party::rules::instruction(robot[i].instruction[x], &mut robot[i]);
                party::rules::collision(tmp.0, tmp.1, &mut robot, lim_y, lim_x, i);
            }
        }
    }
}
