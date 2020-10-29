use colored::*;//gadjet il faudra modifier le cargo.toml
use std::fmt; // On importe le module `fmt`
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;


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
fn file(mut robot :&mut Vec<Robot>) -> Terrain{
                                //Projet/DancingDroids/two_robots.txt
    let mut file = File::open("two_robots.txt").expect("Impossible de lire le fichier");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Impossible de lire le fichier");
    let c : Vec<&str> = s.split(|c| c == '\n' || c == ' ').collect();
    
    let mut m : Vec<&str> = Vec::new();
    let mut id : i32 = 0;

    let terrain = Terrain {x : c[0].parse::<i32>().unwrap(), y : c[1].parse::<i32>().unwrap(),};

    for i in 3..c.len(){
        if c[i] == "" {
            if m.len() == 4{
                create_robot(&mut robot,m.clone(),id,'N');
                id += 1;
                m.clear();
            }
            else if m.len() == 3{
                m.push(c[i]);
                create_robot(&mut robot,m.clone(),id,'O');
                id += 1;
                m.clear();
            }
            else {break;}
        }
        else {m.push(c[i]);}
    }

    return terrain;
}

fn create_robot(robot :&mut Vec<Robot>, c : Vec<&str>,id : i32,vide : char) {
    
    let mut robot_instruction : Vec<&Instruction> = Vec::new();
    let mut position : Vec<i32> = Vec::new();
    let mut orientation : Orientation = Orientation::North;
    
    for i in 0..2{
        match c[i].parse::<i32>() {
            Err(_) => (),
            _ => position.push(c[i].parse::<i32>().unwrap()),
        }
    }
    match c[2]{
            "N" => orientation = Orientation::North,
            "E" => orientation = Orientation::Est,
            "W" => orientation = Orientation::West,
            "S" => orientation = Orientation::South,
            _ => println!("C'est une Orientation ???? {}",c[2]),
            
    }
    if vide == 'N'{
        let instruction : Vec<char> = c[3].chars().collect();
        for i in 0..instruction.len() {
            match instruction[i] {
                'F' => robot_instruction.push(&Instruction::F),
                'R' => robot_instruction.push(&Instruction::R),
                'L' => robot_instruction.push(&Instruction::L),
                _ => {
                    println!("C'est une instruction ???? {} ",instruction[i]);
                    break;
                },
            } 
        }
    }
    else if vide == 'O' {
        let mut i : usize = 0;
        let mut rng= rand::thread_rng();
        let nbre_instruction = rng.gen_range(1, 10);
        while i != nbre_instruction {
            let aleatoire = rng.gen_range(0, 3);
            match aleatoire{

                0 => robot_instruction.push(&Instruction::F),
                1 => robot_instruction.push(&Instruction::R),
                2 => robot_instruction.push(&Instruction::L),
                _ => (),
                
            }
            i += 1;
        }
    }
    
    let y = position[0];
    let x = position[1];
    robot.push(Robot{id : id,x : x, y : y,orientation : orientation,instruction : robot_instruction, });
    
   
}

fn initial_final(robot :&mut Vec<Robot>,position : String){  
    
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
            if robot[m].id != robot[i].id{     
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
fn display(robot :&mut Vec<Robot>,terrain :&mut Terrain){
    
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

    let mut robot = Vec::new();
    let mut terrain = file(&mut robot);
    display(&mut robot,&mut terrain);
    initial_final(&mut robot,"Position initial".to_string());
    game(terrain.x,terrain.y,&mut robot);
    initial_final(&mut robot,"Position finale".to_string());

}


