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
struct Robot<'a> {
    id: i32,
    x: i32,
    y: i32,
    orientation : Orientation,
    instruction : Vec<&'a Instruction>,
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
    
    let mut vec = Vec::new();

    let rb = Robot{ id: 0,x: 1,y: 1,orientation: Orientation::North, instruction : vec![f,l,l,f,f,f]};
    vec.push(rb);
    let rb = Robot{ id: 1,x: 3,y: 2,orientation: Orientation::South,instruction : vec![r,f,l,f,r,r,f]};
    vec.push(rb);
   
    
    let mut tmp :(i32,i32);
    let taille = if instruction_robot1.len() > instruction_robot2.len(){instruction_robot1.len()}else {instruction_robot2.len()};

    for x in 0..taille{
        for i in 0..vec.len(){
            if x < vec[i].instruction.len() {
                game(vec[i].instruction[x],& mut vec[i]);
                
            }  
        }
    }
    
    println!("\n{} du {}<{}>  x : {} y : {}","Position finale".magenta(),"Robot".yellow(),vec[0].id,vec[0].x,vec[0].y);
    println!("{} du {}<{}> final x : {} y : {}","Position finale".magenta(),"Robot".cyan(),vec[1].id,vec[1].x,vec[0].y);
        
}







