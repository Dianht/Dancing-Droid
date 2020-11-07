use crate::party;
use rand::Rng;
use std::collections::HashSet;
use std::{thread, time};

pub fn create_robot(robot: &mut Vec<party::Robot>, c: &mut Vec<&str>, id: i32, vide: char) {
    let mut robot_instruction: Vec<&party::Instruction> = Vec::new();
    let mut position: Vec<i32> = Vec::new();
    let mut orientation = party::Orientation::North;

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
    robot.push(party::Robot {
        id: id,
        x: position[1],
        y: position[0],
        orientation: orientation,
        instruction: robot_instruction,
    });
}

pub fn game(
    lim_x: i32,
    lim_y: i32,
    mut robot: &mut Vec<party::Robot>,
    mut terrain: party::Terrain,
) {
    let mut crash: HashSet<party::Crash> = HashSet::new();
    let mut rng = rand::thread_rng();
    let mut max = (terrain.x * terrain.y) / 3;
    if max == 0 {
        max = 1;
    }
    let mut obstacle = Vec::new();
    let mut i = 0;
    for _ in 0..rng.gen_range(0, max) {
        obstacle.push(party::Obstacle {
            x: rng.gen_range(0, terrain.x),
            y: rng.gen_range(0, terrain.y),
            id: i,
        });
        i += 1;
        if i > 2 {
            i = 0;
        }
    }

    let mut tmp: (i32, i32);
    for x in 0..taille(robot) {
        for i in 0..robot.len() {
            if x < robot[i].instruction.len() {
                tmp = (robot[i].x, robot[i].y);
                party::rules::instruction(robot[i].instruction[x], &mut robot[i]);
                if robot.len() > 1 {
                    party::rules::collision(
                        tmp,
                        i,
                        &mut robot,
                        lim_y,
                        lim_x,
                        i,
                        &mut crash,
                        &mut obstacle,
                        &mut terrain,
                    );
                }
            }
        }
        taille(robot);
    }
    let ten_millis = time::Duration::from_millis(500);
    if crash.is_empty() {
        println!("La soirée s'est bien passé, aucun incident à déplorer");
    } else {
        for aie in &crash {
            println!("{}", aie);
            thread::sleep(ten_millis);
        }
    }
}

pub fn taille(robot: &mut Vec<party::Robot>) -> usize {
    let mut taille = robot[0].instruction.len();
    for i in 0..robot.len() - 1 {
        if taille > robot[i + 1].instruction.len() {
            taille = robot[i].instruction.len();
        } else {
            taille = robot[i + 1].instruction.len()
        }
    }
    return taille;
}
