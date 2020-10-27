use colored::*; //gadjet il faudra modifier le cargo.toml
use std::fmt; // On importe le module `fmt`

#[derive(Debug)]
enum Orientation {
    North,
    West,
    South,
    Est,
}
#[derive(Debug)]
enum Instruction {
    F,
    L,
    R,
}
#[derive(Debug)]
struct Robot<'a> {
    id: i32,
    x: i32,
    y: i32,
    orientation : Orientation,
    instruction : Vec<&'a Instruction>,
}

struct Terrain {
    x : i32,
    y : i32,
}


fn initial_final(robot :&mut Vec<Robot>,position : String){  
    println!("\n");
    for i in 0..robot.len(){
        println!("{} du {}<{}>  x : {} y : {}",position.magenta(),"Robot".blue(),robot[i].id,robot[i].x,robot[i].y);
    }
    println!("\n");
}

fn game(lim_x : i32,lim_y : i32,mut robot :&mut Vec<Robot>){
    
    //On trouve le grand nombre d'instruction qu'a un robots
    let mut taille = robot[0].instruction.len();
    for i in 0..robot.len() - 1{
        if taille > robot[i + 1].instruction.len(){
            taille = robot[i].instruction.len();
        }
        else {
            taille = robot[i + 1].instruction.len()
        }
    }
    // Nous permetra de stocker les valeurs des coordonées du robot avant l'instruction
    let mut tmp :(i32,i32);
    //Une boucle jusqu'à que le robot avec le + d'instruction n'aura plus d'instruction
    for x in 0..taille{       
                              
        for i in 0..robot.len(){              //Une boucle qui fera jouer les robots un par un
            if x < robot[i].instruction.len() {           //On ignorera les Robot qui n'ont plus d'instruction
                tmp = (robot[i].x,robot[i].y);
                instruction(robot[i].instruction[x],& mut robot[i]);                               
                collision(tmp.0,tmp.1,&mut robot, lim_y,lim_x,i);  
            }  
        }
    }
}

fn collision(tmp_x : i32,tmp_y :i32,robot :&mut Vec<Robot>,lim_y : i32,lim_x :i32,m : usize){

    for i in 0..robot.len(){

        if robot[m].x == robot[i].x && robot[m].y == robot[i].y{
            if robot[m].id != robot[i].id{     //hehe
                println!("{} 💥\n{}<{}> fonce vers {}<{}> aux coordonnée x : {} y : {} !","Collision !".red(),"Le Robot".cyan(),robot[m].id,"le Robot".green(),robot[i].id,robot[m].x,robot[m].y);
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        }

        else if robot[m].x == lim_x + 1 || robot[m].x < 0 || robot[m].y == lim_y + 1|| robot[m].y < 0{
            if robot[m].id != robot[i].id{ 
                println!("{} 🚧\n{}<{}> se dirige vers les limites de la map !","Attention !".red(),"Le Robot".yellow(),robot[m].id);
                robot[m].x = tmp_x;
                robot[m].y = tmp_y;
            }
        }  
    }
}

fn instruction(instruction_robot : &Instruction,robot :&mut Robot){
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
fn display(robot : Vec<Robot>,terrain : Terrain){
    println!("Terrain {{ {} }}",terrain);

    println!("Robots {{");
    for i in 0..robot.len(){
        println!("  {{ {}, }}",robot[i]);
    }
    println!("}}");
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x_max = {}, y_max = {}", self.x, self.y)
    }
}

impl <'a> fmt::Display for Robot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "id = {}, x = {}; y = {}, orientation = {:?}, instruction = {:?}", self.id, self.x,self.y,self.orientation,self.instruction)
    }
}
fn main() {

    let f = &Instruction::F;
    let r = &Instruction::R;
    let l = &Instruction::L;
    let terrain = Terrain {x : 5,y : 8};
    
    let mut robot = Vec::new();

    // Faire une boucle jusqu'a qu'il y a plus d'instruction dans le fichier
    let rb = Robot{ id: 0,x: 1,y: 1,orientation: Orientation::North, instruction : vec![l,f,f,f,f,f]};
    robot.push(rb);
    let rb = Robot{ id: 1,x: 1,y: 2,orientation: Orientation::South,instruction : vec![f,f,l,f,r,r,f]};
    robot.push(rb);
    let rb = Robot{ id: 2,x: 4,y: 1,orientation: Orientation::West,instruction : vec![f,r,r,f,l,f,f,f,l,r]};
    robot.push(rb);
    let rb = Robot{ id: 3,x: 1,y: 0,orientation: Orientation::West,instruction : vec![f,l,f,f,r,r,r,f,l,f]};
    robot.push(rb);
    //

    initial_final(&mut robot,"Position initial".to_string());
    game(terrain.x,terrain.y,&mut robot);
    initial_final(&mut robot,"Position finale".to_string());
    display(robot,terrain);
    
    

        
}







