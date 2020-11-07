use crate::party;
use colored::*;
use rand::Rng;

pub fn crash(
    tmp: (i32, i32),
    robot: &mut Vec<party::Robot>,
    m: usize,
    crash: &mut Vec<party::Crash>,
) {
    //La fonction crash va push dans le vecteur crash les collision déroulé durant la soirée
    //On va inspecter tout les robots pour comparer leurs coordonnées, ce n'est pas une bonne méthode
    //mais bon...

    for i in 0..robot.len() {
        //Si l'id du robot 1 et du robot 2 sont pas les même alors on peut les comparer
        //Si les coordonnées des deux robots sont les mêmes alors il y a là une collision
        if robot[m].id != robot[i].id {
            if robot[m].x == robot[i].x && robot[m].y == robot[i].y {
                let s = format!(
                    "{} 💥 \n{}<{}> fonce vers {}<{}> aux coordonnée x : {} y : {} !",
                    "Collision !".red().bold(),
                    "Le Robot".cyan(),
                    robot[m].id,
                    "le Robot".green(),
                    robot[i].id,
                    robot[m].x,
                    robot[m].y
                );
                //Le robot reprend alors sa position pre-collision
                robot[m].x = tmp.0;
                robot[m].y = tmp.1;
                //On push le texte de collision dans le vecteur crash
                crash.push(party::Crash::Collision(s));
            }
        }
    }
}
pub fn limit(
    tmp: (i32, i32),
    robot: &mut Vec<party::Robot>,
    m: usize,
    crash: &mut Vec<party::Crash>,
    terrain: &mut party::Terrain,
) {
    //La fonction limit va push dans le vecteur crash tout les robots qui tentent de dépasser la limite
    //du terrain
    if robot[m].x == terrain.x + 1
        || robot[m].x == -1
        || robot[m].y == terrain.y + 1
        || robot[m].y == -1
    {
        let s = format!(
                "{} 🚧 |x : {} | y : {}|\n{}<{}> se dirige vers les limites de la piste ! (il risque de tomber)",
                "Attention !".red().bold(),
                robot[m].x,robot[m].y,
                "Le Robot".yellow(),
                robot[m].id
                );
        //Le robot reprend alors sa position pre-collision
        robot[m].x = tmp.0;
        robot[m].y = tmp.1;
        //On push le texte de collision dans le vecteur crash
        crash.push(party::Crash::Attention(s));
    }
}
pub fn obstacle(
    obstacle: &mut Vec<party::Obstacle>,
    robot: &mut Vec<party::Robot>,
    m: usize,
    crash: &mut Vec<party::Crash>,
    terrain: &mut party::Terrain,
) {
    //La fonction obstacle va push dans le vecteur crash tout les robots qui ont rencontré un obstacle
    let mut rng = rand::thread_rng();
    //On va comparer les coordonnée du robot avec tout les coordonnée des obstacles
    //(C'est pas le meilleur des moyens mais bon...)
    for o in 0..obstacle.len() {
        //En fonction de l'id du robot, on crée des obstacles
        //si l'id est 1 par exemple l'obstacle est que le robot devient vomi
        match obstacle[o].id {
            0 => {
                //Si l'id est 0 alors l'obstacle consistera à ce que le robot s'arrete et que sa liste d'instruction soit vide
                //On compare les coordonées du robot et de l'obstacle
                if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                    let s = format!(
                            "|{} 😵 |x:  {} | y : {}|\nLe {}<{}> se met à vomir 🤮, le videur le sort de la piste de danse",
                            "Obstacle".magenta().bold(),
                        robot[m].x,robot[m].y,
                            "Robot".green(),
                            robot[m].id
                        );
                    //On oublie pas d'enlever cet obstacle de la liste
                    obstacle.remove(o);
                    //On push le texte de l'obstacle dans crash
                    crash.push(party::Crash::Collision(s));
                    //On nettoie sa liste d'instruction
                    robot[m].instruction.clear();
                    //Le robot se trouve plus sur la piste
                    robot[m].x = -2;
                    robot[m].y = -2;
                    //On a plus besoin de voir les autres obstacles
                    return;
                }
            }
            1 => {
                //Si l'id est 1 alors l'obstacle consistera à ce que le robot s'ajoute lui même 3 instructions
                if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                    let s = format!(
                        "{} 😵 |x : {} | y : {}|\nLe {}<{}> est en feu 🔥, il commence à improvisé une danse et le rajoute dans sa liste d'instruction",
                        "Obstacle".magenta().bold(),
                        robot[m].x,robot[m].y,
                        "Robot".green(),
                        robot[m].id
                    );
                    //Le robot improvise 3 mouvements sur la piste de dance
                    //qu'il va généré aléatoirement
                    for _ in 0..3 {
                        robot[m].instruction.push(party::random_game::random_instruction());
                    }

                    //On oublie pas d'enlever cet obstacle de la liste
                    obstacle.remove(o);
                    crash.push(party::Crash::Collision(s));
                    //On a plus besoin de voir les autres obstacles
                    return;
                }
            }
            2 => {
                if robot[m].x == obstacle[o].x && robot[m].y == obstacle[o].y {
                    //On crée deux variables qui correspondent aux futurs coordonnée généré aléatoirement du robot
                    let x = rng.gen_range(0, terrain.x);
                    let y = rng.gen_range(0, terrain.y);
                    let s = format!(
                        "{} 😵 |x : {} | y : {}|\nLe {}<{}> s'est téléporté grâce à une faille spacio-temporelle aux coordonnées x : {}  y : {} 🌀 !",
                        "Obstacle".magenta().bold(),
                        robot[m].x,robot[m].y,
                        "Robot".green(),
                        robot[m].id,
                        x,y
                    );
                    robot[m].x = x;
                    robot[m].y = y;
                    //On oublie pas d'enlever cet obstacle de la liste
                    obstacle.remove(o);
                    //On a plus besoin de voir les autres obstacles
                    crash.push(party::Crash::Collision(s));
                    return;
                }
            }
            _ => (),
        }
    }
}
