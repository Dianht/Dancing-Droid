use crate::party;
use party::Instruction as I;
use party::Orientation as O;
use party::Robot as R;

pub fn instruction(instruction_robot: &I, robot: &mut R) -> bool {
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
    return true;
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::party;
    use party::Instruction as I;
    use party::Orientation as O;
    use party::Robot as R;
    #[test]
    fn test_intruction() {
        let mut rb = vec![R {
            id: 1,
            x: 1,
            y: 2,
            orientation: O::North,
            instruction: vec![&I::F, &I::L],
        }];
        assert_eq!(instruction(&I::F, &mut rb[0]), true);
        assert_eq!(instruction(&I::L, &mut rb[0]), true);
        assert_eq!(instruction(&I::R, &mut rb[0]), true);
    }
}
