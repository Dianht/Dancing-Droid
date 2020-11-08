use crate::party;
use rand::Rng;

use party::Instruction as I;
use party::Orientation as O;
use party::Robot as R;
use party::Terrain as T;

pub fn random_robot(robot: &mut Vec<R>, terrain: &mut T, id: i32) {
    //Fonction qui crée des robots avec des instructions,orientation,coordonnée généré aléatoirement
    let mut rng = rand::thread_rng();
    let mut robot_instruction: Vec<&I> = Vec::new();
    let mut robot_coordonnee: (i32, i32);
    //Prendra une valeur pour la taille de la liste d'instruction entre 1 et 10 (10 n'est pas compté)
    let nbre_instruction = rng.gen_range(1, 10);

    //Genere aléatoirement les coordonnée x et y du robot

    //On ne veut pas que le programme panic!
    //Si un le coordonnée x ou y du terrain était égale à 0, il y aurait un panic! car
    //On ne peut généré aléatoirement un chiffre entre 0 et 0
    robot_coordonnee = random_coordonnee_rb(terrain);
    //Empeche qu'on ait plusieurs robots avec les mêmes coordonnées
    //Si le vecteur robot à au moins un robot dans son vecteur alors on regarde si
    //le robot que la fonction "rand_robot" est entrain de crée à les même coordonnée
    //qu'un robot dans le vecteur
    let mut i = 0;
    if robot.len() > 0 {
        //Le programme ne s'arretera uniquement lorsque tout les robots ont des coordonnée différents
        loop {
            if robot[i].x == robot_coordonnee.0 && robot[i].y == robot_coordonnee.1 {
                //On genere alors de nouveaux coordonées
                robot_coordonnee = random_coordonnee_rb(terrain);
                i = 0;
            }
            if i == robot.len() - 1 {
                break;
            }
            i += 1;
        }
    }

    //Genere aléatoirement une orientation grâce à la fonction "random_orientation"
    //et des instructions en fonction du nombre d'instruction généré grâce à la fonction "random_instruction"
    let robot_orientation = random_orientation();
    for _ in 0..nbre_instruction {
        robot_instruction.push(random_instruction());
    }

    //On push le nouveau robot dans un vecteur
    robot.push(R {
        id: id,
        x: robot_coordonnee.0,
        y: robot_coordonnee.1,
        orientation: robot_orientation,
        instruction: robot_instruction,
    });
}
pub fn random_world(mut robot: &mut Vec<R>) -> T {
    //Cette fonction va généré un monde aléatoirement avec le terrain et le nombre de robot
    let mut rng = rand::thread_rng();
    let mut terrain = T {
        x: rng.gen_range(1, 20),
        y: rng.gen_range(1, 20),
    };

    //On crée un nombre de robot proportionnel au tiers du terrain
    //Cela nous évitera d'avoir + de robots que de case dans le terrain
    let mut max = (terrain.x * terrain.y) / 3;
    if max == 0 {
        max = 1;
    }
    let mut nbre_robots = rng.gen_range(0, max);
    if nbre_robots > 10 {
        //Par contre + de 10 robot c'est beaucoup trop
        nbre_robots = 10;
    }

    //On crée une boucle qui va crée des robots à l'aide de la fonction "random_robot"
    //"p" correspond à l'id du robot
    for p in 0..nbre_robots {
        random_robot(&mut robot, &mut terrain, p);
    }

    return terrain;
}

pub fn random_instruction() -> &'static I {
    //Cette fonction return aléatoire un des trois instructions
    let mut rng = rand::thread_rng();
    let aleatoire = rng.gen_range(0, 3);
    match aleatoire {
        0 => return &I::F,
        1 => return &I::R,
        2 => return &I::L,
        _ => return &I::R, //() ne fonctionné pas
    }
}
fn random_orientation() -> O {
    //Cette fonction return aléatoirement un des quatre orientation
    let mut rng = rand::thread_rng();
    let aleatoire = rng.gen_range(0, 4);
    match aleatoire {
        0 => return O::North,
        1 => return O::West,
        2 => return O::Est,
        3 => return O::South,
        _ => return O::South,
    }
}
fn random_coordonnee_rb(terrain: &mut T) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    if terrain.x == 0 {
        return (0, rng.gen_range(0, terrain.y));
    } else if terrain.y == 0 {
        return (rng.gen_range(0, terrain.x), 0);
    } else {
        return (rng.gen_range(0, terrain.x), rng.gen_range(0, terrain.y));
    }
}
