use crate::party;
use rand::Rng; //Pour générer des nombres aléatoires(ajouter rand = "0.7" dans Cargo.toml)

pub fn random_robot(robot: &mut Vec<party::Robot>, terrain: &mut party::Terrain, id: i32) {
    let mut rng = rand::thread_rng();
    let mut orientation = party::Orientation::North;
    let mut robot_instruction: Vec<&party::Instruction> = Vec::new();

    let x = rng.gen_range(0, terrain.x);
    let y = rng.gen_range(0, terrain.y);
    let nbre_instruction = rng.gen_range(1, 10);
    let mut i = 0;

    let aleatoire = rng.gen_range(0, 4);
    match aleatoire {
        0 => orientation = party::Orientation::North,
        1 => orientation = party::Orientation::West,
        2 => orientation = party::Orientation::Est,
        3 => orientation = party::Orientation::South,
        _ => (),
    }
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
    robot.push(party::Robot {
        id: id,
        x: x,
        y: y,
        orientation: orientation,
        instruction: robot_instruction,
    });
}
pub fn random_world(mut robot: &mut Vec<party::Robot>) -> party::Terrain {
    let mut rng = rand::thread_rng();
    let mut terrain = party::Terrain {
        x: rng.gen_range(1, 20),
        y: rng.gen_range(1, 20),
    };

    let nbre_robots = rng.gen_range(0, 10);

    let mut p = 0;

    while p != nbre_robots {
        random_robot(&mut robot, &mut terrain, p);
        p += 1;
    }

    return terrain;
}
