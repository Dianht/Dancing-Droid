use crate::party;

pub fn instruction(instruction_robot: &party::Instruction, robot: &mut party::Robot) {
    //Cette fonction va permettre de realisé les mouvements du robot
    //en fonction des instructions
    match instruction_robot {
        //En fonction de l'orientation du robot, le moment où l'instruction est F
        //le programme va modifier les coordonnée du robot
        party::Instruction::F => match robot.orientation {
            party::Orientation::North => robot.y = robot.y + 1,
            party::Orientation::West => robot.x = robot.x - 1,
            party::Orientation::Est => robot.x = robot.x + 1,
            party::Orientation::South => robot.y = robot.y - 1,
        },
        //En fonction de l'orientation du robot, le moment où l'instruction est L ou R
        //le programme va modifier l'orientation du robot
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
