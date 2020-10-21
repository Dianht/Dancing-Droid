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



fn main() {

    
    let limite_x : i32 = 5;
    let limite_y : i32 = 5;
    
    let mut direction_robot1 = Orientation::North;
    let mut direction_robot2 = Orientation::South;

    let instruction_robot1 = vec![Instruction::F,Instruction::L,Instruction::L,Instruction::F,Instruction::R,Instruction::F];
    let instruction_robot2 = vec![Instruction::F,Instruction::F,Instruction::L,Instruction::F,Instruction::R,Instruction::R,Instruction::F];
    let mut vecteur = vec![Robot{ id: 0,x: 1,y: 1 },
                             Robot{ id: 1,x: 3,y: 2}];

    let (rb1,rb2) = vecteur.split_at_mut(1);    
    let mut robot1 = &mut rb1[0];
    let mut robot2 = &mut rb2[0];
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
                    match direction_robot1 {
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
                            if robot2.x - 1 == robot1.x && robot2.y == robot1.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            }else if robot2.x - 1 == -1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {
                                robot2.x = robot2.x - 1;
                            }
                        }
                        Orientation::Est => {
                            if robot2.x + 1 == robot1.x && robot2.y == robot1.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            } else if robot2.x + 1 == limite_x + 1{
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {    
                                robot2.x = robot2.x + 1;
                            }
                        }
                        Orientation::South =>{
                            if robot2.x == robot1.x && robot2.y - 1 == robot1.y {
                                println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                            } else if robot2.y - 1 == -1 {
                                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
                            }else {
                                robot2.y = robot2.y - 1;
                            }
                        }
                    }
                }
                Instruction::L => {
                    match direction_robot1 {
                        Orientation::North => {
                            direction_robot1 = Orientation::West;
                        }
                        Orientation::West => {
                            direction_robot1 = Orientation::South;
                        }
                        Orientation::Est => {
                            direction_robot1 = Orientation::North;
                        }
                        Orientation::South => {
                            direction_robot1 = Orientation::Est;
                        }
                    }
                }
                Instruction::R => {
                    match direction_robot1 {
                        Orientation::North => {
                            direction_robot1 = Orientation::Est;
                        }
                        Orientation::West => {
                            direction_robot1 = Orientation::North;
                        }
                        Orientation::Est => {
                            direction_robot1 = Orientation::South;
                        }
                        Orientation::South => {
                            direction_robot1 = Orientation::West;
                        }
                    }
                }
            }
        }
        if i < instruction_robot2.len() {
            match instruction_robot2[i]{
                Instruction::F => {
                    match direction_robot2 {
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
                    match direction_robot2 {
                        Orientation::North => {
                            direction_robot2 = Orientation::West;
                        }
                        Orientation::West => {
                            direction_robot2 = Orientation::South;
                        }
                        Orientation::Est => {
                            direction_robot2 = Orientation::North;
                        }
                        Orientation::South => {
                            direction_robot2 = Orientation::Est;
                        }
                    }
                }
                Instruction::R => {
                    match direction_robot2 {
                        Orientation::North => {
                            direction_robot2 = Orientation::Est;
                        }
                        Orientation::West => {
                            direction_robot2 = Orientation::North;
                        }
                        Orientation::Est => {
                            direction_robot2 = Orientation::South;
                        }
                        Orientation::South => {
                            direction_robot2 = Orientation::West;
                        }
                    }
                }
            }
        }
        
    }
    
    



    


    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".magenta(),robot1.id,robot1.x,robot1.y);
    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".cyan(),robot2.id,robot2.x,robot2.y);
        
}
    
