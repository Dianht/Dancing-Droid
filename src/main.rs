use colored::*;
pub mod party;

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
    let mut terrain = party::display::choice(&mut robot);

    //Si le robot est vide, on ne lance pas le programme
    if robot.is_empty() {
        println!("Votre monde n'a pas de robot 🤡");
    } else {
        party::display::display(&mut robot, &mut terrain);
        println!("========================\nEtat initial\n========================");
        party::display::initial_final(&mut robot, "initial".cyan().bold().to_string());
        println!("========================\n🎶 └[∵┌]└[ ∵ ]┘[┐∵]┘ 🎶\n========================");
        party::normal_game::game(&mut robot, terrain);
        println!("========================\nEtat final\n========================");
        party::display::initial_final(&mut robot, "finale".blue().bold().to_string());
    }
}
