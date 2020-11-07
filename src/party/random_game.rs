use crate::party;
use rand::Rng;

pub fn random_robot(robot: &mut Vec<party::Robot>, terrain: &mut party::Terrain, id: i32) {
    //Fonction qui crée des robots avec des instructions,orientation,coordonnée généré aléatoirement
    let mut rng = rand::thread_rng();
    let mut robot_instruction: Vec<&party::Instruction> = Vec::new();

    //Prendra une valeur pour la taille de la liste d'instruction entre 1 et 10 (10 n'est pas compté)
    let nbre_instruction = rng.gen_range(1, 10);

    //Genere aléatoirement les coordonnée x et y
    let robot_coordonnee = (rng.gen_range(0, terrain.x), rng.gen_range(0, terrain.y));
    //Genere aléatoirement une orientation grâce à la fonction "random_orientation"
    //et des instructions en fonction de la taille grâce à la fonction "random_instruction"
    let robot_orientation = random_orientation();
    for _ in 0..nbre_instruction {
        robot_instruction.push(random_instruction());
    }

    //On push le nouveau robot dans un vecteur
    robot.push(party::Robot {
        id: id,
        x: robot_coordonnee.0,
        y: robot_coordonnee.1,
        orientation: robot_orientation,
        instruction: robot_instruction,
    });
}
pub fn random_world(mut robot: &mut Vec<party::Robot>) -> party::Terrain{
    //Cette fonction va généré un monde aléatoirement
    //Avec Terrain et le nombre de robot
    let mut rng = rand::thread_rng();
    let mut terrain = party::Terrain {
        x: rng.gen_range(1, 20),
        y: rng.gen_range(1, 20),
    };

    let nbre_robots = rng.gen_range(0, 10);
    //On crée une boucle qui va crée des robots à l'aide de la fonction "random_robot"
    //"p" correspond à l'id du robot
    for p in 0..nbre_robots {
        random_robot(&mut robot, &mut terrain, p);    
    }
    return terrain;
}

pub fn random_instruction() -> &'static party::Instruction {
    //Cette fonction return aléatoire un des trois instructions
    let mut rng = rand::thread_rng();
    let aleatoire = rng.gen_range(0, 3);
    match aleatoire {
        0 => return &party::Instruction::F,
        1 => return &party::Instruction::R,
        2 => return &party::Instruction::L,
        _ => return &party::Instruction::R, //() ne fonctionné pas
    }
}
fn random_orientation() -> party::Orientation {
    //Cette fonction return aléatoirement un des quatre orientation
    let mut rng = rand::thread_rng();
    let aleatoire = rng.gen_range(0, 4);
    match aleatoire {
        0 => return party::Orientation::North,
        1 => return party::Orientation::West,
        2 => return party::Orientation::Est,
        3 => return party::Orientation::South,
        _ => return party::Orientation::South,
    }
}
