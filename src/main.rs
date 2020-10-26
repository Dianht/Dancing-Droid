use colored::*;
use std::fmt; // On importe le module `fmt`
use std::fmt::Debug;

#[derive(Debug)]
struct Robot {
    id: i32,
    x: i32,
    y: i32,
    orientation : char,
    instruction : Vec<char>,
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({}, {},  {})", self.id, self.x,self.y)
    }
}
#[derive(Debug)]
struct Terrain {
    x : i32,
    y : i32,
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({}, {})", self.x, self.y)
    }
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

fn instruction(instruction_robot : char,robot :&mut Robot){
    match instruction_robot{
        'F' => {
            match robot.orientation{
                'N' => robot.y = robot.y + 1,
                'W' => robot.x = robot.x - 1,
                'E' => robot.x = robot.x + 1,
                'S' => robot.y = robot.y - 1,
                _ => println!("ok"),
            }
        }
        'L' => {
            match robot.orientation{
                'N'  => robot.orientation =  'W',
                'W' => robot.orientation = 'S',
                'E' => robot.orientation = 'N',
                'S' => robot.orientation = 'E',
                _ => println!("ok"),
            }
        }
        'R' => {
            match robot.orientation {
                'N' => robot.orientation =  'E',
                'W' => robot.orientation = 'N',
                'E' => robot.orientation = 'S',
                'S' => robot.orientation = 'W',
                _ => println!("ok"),
            }
        }
        _ => {
            println!("ok");
        }
    }
}   

fn main() {

    let limite = Terrain{ x : 5,y : 5};


    
    let mut robot = Vec::new();

    // Faire une boucle jusqu'a qu'il y a plus d'instruction dans le fichier
    let rb = Robot{ id: 0,x: 1,y: 1,orientation: 'N', instruction : vec!['L','F','F','F','F','F']};
    robot.push(rb);
    let rb = Robot{ id: 1,x: 1,y: 2,orientation: 'S',instruction : vec!['F','F','L','F','R','R','F']};
    robot.push(rb);
    let rb = Robot{ id: 2,x: 4,y: 1,orientation: 'W',instruction : vec!['F','R','R','F','L','F','F','F','L','R']};
    robot.push(rb);
    let rb = Robot{ id: 3,x: 1,y: 0,orientation: 'E',instruction : vec!['F','L','F','F','R','R','R','F','L','F']};
    robot.push(rb);
    //

    initial_final(&mut robot,"Position initial".to_string());
    game(limite.x,limite.y,&mut robot);
    initial_final(&mut robot,"Position finale".to_string());

    println!("{:?}",limite);
    println!("{:?}",robot);
    
        
}







