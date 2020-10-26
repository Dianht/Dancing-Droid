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

fn collision(tmp_x : i32,tmp_y :i32,vec :&mut Vec<Robot>,lim_y : i32,lim_x :i32,m : usize){
    //Il faut trouver une autre solution (la boucle en bas va comparer au moins une fois la position du robot m avec elle meme)
    let mut une = 0;        
    for i in 0..vec.len(){
        if vec[m].x == vec[i].x && vec[m].y == vec[i].y{
            if une > 0{     //hehe
                println!("{} du robot<{}> aux coordonnée x : {} y : {}","Collision !".red(),vec[m].id,vec[m].x,vec[m].y);
                vec[m].x = tmp_x;
                vec[m].x = tmp_y;
            }
            une += 1;   //hehe
        }
        else if vec[m].x == lim_x + 1 || vec[m].x < 0 || vec[m].y == lim_y + 1|| vec[m].y < 0{
            if une > 0{ //hehe
                println!("{}  le Robot<{}>! se dirige vers les limites de la map 😮 !","Attention !".red(),vec[m].id);
                vec[m].x = tmp_x;
                vec[m].y = tmp_y;
            }
            une += 1;
        }  
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
    
    let mut vec = Vec::new();

    // Faire une boucle jusqu'a qu'il y a plus d'instruction dans le fichier
    let rb = Robot{ id: 0,x: 1,y: 1,orientation: Orientation::North, instruction : vec![f,l,l,f,f,f]};
    vec.push(rb);
    let rb = Robot{ id: 1,x: 3,y: 2,orientation: Orientation::South,instruction : vec![r,f,l,f,r,r,f]};
    vec.push(rb);
    let rb = Robot{ id: 2,x: 4,y: 1,orientation: Orientation::West,instruction : vec![f,r,r,f,l,f,f,f,l,r]};
    vec.push(rb);
   
    
    let mut tmp :(i32,i32);

    let mut taille = vec[0].instruction.len();
    for i in 0..vec.len() - 1{
        if taille > vec[i + 1].instruction.len(){
            taille = vec[i].instruction.len();
        }
        else {
            taille = vec[i + 1].instruction.len()
        }
    }

    for x in 0..taille{
        for i in 0..vec.len(){
            if x < vec[i].instruction.len() {
                tmp = (vec[i].x,vec[i].y);
                game(vec[i].instruction[x],& mut vec[i]);                               
                collision(tmp.0,tmp.1,&mut vec, lim_y,lim_x,i);  
            }  
        }
    }
    
    for i in 0..vec.len(){
        println!("\n{} du {}<{}>  x : {} y : {}","Position finale".magenta(),"Robot".yellow(),vec[i].id,vec[i].x,vec[i].y);
    }
        
}







