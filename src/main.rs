use colored::*;
pub mod party;

fn main() {
    let mut robot = Vec::new();

    print!(
        "🤖 {} Dancing Droid 0.3.1 {} 🦾\n",
        "......".cyan(),
        "......".cyan()
    );
    println!(
        "\nVoulez-vous génerer un monde aléatoire ?\n{}/{} ?",
        "Y".green(),
        "N".red()
    );
    let mut terrain = party::display::choice(&mut robot);

    //Si le robot est vide, on ne lance pas le programme
    if robot.is_empty() {
        println!("Votre monde n'a pas de robot 🤡");
    } else {
        party::display::display(&mut robot, &mut terrain);
        party::display::initial_final(&mut robot, "Position initial".to_string());
        party::normal_game::game(terrain.x, terrain.y, &mut robot);
        party::display::initial_final(&mut robot, "Position finale".to_string());
    }
}
