struct Robot<'a>  {
    id: String,
    orientation : String,
    instructions : Vec<&'a str>,
    x: i16,
    y: i16,
}



fn main() {

    
    let limite_x : i16 = 3;
    let limite_y : i16 = 3;
    
    
    let mut vecteur = vec![Robot{ id: "Blitz".to_string(), orientation: "North".to_string(),instructions: ["F","F","F","F"].to_vec(),x: 1,y: 1 },
                             Robot{ id: "Dinabot".to_string(), orientation: "North".to_string(),instructions: ["F","F","F"].to_vec(),x: 2,y: 2}];

    let (rb1,rb2) = vecteur.split_at_mut(1);    //Si on fait directement rb1 = &mut robot1[0]et rb = &mut robot2[1]
    let mut robot1 = &mut rb1[0];
    let mut robot2 = &mut rb2[0];

    let taille = if robot1.instructions.len() > robot2.instructions.len(){              //On prend la plus grande taille des deux vecteurs
                        robot1.instructions.len()}
                else {
                    robot2.instructions.len()
                };

    println!("Robot<{}> // position initial x : {} y : {}, orientation : {}, instruction{:?}",robot1.id,robot1.x,robot1.y,robot1.orientation,robot1.instructions);
    println!("Robot<{}> // position initial x : {} y : {}, orientation : {}, instruction{:?}",robot2.id,robot2.x,robot2.y,robot2.orientation,robot2.instructions);

    


    for i in 0..taille {
        if i != robot1.instructions.len(){          //Parce que si i est superieur à la taille d'un des deux tableaux, le programme s'arrete du coup on arrete seulement un des dexu match
            match robot1.instructions[i]{
                "F" => {
                    if robot1.orientation == "North"{
                        if robot1.x == robot2.x && robot1.y + 1 == robot2.y || robot1.y + 1 == limite_y {
                            println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                        }else {
                            robot1.y = robot1.y + 1;
                        }
                    }
                    else if robot1.orientation == "West"{
                        if robot1.x - 1 == robot2.x && robot1.y == robot2.y || robot1.x - 1 == limite_x {
                            println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                        } else {
                            robot1.x = robot1.x - 1;
                        }
                    }
                    else if robot1.orientation == "Est"{
                        if robot1.x + 1 == robot2.x && robot1.y == robot2.y || robot1.x + 1 == limite_x {
                            println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                        } else {
                            robot1.x = robot1.x + 1;
                        }
                    }   
                    else if robot1.orientation == "South"{
                        if robot1.x == robot2.x && robot1.y - 1 == robot2.y || robot1.y - 1 == limite_y{
                            println!("Collision ! du Robot<{}> aux coordonnées x:{}, y:{}",robot1.id,robot1.x,robot1.y);
                        } else {
                            robot1.y = robot1.y - 1;
                        }
                    }
                }
                "L" => {
                    if robot1.orientation == "North"{
                        robot1.orientation = "West".to_string();
                    }
                    else if robot1.orientation == "West"{
                        robot1.orientation = "South".to_string();
                    }
                    else if robot1.orientation == "Est"{
                        robot1.orientation = "North".to_string();
                    }
                    else if robot1.orientation == "South"{
                        robot1.orientation = "Est".to_string();
                    }
                
                }
                "R" => {
                    if robot1.orientation == "North"{
                        robot1.orientation = "Est".to_string();
                    }
                    else if robot1.orientation == "West"{
                        robot1.orientation = "North".to_string();
                    }
                    else if robot1.orientation == "Est"{
                        robot1.orientation = "South".to_string();
                    }
                    else if robot1.orientation == "South"{
                        robot1.orientation = "West".to_string();
                    }
                    }
                _ => println!("Je reconnais pas votre lettre"),
            }
        }
        if i != robot2.instructions.len(){
            match robot2.instructions[i]{
                "F" => {
                    if robot2.orientation == "North"{
                        if robot2.x == robot1.x && robot2.y + 1 == robot1.y || robot2.y + 1 == limite_y {
                            println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                        }else {
                            robot2.y = robot2.y + 1;
                        }
                    }
                    else if robot2.orientation == "West"{
                        if robot2.x - 1 == robot1.x && robot2.y == robot1.y || robot2.x - 1 == limite_x {
                            println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                        } else {
                            robot2.x = robot2.x - 1;
                        }
                    }
                    else if robot2.orientation == "Est"{
                        if robot2.x + 1 == robot1.x && robot2.y == robot1.y || robot2.x + 1 == limite_x {
                            println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                        } else {
                            robot2.x = robot2.x + 1;
                        }
                    }   
                    else if robot2.orientation == "South"{
                        if robot2.x == robot1.x && robot2.y - 1 == robot1.y || robot2.y - 1 == limite_y {
                            println!("Collision ! du Robot <{}> aux  coordonnées x:{}, y:{}",robot2.id,robot2.x,robot2.y);
                        } else {
                            robot2.y = robot2.y - 1;
                        }
                    }
                }
                "L" => {
                    if robot2.orientation == "North"{
                        robot2.orientation = "West".to_string();
                    }
                    else if robot2.orientation == "West"{
                        robot1.orientation = "South".to_string();
                    }
                    else if robot2.orientation == "Est"{
                        robot2.orientation = "North".to_string();
                    }
                    else if robot2.orientation == "South"{
                        robot2.orientation = "Est".to_string();
                    }
                
                }
                "R" => {
                    if robot2.orientation == "North"{
                        robot2.orientation = "Est".to_string();
                    }
                    else if robot2.orientation == "West"{
                        robot2.orientation = "North".to_string();
                    }
                    else if robot2.orientation == "Est"{
                        robot2.orientation = "South".to_string();
                    }
                    else if robot2.orientation == "South"{
                        robot2.orientation = "West".to_string();
                    }
                    }
                _ => println!("Je reconnais pas votre lettre"),
            }
        }
        
    }

    println!("position du Robot<{}> final x : {} y : {}",robot1.id,robot1.x,robot1.y);
    println!("position du Robot<{}> final x : {} y : {}",robot2.id,robot2.x,robot2.y);

        
}
    
