use crate::party;
use std::convert::TryInto;
use colored::*;

use party::Robot as R;
use party::Orientation as O;
use party::Obstacle as OB;
    
pub fn affichage(robot : &mut Vec<R>,terrain : (i32,i32),obstacle :&mut Vec<OB>) {
    //pour l'affichage on utilise une grille avec tous les points et des flèches qui représentent les robots
    let mut le_vecteur: Vec<String> = Vec::new(); 
    for _i in 0..=terrain.0 { //on remplit toute notre grille avec des points
        for _j in 0..=terrain.1 {
            le_vecteur.push("•".magenta().bold().to_string());
        }
    }
    for bot in robot{
        //il y a un cas où un robot butant sur un obstacle sort des limites du terrain on exclue donc ce cas
        if bot.x != -2 {
        //on change les points en flèches en fonction de la position des robots et de la taille du terrain
            let a: usize = ((bot.x)+(bot.y)*(terrain.0 + 1)).try_into().unwrap();
            le_vecteur[a] = match bot.orientation {
                O::North => "🢁".yellow().to_string(), // on choisit les flèches en fonction de l'orientation 
                O::West => "🡸".green().to_string(),
                O::Est => "🢂".red().to_string(),
                O::South => "🢃".blue().to_string(),
            };
        }
    }

    for ob in obstacle{
        let a: usize = ((ob.x)+(ob.y)*(terrain.0 + 1)).try_into().unwrap();
        le_vecteur[a] = match ob.id {
            0 => "♫".black().bold().to_string(), 
            1 => "✘".black().bold().to_string(),
            2 => "❂".black().bold().to_string(),
            _ => "?".to_string(),
           
        };

    }

    
        let mut compteur_case = 0; //ce compteur représente le numéro de la case courante
        let mut compteur_ligne_inverse = terrain.1; //ce compteur compte les lignes en partant de la fin
        let mut compteur_ligne = 0; //ce compteur compte le nombre de lignes parcourues
        let mut compteur_colonne = 0; //ce compteur indique à quelle colonne nous en sommes 
    
        print!("{}", "   ");
        for i in 0..(terrain.0 + 1) { //on affiche le numéro des colonnes
            print!("{}", i);
            print!("{}", "     ");
        }
        println!("{}", "");
        for v in &le_vecteur { //on parcourt la grille pour l'afficher
            compteur_colonne = compteur_colonne + 1; //on change de colonne
            if compteur_ligne < 10 { //tant qu'on a un nombre de ligne inférieur à 10 l'espace entre les numéros des 
                print!("{}", " ");
            }
            if compteur_case % (terrain.0 + 1) == 0 {
                print!("{}", terrain.1 - compteur_ligne_inverse); // on note le numéro de la ligne 
                print!("{}", " "); //espace pour faire jolie
                compteur_ligne_inverse = compteur_ligne_inverse - 1; //on baisse ce compteur pour afficher un numéro plus élevé au début de la ligne
                compteur_ligne = compteur_ligne + 1; //on augmente le numéro de la ligne pour le tour suivant
            }
            print!("{}", v);
            if compteur_ligne < 10 {
                print!("{}", "    ");// espace entre les points
            } else {
                print!("{}", "     ");// si il y a plus de 10 colonnes les points sont décalés on rajoute donc un obstacle
            }
            if compteur_colonne > 10 {
                print!("{}", " "); //si il y a plus de 10 colonnes les numéros des colonnes sont décalés
            } 
            if ((compteur_case + 1) % (terrain.0 + 1) == 0) && (compteur_case != 0) {
                print!("{}", '\n'); //au bout de la ligne on retourne à la ligne 
                compteur_colonne = 0; //on réinitialise donc le numéro de la colonne
            }

            compteur_case = compteur_case + 1; //on augmente le numéro de la case
            
        }
        println!("{}", "");
   
}    
    