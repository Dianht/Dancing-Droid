use colored::*;
enum Orientation {
    North,
    West,
    South,
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
    orientation : Orientation,
}

fn collision(tmp_x : i32,tmp_y :i32,robot1 :&mut Robot,robot2 :&mut Robot,lim_x :i32,lim_y:i32){
    if robot1.x == robot2.x && robot1.y == robot2.y{
        println!("{} du robot<{}> aux coordonnée x : {} y : {}","Collision !".red(),robot1.id,robot1.x,robot2.y);
        robot1.x = tmp_x;
        robot1.y = tmp_y; 
    }
    else if robot1.x == lim_x + 1 || robot1.x < 0 || robot1.y == lim_y + 1|| robot1.y < 0{
        println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),robot1.id);
        robot1.x = tmp_x;
        robot1.y = tmp_y;
    }
}

fn game(instruction_robot : &Instruction,robot :&mut Robot){
    match instruction_robot{
        Instruction::F => {
            match robot.orientation{
                Orientation::North => robot.y = robot.y + 1,
                Orientation::West => robot.x = robot.x - 1,
                Orientation::Est => robot.x = robot.x + 1,
                Orientation::South => robot.y = robot.y - 1,
            }
        }
        Instruction::L => {
            match robot.orientation{
                Orientation::North  => robot.orientation =  Orientation::West,
                Orientation::West => robot.orientation = Orientation::South,
                Orientation::Est => robot.orientation = Orientation::North,
                Orientation::South => robot.orientation = Orientation::Est,
            }
        }
        Instruction::R => {
            match robot.orientation {
                Orientation::North => robot.orientation =  Orientation::Est,
                Orientation::West => robot.orientation = Orientation::North,
                Orientation::Est => robot.orientation = Orientation::South,
                Orientation::South => robot.orientation = Orientation::West,
            }
        }
    }
}   

fn main() {

    let f = &Instruction::F;
    let r = &Instruction::R;
    let l = &Instruction::L;
    let lim_x = 5;
    let lim_y = 5;
     
    
    let instruction_robot1 = vec![f,l,l,f,f,f];
    let instruction_robot2 = vec![r,f,l,f,r,r,f];
    
    let mut vecteur = vec![Robot{ id: 0,x: 3,y: 3,orientation: Orientation::South},
                             Robot{ id: 1,x: 3,y: 2,orientation: Orientation::West}];

    let (rb1,rb2) = vecteur.split_at_mut(1);    
    let  robot1 = &mut rb1[0];
    let  robot2 = &mut rb2[0];
    
    let mut tmp :(i32,i32);
    let taille = if instruction_robot1.len() > instruction_robot2.len(){instruction_robot1.len()}else {instruction_robot2.len()};


    for i in 0..taille{
        if i < instruction_robot1.len() {
            tmp = (robot1.x,robot1.y);
            game(instruction_robot1[i],robot1);
            collision(tmp.0,tmp.1,robot1,robot2,lim_x,lim_y);
        }
        if i < instruction_robot2.len() {
            tmp = (robot2.x,robot2.y);
            collision(tmp.0,tmp.1,robot2,robot1,lim_x,lim_y);
            game(instruction_robot2[i],robot2);
        }
    }
    println!("\n{} du {}<{}>  x : {} y : {}","Position finale".magenta(),"Robot".yellow(),robot1.id,robot1.x,robot1.y);
    println!("{} du {}<{}> final x : {} y : {}","Position finale".magenta(),"Robot".cyan(),robot2.id,robot2.x,robot2.y);
        
}
    
