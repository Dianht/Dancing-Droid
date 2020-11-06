use crate::party;
use colored::*;

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
) {
    for i in 0..robot.len() {
        if robot[m].x == robot[i].x && robot[m].y == robot[i].y {
            if robot[m].id != robot[i].id {
                println!(
                    "{} 💥\n{}<{}> fonce vers {}<{}> aux coordonnée x : {} y : {} !",
                    "Collision !".red(),
                    "Le Robot".cyan(),
                    robot[m].id,
                    "le Robot".green(),
                    robot[i].id,
                    robot[m].x,
                    robot[m].y
                );
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        } else if robot[m].x == lim_x + 1
            || robot[m].x < 0
            || robot[m].y == lim_y + 1
            || robot[m].y < 0
        {
            if robot[m].id != robot[i].id {
                println!(
                    "{} 🚧\n{}<{}> se dirige vers les limites de la map !",
                    "Attention !".red(),
                    "Le Robot".yellow(),
                    robot[m].id
                );
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        }
    }
}
