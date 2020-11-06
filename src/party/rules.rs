use crate::party;
use colored::*;
use std::collections::HashSet;

pub fn instruction(instruction_robot: &party::Instruction, robot: &mut party::Robot) {
    match instruction_robot {
        party::Instruction::F => match robot.orientation {
            party::Orientation::North => robot.y = robot.y + 1,
            party::Orientation::West => robot.x = robot.x - 1,
            party::Orientation::Est => robot.x = robot.x + 1,
            party::Orientation::South => robot.y = robot.y - 1,
        },
        party::Instruction::L => match robot.orientation {
            party::Orientation::North => robot.orientation = party::Orientation::West,
            party::Orientation::West => robot.orientation = party::Orientation::South,
            party::Orientation::Est => robot.orientation = party::Orientation::North,
            party::Orientation::South => robot.orientation = party::Orientation::Est,
        },
        party::Instruction::R => match robot.orientation {
            party::Orientation::North => robot.orientation = party::Orientation::Est,
            party::Orientation::West => robot.orientation = party::Orientation::North,
            party::Orientation::Est => robot.orientation = party::Orientation::South,
            party::Orientation::South => robot.orientation = party::Orientation::West,
        },
    }
}

pub fn collision(
    tmp_x: i32,
    tmp_y: i32,
    robot: &mut Vec<party::Robot>,
    lim_y: i32,
    lim_x: i32,
    m: usize,
    crash: &mut HashSet<party::Crash>,
    obstacle: (i32, i32),
) {
    for i in 0..robot.len() {
        if robot[m].x == robot[i].x && robot[m].y == robot[i].y {
            if robot[m].id != robot[i].id {
                let s = format!(
                    "{} 💥\n{}<{}> fonce vers {}<{}> aux coordonnée x : {} y : {} !",
                    "Collision !".red().bold(),
                    "Le Robot".cyan(),
                    robot[m].id,
                    "le Robot".green(),
                    robot[i].id,
                    robot[m].x,
                    robot[m].y
                );
                crash.insert(party::Crash::Collision(s));
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        } else if robot[m].x == lim_x + 1
            || robot[m].x < 0
            || robot[m].y == lim_y + 1
            || robot[m].y < 0
        {
            if robot[m].id != robot[i].id {
                let s = format!(
                    "{} 🚧\n{}<{}> se dirige vers les limites de la piste ! (il risque de tomber)",
                    "Attention !".red().bold(),
                    "Le Robot".yellow(),
                    robot[m].id
                );
                crash.insert(party::Crash::Attention(s));
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        } else if robot[m].x == obstacle.0 && robot[m].y == obstacle.1 {
            let s = format!(
                "{} 🤮\nLe {}<{}> se met à vomir, le videur le sort de la piste de danse",
                "Obstacle".magenta().bold(),
                "Robot".green(),
                robot[m].id
            );
            crash.insert(party::Crash::Collision(s));
            robot[m].instruction.clear(); //On nettoie sa liste d'instruction
            break;
        }
    }
}
