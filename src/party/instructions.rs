use crate::party;
use party::Instruction as I;
use party::Orientation as O;

pub fn instruction(instruction_robot: &party::Instruction, robot: &mut party::Robot) {
    //Cette fonction va permettre de realisé les mouvements du robot
    //en fonction des instructions
    match instruction_robot {
        //En fonction de l'orientation du robot, le moment où l'instruction est F
        //le programme va modifier les coordonnée du robot
        I::F => match robot.orientation {
            O::North => robot.y = robot.y + 1,
            O::West => robot.x = robot.x - 1,
            O::Est => robot.x = robot.x + 1,
            O::South => robot.y = robot.y - 1,
        },
        //En fonction de l'orientation du robot, le moment où l'instruction est L ou R
        //le programme va modifier l'orientation du robot
        I::L => match robot.orientation {
            O::North => robot.orientation = O::West,
            O::West => robot.orientation = O::South,
            O::Est => robot.orientation = O::North,
            O::South => robot.orientation = O::Est,
        },
        I::R => match robot.orientation {
            O::North => robot.orientation = O::Est,
            O::West => robot.orientation = O::North,
            O::Est => robot.orientation = O::South,
            O::South => robot.orientation = O::West,
        },
    }
}
