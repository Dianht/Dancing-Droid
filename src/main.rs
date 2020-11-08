use colored::*;
pub mod party;
use party::display as D;
use party::normal_game as N_G;
use party::affichage as A;
fn main() {
    //Vecteur qui va accueillir le(s) robot(s)
    let mut robot = Vec::new();

    print!(
        "🤖 {} Dancing Droid 0.3.3 {} 🦾\n",
        "......".cyan(),
        "......".cyan()
    );
    println!(
        "\nVoulez-vous génerer un monde aléatoire ?\n{}/{} ?",
        "Y".green(),
        "N".red()
    );
    //En fonction de la réponse de l'utilisateur la taille du terrain
    //est soit donné par la fonction "random_world" soit donné par la fonction "file"
    let mut terrain = D::choice(&mut robot);
    let piste = (terrain.x,terrain.y);
    //Si le robot est vide, on ne lance pas le programme
    if robot.is_empty() {
        println!("Votre monde n'a pas de robot 🤡");
    } else {
        let mut obstacle = N_G::create_barrier(&mut terrain);
        D::display(&mut robot, &mut terrain);
        println!("========================\nEtat initial\n========================");
        D::initial_final(&mut robot, "initial".cyan().bold().to_string());
        A::affichage(&mut robot,piste,&mut obstacle);
        println!("========================\n🎶 └[∵┌]└[ ∵ ]┘[┐∵]┘ 🎶\n========================");
        N_G::game(&mut robot,terrain,&mut obstacle);
        println!("========================\nEtat final\n========================");
        D::initial_final(&mut robot, "finale".blue().bold().to_string());
        A::affichage(&mut robot,piste,&mut obstacle);

    }
}

