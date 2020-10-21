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

fn game(instruction_robot : &Instruction,robot : &mut Robot,mut direction : &Orientation){
    
        match instruction_robot{
            Instruction::F=>{
                match direction {
                    Orientation::North => {
                        robot.y = robot.y + 1;
                    }
                    Orientation::West => {
                        robot.x = robot.x - 1;
                    }
                    Orientation::Est => {
                        robot.x = robot.x + 1
                    }
                    Orientation::South => {
                        robot.y = robot.y - 1;
                    }
                }
            }
            Instruction::L => {
                match direction {
                    Orientation::North => {
                        direction = &Orientation::West;
                    }
                    Orientation::West => {
                        direction = &Orientation::South;
                    }
                    Orientation::Est => {
                        direction = &Orientation::North;
                    }
                    Orientation::South => {
                        direction = &Orientation::Est;
                    }
                }
            }
            Instruction::R => {
                match direction {
                    Orientation::North => {
                        direction = &Orientation::Est;
                    }
                    Orientation::West => {
                        direction = &Orientation::North;
                    }
                    Orientation::Est => {
                        direction = &Orientation::South;
                    }
                    Orientation::South => {
                        direction = &Orientation::West;
                    }
    
                }
            }
        }
    
    
}

fn main() {

    let F = &Instruction::F;
    let R = &Instruction::R;
    let L = &Instruction::L;
    
    let mut direction_robot1  = &Orientation::North;
    let mut direction_robot2 = Orientation::South;

    let instruction_robot1 = vec![F,L,L,F,R,F];
    let instruction_robot2 = vec![F,F,L,F,R,R,F];
    
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

    for i in 0..5{
        game(instruction_robot1[i],robot1,direction_robot1);

    }

    


    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".magenta(),robot1.id,robot1.x,robot1.y);
    println!("{} du {}<{}> final x : {} y : {}","Position".blue(),"Robot".cyan(),robot2.id,robot2.x,robot2.y);
        
}
    
