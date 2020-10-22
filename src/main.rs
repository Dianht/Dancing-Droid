use colored::*;

enum Orientation {
    North,
    South,
    West,
    Est,
}
enum Instruction {
    F,
    L,
    R,
}
struct Robot {
    id: i32,
    x: i32,
    y: i32,
}

//Il faudrait qu'on optimise tout sa pour qu'on puisse utiliser cette fonction un robot par un (je peux le faire sans cette enum de m*)
fn game(instruction_robot1 : Vec<&Instruction>,robot1 : &mut Robot,direc_rb1: Orientation,instruction_robot2 : Vec<&Instruction>,robot2 : &mut Robot,direc_rb2: Orientation){
    
    let limite_y = 5;
    let limite_x = 5;

    let mut direction1 = direc_rb1;
    let mut direction2 = direc_rb2; 

    let taille = 
    if instruction_robot1.len() > instruction_robot2.len(){              //On prend la plus grande taille des deux vecteurs
        instruction_robot1.len()
    }else {
        instruction_robot2.len()
    };

    for i in 0..taille {
        if i < instruction_robot1.len() {
            match instruction_robot1[i]{
                Instruction::F => {
                    match direction1 {
                        Orientation::North => {
                            if robot1.x == robot2.x && robot1.y + 1 == robot2.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            }else if robot1.y + 1 == limite_y + 1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {
                                robot1.y = robot1.y + 1;
                            }
                        }
                        Orientation::West => {
                            if robot1.x - 1 == robot2.x && robot1.y == robot2.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            }else if robot1.x - 1 == -1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {
                                robot1.x = robot1.x - 1;
                            }
                        }
                        Orientation::Est => {
                            if robot1.x + 1 == robot2.x && robot1.y == robot2.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            } else if robot1.x + 1 == limite_x + 1{
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {    
                                robot1.x = robot1.x + 1;
                            }
                        }
                        Orientation::South =>{
                            if robot1.x == robot2.x && robot1.y - 1 == robot2.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            } else if robot1.y - 1 == -1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {
                                robot1.y = robot1.y - 1;
                            }
                        }
                    }
                }
                Instruction::L => {
                    match direction1 {
                        Orientation::North => {
                            direction1 = Orientation::West;
                        }
                        Orientation::West => {
                            direction1 = Orientation::South;
                        }
                        Orientation::Est => {
                            direction1 = Orientation::North;
                        }
                        Orientation::South => {
                            direction1 = Orientation::Est;
                        }
                    }
                }
                Instruction::R => {
                    match direction1 {
                        Orientation::North => {
                            direction1 = Orientation::Est;
                        }
                        Orientation::West => {
                            direction1 = Orientation::North;
                        }
                        Orientation::Est => {
                            direction1 = Orientation::South;
                        }
                        Orientation::South => {
                            direction1 = Orientation::West;
                        }
                    }
                }
            }
        }   
        if i < instruction_robot2.len() {
            match instruction_robot2[i]{
                Instruction::F => {
                    match direction2 {
                        Orientation::North => {
                            if robot2.x == robot1.x && robot2.y + 1 == robot1.y{
                                println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                            }else if robot2.y + 1 == limite_y + 1{
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot2.id);
                            }else{
                                robot2.y = robot2.y + 1;
                            }
                        }
                        Orientation::West => {
                            if robot2.x - 1 == robot1.x && robot2.y == robot1.y{
                                println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                            }else if robot2.x - 1 == -1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot2.id);
                            }else {
                                robot2.x = robot2.x - 1;
                            }
                        }
                        Orientation::Est => {
                            if robot2.x + 1 == robot1.x && robot2.y == robot1.y{
                                println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                            }else if robot2.x + 1 == limite_x + 1{
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot2.id);
                            }else {
                                robot2.x = robot2.x + 1;
                            }
                        }
                        Orientation::South =>{
                            if robot2.x == robot1.x && robot2.y - 1 == robot1.y{
                                println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                            } else if robot2.y - 1 == - 1{
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot2.id);
                            }else{
                                robot2.y = robot2.y - 1;
                            }
                        }
                    }
                }
                Instruction::L => {
                    match direction2 {
                        Orientation::North => {
                            direction2 = Orientation::West;
                        }
                        Orientation::West => {
                            direction2 = Orientation::South;
                        }
                        Orientation::Est => {
                            direction2 = Orientation::North;
                        }
                        Orientation::South => {
                            direction2 = Orientation::Est;
                        }
                    }
                }
                Instruction::R => {
                    match direction2 {
                        Orientation::North => {
                            direction2 = Orientation::Est;
                        }
                        Orientation::West => {
                            direction2 = Orientation::North;
                        }
                        Orientation::Est => {
                            direction2 = Orientation::South;
                        }
                        Orientation::South => {
                            direction2 = Orientation::West;
                        }
                    }
                }
            }
        }
        
    }

}

fn main() {

    let f = &Instruction::F;
    let r = &Instruction::R;
    let l = &Instruction::L;
    
    let direction_robot1 = Orientation::North;      // essaie de donner les valeurs de l'orientation ici
    let direction_robot2 = Orientation::South;      

    let instruction_robot1 = vec![f,l,l,f,r,f];
    let instruction_robot2 = vec![f,f,l,f,r,r,f];
    
    let mut vecteur = vec![Robot{ id: 0,x: 1,y: 1 },
                             Robot{ id: 1,x: 3,y: 2}];

    let (rb1,rb2) = vecteur.split_at_mut(1);    
    let  robot1 = &mut rb1[0];
    let  robot2 = &mut rb2[0];

    game(instruction_robot1,robot1,direction_robot1,instruction_robot2,robot2,direction_robot2);

    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".magenta(),robot1.id,robot1.x,robot1.y);
    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".cyan(),robot2.id,robot2.x,robot2.y);
        
}
    
