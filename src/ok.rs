use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction{
    F,L,R,
}
#[derive(Debug)]
enum Orientation{
    North,South,Est,West,
}

fn main(){



    let mut file = File::open("fichier.txt").expect("Impossible de lire le fichier");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Impossible de lire le fichier");

    println!("{}",s);

    
    let mut instruction : Vec<char> = Vec::new();
    let mut orientation : Orientation;
    let c : Vec<&str> = s.split(|c| c == '\n' || c == ' ')
    .collect();
    let mut position : Vec<i32> = Vec::new();
    let mut robot_instruction : Vec<Instruction> = Vec::new();
    let mut m : usize = 0;

    
    for i in 0..c.len(){
        match c[i] {
            "N" => rb[u].orientation = Orientation::North,
            "E" => rb[u].orientation = Orientation::Est,
            "W" => rb[u].orientation = Orientation::West,
            "S" => rb[u].orientation = Orientation::South,
            "" =>   {
                        instruction = c[i - 1].chars().collect();
                        for i in 0..instruction.len() {
                            match instruction[i] {
                                'F' => robot_instruction.push(&Instruction::F),
                                'R' => robot_instruction.push(&Instruction::R),
                                'L' => robot_instruction.push(&Instruction::L),
                                _ => (),
                            } 
                        } 
                        robot.push(rb[u]);
                        robot_instruction.clear();
                        position.clear();
                        instruction.clear();
                    }
            _ => match c[i].parse::<i32>() {
                Err(_) => (),
                _ => position.push(c[i].parse::<i32>().unwrap())
            }
        } 
    }
    

    println!("{:?}",position);        
    println!("{:?}",robot_instruction);

    
/*
    let mut vec =  Vec::new();
    let mut x  =  Vec::new();
    let mut orientation : Orientation;
    
    'global: for i in 0..v.len(){
        match v[i]{
            'N' => orientation = Orientation::North,
            ' ' => (),
            '\n' => for m in i+1..v.len(){
                        match v[m] {
                            'F' => vec.push("F"),
                            'L' => vec.push("L"),
                            'R' => vec.push("R"),
                            '\n' => break 'global,
                            _ => (),
                        }
                    } 
            _ => match v[i].to_digit(10) {
                None => (),
                _ => x.push(v[i].to_digit(10)),
            }
        }
    }
    
    if let Some(m) = &x[0] {
        println!("{}", *m);
    }
    for i in 0..x.len(){
        println!{"{:?}",x[i]};
    }
    for i in 0..vec.len(){
        println!("{}",vec[i]);
    }
    //println!("{:?}",vec);
 */
}