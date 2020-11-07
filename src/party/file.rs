use crate::party;
use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn file(mut robot: &mut Vec<party::Robot>) -> party::Terrain {
    //On crée deux variable, un qui va contenenir le chemin où se trouve le fichier des robots
    //Et l'autre pour afficher le chemin en cas de panic!
    let path = Path::new("../two_robots.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Impossible d'ouvrir {}: {}", display, why),
        Ok(file) => file,
    };

    //La variable s va contenir tout le contenue du fichier robot
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(why) => panic!("Impossible de lire {}: {}", display, why),
    }
    //Pour séparer les différents robots qu'il peut y avoir
    //Le vecteur c va contenir chaque éléments du fichier two_robot.txt
    //Plutot que d'avoir uniquement chaque caractere du fichier txt, on aura là
    //à chaque "\n" et ' ' un string qui precede ces "\n" et ' '
    //pour avoir aussi des nombres
    let c: Vec<&str> = s.split(|c| c == '\n' || c == ' ').collect();

    let mut m: Vec<&str> = Vec::new();
    let mut id: i32 = 0;

    let mut terrain = party::Terrain {x : 0, y : 0};
    //Bien evidemment le fichier two_robots.txt commence par les coordonnées du terrain
    //Du coup, les deux premieres cases du vecteur seront les coordonnées du terrain
    //Étant donné que le vecteur c ne contient que des String, on devra parser le string en i32
    let _number = match c[0].parse::<i32>() {
        Ok(number) => terrain.x = number,
        Err(_) => (),
    };
    let _number = match c[1].parse::<i32>() {
        Ok(number) => terrain.y = number,
        Err(_) => (),
    };

    //Une fois qu'on aura eu les coordonnée du terrain
    //On aura affaire aux robot, donc les informations des robots commencent à la case 3
    for i in 3..c.len() {
        //Lorsque une case ne contiendra une string ""
        //Le programme dira que l'on est passé à un autre robot
        if c[i] == "" {
            //Un robot doit recevoir 4 String (x,y,orientation,instruction)
            if m.len() == 4 {
                //On envoie le vecteur qui contient les informations à la fonction
                //create_robot qui va se charger de parser tout les chaines de caractere
                //on oublie pas de lui donnée un id
                //'N' va juste dire que le robot à bien tout sa liste d'instruction
                party::normal_game::create_robot(&mut robot, &mut m, id, 'N');
                id += 1;
                //Bien sur on réintilise le vecteur pour que le vecteur m puisse acceuillir d'autre robots
                m.clear();
            }
            //Si le vecteur n'a reçu que 3 string (x,y,orientation),cela signifie qu'il n'a pas reçu d'instruction
            else if m.len() == 3 {
                m.push(c[i]);
                let s = id.to_string();
                println!("🚨 Le robot<{}> ne contient pas d'instruction, les instructions seront générés aléatoirement 🎲 ...",s.red());
                //On envoie le char "O"(oui) qui va dire au programme que le robot n'a pas d'instruction
                party::normal_game::create_robot(&mut robot, &mut m, id, 'O');
                id += 1;
                m.clear();
            } else {
                break;
            }
        } else {
            m.push(c[i]);
        }
    }

    return terrain;
}
