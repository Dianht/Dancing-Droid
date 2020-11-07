use crate::party;
use colored::*;
use rand::Rng;
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
    tmp: (i32, i32),
    i_instru: usize,
    robot: &mut Vec<party::Robot>,
    lim_y: i32,
    lim_x: i32,
    m: usize,
    crash: &mut HashSet<party::Crash>,
    obstacle: &mut Vec<party::Obstacle>,
    terrain: &mut party::Terrain,
) {
    let mut rng = rand::thread_rng();
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
                robot[m].x = tmp.0;
                robot[m].y = tmp.1;
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
                robot[m].x = tmp.0;
                robot[m].y = tmp.1;
            }
        } else {
            for o in 0..obstacle.len() {
                match obstacle[o].id {
                    0 => {
                        if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                            let s = format!(
                                    "{} 😵\nLe {}<{}> se met à vomir 🤮, le videur le sort de la piste de danse",
                                    "Obstacle".magenta().bold(),
                                    "Robot".green(),
                                    robot[m].id
                                );
                            crash.insert(party::Crash::Collision(s));
                            robot[m].instruction.clear(); //On nettoie sa liste d'instruction
                            return;
                        }
                    }
                    1 => {
                        if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                            let s = format!(
                                "{} 😵\nLe {}<{}> est en feu 🔥, il commence à improvisé une danse",
                                "Obstacle".magenta().bold(),
                                "Robot".green(),
                                robot[m].id
                            );
                            let u: Vec<_> = robot[m].instruction.drain(i_instru + 1..).collect();
                            if u.is_empty() {
                                for _ in 0..u.len() {
                                    let aleatoire = rng.gen_range(0, 3);
                                    match aleatoire {
                                        0 => robot[m].instruction.push(&party::Instruction::F),
                                        1 => robot[m].instruction.push(&party::Instruction::R),
                                        2 => robot[m].instruction.push(&party::Instruction::L),
                                        _ => (),
                                    }
                                }
                            }
                            crash.insert(party::Crash::Collision(s));
                            return;
                        }
                    }
                    2 => {
                        if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                            let s = format!(
                                "{} 😵\nLe {}<{}> s'est téléporté grâce à une faille spacio-temporelle 🌀",
                                "Obstacle".magenta().bold(),
                                "Robot".green(),
                                robot[m].id
                            );
                            robot[m].x = rng.gen_range(0, terrain.x);
                            robot[m].y = rng.gen_range(0, terrain.y);
                            crash.insert(party::Crash::Collision(s));
                            return;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
