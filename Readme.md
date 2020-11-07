# Projet DancingDroids

/!\ Attention sujet non fini!!! prevoir des changements /!\

Dans ce projet nous allons réaliser un jeu joué par l'ordinateur dans
un premier temps depuis un fichier predefini.

## Groupes

Compositions des groupes maximum 3 personnes, tentez de pas vous mettre à deux personnes
très à l'aise le but de se projet de de faire coder et progresser tout le monde. ;)

## Rendu

*Nouvelles dates de rendu !*

- L2 A-B: Dimanche 08 novembre 2020 à 18:00 pile heure de Paris

Procédure d'annonce des groupes:

- [L2-A](https://lite.framacalc.org/9juxgroupesrustrendur1414)
- [L2-B](https://lite.framacalc.org/9jux-rendurustgroupes)

Pensez à faire un `LISEZ_MOI.md` pour noter les détails fun, ce qui vous a plu,
ce que vous aurriez aimer faire etc. Ou pour raconter des détails de réalisation.

### Usages de dependances avec Cargo

Autant que besoin, mais attention les dependances c'est avec modérations et si vous
sentez que c'est neccessaire. Ça me derange pas mais faut comprendre ce qu'on fait! ;)

### Triche ou Plaggiat

Plaggiat ou triche: note entre 0 et 5. C'est facile de voir ça avec git et je ferais
une recherche avec de quoi motiver la note. Bref trichez pas. 😎

#### Partage de code: Partage != Plaggiat

Supposons vous avez emprunter la magnifique fonction du groupe **Matrix Squirrel**,
Notez explicitement toute ces situations un commentaire de la forme:

`// REMIX: explication + améliorations que vous auriez faites`

Je serais surement plus conciliant que si je constate du plaggiat.
Remixer n'est pas plaggier.

> Note: Quite a faire du gros partage faites une bibliothèque et partagez la... :O

## Partie obligatoire

Afin d'avoir au moins 10 il est neccessaire de faire toutes les parties indispensables.
C'est à dire TOUTE la version 0.1.0. J'incite cependant tout le monde a tenter des
améliorations une fois que vous avez un code qui fait ce qui est demandé.

Le rendu sera un fork du projet versionné sous git. Pour ce faire vous
pouvez forker ce dépot et faire vos contributions dans votre fork avec votre groupe.

A vous de gérer les droits pour pouvoir facilement editer et push a deux. Les dépots seront
publiques, mais je veillerais a ce que vous n'ayez pas de plaggiat personnellement. :)

L'entre-aide est tolérée surtout pour les bonus! Happy hacking!

## Version 0.1.0 Déplacements et collisions fonctionnelles

## Méthodologie: Lire le sujet, établir des étapes de réalisation

**TooLong;Don'tRead**: Ne sautez pas direct sur le code, il vous faut un plan.

Dans tout projet il faut un plan. Pour avoir un plan, il faut lire les consignes
ou les spécifications, qui décrivent ce qu'on attends de vous.

Ensuite découper en étapes résolvable facilement exemple:

- Écrire une `enum Orientation` qui gére les 4 cas possibles
- Écrire une fonction qui lit un caractères `char` et crée une `Orientation`
- Écrire un [commentaire de documentation](https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html) de cette fonction qui dit comment l'utiliser
- Écrire des [tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) pour vérifer que votre fonction fait ce que vous voulez.

Si besoin, décrivez des choses simples et bien découpées en français, ou faites un schema et partagez le avec votre
équipe.

Pour vous organiser et suivre les choses à faire entre vous il y a les **Issues** de gitlab.

## Deplacement orientation

Dans ce jeu des robots vont pouvoir se déplacer dans un espace en deux
dimensions. A chaque tour un robot executera un ordre les ordres concistent en
effectuer une rotation à droite ou à gauche ou avancer ou ne rien faire!

> Conseil: Deux type `enum` vous serons utile! ;) Une pour l'orientation, une pour
> representer les instructions!

Dans le fichier d'instructions vous aurrez les caractères suivants possibles:

- `L`: Tourne le robot à gauche par exemple passe de orientation: `N` à `W`
- `R`: Tourne le robot à droite par exemple passe de orientation: `N` à `E`
- `F`: Avance dans le sens de l'orientation d'une case

## Collisions

En cas de collision avec un autre robot lors d'un déplacement le robot devra
dire sur la sortie standard:

Cas des collisions: Faire dire `"Robot ID<numId> Collision en (x, y)"` et ne
pas comptabiliser le mouvement du Robot qui occassionne la collision, mais
consommer son instruction.

Format du fichier definisant le monde:

**Important** : Dans le format de fichier la gestion des commentaires `\\` est optionnelle .
```txt
5 5   // X_max Y_max

1 1 N // position du robot en x=1 y=1 orientation = nord
FLLFRF

3 2 S // position du robot 2 x=3 y=2 orientation=South
FFLFRRF
```

Pour representer:

- un robot une simple structure suffira.
- La structure pour contenir les robots pourras être un `Vec<Robot>`

### Tests et Documentation

Écrire des tests de votre code comme indiqué dans cette partie du livre
[ch11 tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) ou de [Rust By Example](https://doc.rust-lang.org/rust-by-example/testing.html)

Il est aussi attendu que vous ayez documenter votre code avec les commentaires de
documentation `\\\`. Les Tests et la documentation font parti de l'evaluation.
Dans le doute demandez moi mon avis!

### Conclusion version 0.1.0

Dans certe première itération, vous devrez pouvoir faire la simulation des robots,
depuis un fichier d'instruction, et afficher a la fin leurs positions finales.

## Version 0.2.0 Affichage implementation de Display

Dans cette version on va écrire du code pour afficher la positions des robots et
dessiner la grille.

Vous devrez pouvoir dessiner quelque chose comme cela en partant de la grille
donnée en exemple plus haut dans votre terminal:

```txt
Terrain { x_max = 5; y_max = 5 }
Robots [
 { id = 0, x = 1; y = 1; orientation: North, instructions: [F,L,L,F,R,F], },
 { id = 1; x = 3; y = 2; orientation: South, instructions: [F,F,L,F,R,R,F], },
]
Etat initial
============================
5 .  .  .  .  .  .
4 .  .  .  .  .  .
3 .  .  .  .  .  .
2 .  .  .  ⬇  .  .
1 .  ⬆  .  .  .  .
0 .  .  .  .  .  .
  0  1  2  3  4  5
Etat final
=============================
5 .  .  .  .  .  .
4 .  .  .  .  .  .
3 .  .  .  .  .  .
2 .  .  .  .  .  .
1 ⬅  .  .  .  .  .
0 .  .  .  ⬅  .  .
  0  1  2  3  4  5
```

Libre a vous d'adapter l'affichage tant que cela reste lisible.

Pour réaliser cela on peut implementer un trait pour nos structures qui s'appelle
`Display` c'est un peu comme un contrat, qui garanti que si une structure,
l'implemente alors on peut avoir une representation humaine sous forme de `String`.

Vous pouvez implementer aussi `Debug` pour vous faciliter la lecture en debuggant
votre code.

## version 0.3.0 : un peu d'aléatoire

Programmer ce qui est neccessaire pour que a chaque *tick* (pas discret d'avancement du monde)
un robot fait une instruction de façon aléatoire. Sa liste d'instruction serait symbolisée
par la liste d'instructions vide au parsing.

## 0.3.1 gen-world

Écrire une option de la ligne de commande qui lance lance un jeu avec un monde généré
avec aléatoirement: X_max, Y_max et positions et listes d'instructions de Robots,
inclus.

## 0.3.3 Ajouter des obstacles sur le plateau de jeu

Les robots ont maintenant sur piste de dance des obstacles a vous de choisir si
les obstacles bloquent ou auraient des propriétées amusantes.

## version 0.4.0 : un peu de couleurs

A présent les robots lorsque ils se déplacent laissent une couleur sur les cases
qu'ils ont traversée, les couleurs sont determinées à partir de l'id d'un robot.

Les couleurs seront des couleurs affichables en terminal, à vous d'ecrire une
fonction qui va des id vers les couleurs. indice: fonction de hashage, trait `Hash`.

Une couleur se degrade progressivement sur 5 tours et disparait elle reste destructrice jusque a
sa disparition totale.

Si un robot traverse la couleur d'un autre robot il est mis hors service, il est
hors jeu pour le reste de la partie et sera symbolisé par `'🤖'` par exemple il
reste un element dans lequel les autres robots peuvent entrer en collision.

La collision entre deux robots n'implique pas de mise hors service.

Programmer ce qui est neccessaire pour que a chaque *tick* de façon aléatoire un
robot avance sa liste d'instruction serait symbolisée par simplement aucune liste
d'instructions.

Pour vous aider avec les couleurs vous pouvez utiliser la crate [termion](https://lib.rs/crates/termion)
ou [colored](https://crates.io/crates/colored).

## version 0.5.0 : Paralellization des Robot'z

A présent dans ce bonus, faites le neccessaire pour pouvoir avoir plusieurs robots
qui agisent en parallèlle, vous devrez découvrir l'usage des threads, channels,
`Mutex`, `Arc` ou bien utiliser une bibliothèque comme [Rayon](https://docs.rs/rayon/1.4.1/rayon/) qui abstrait cela.

Voici le chapitre du livre sur le sujet [FearLess concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html).

Un petit exercice pour faire des threads est dans exercices/

## version 0.6.0 : De-Serialisation/Serialisation de l'etat du monde

Utilisez une bibliothèque comme [Serde](https://github.com/serde-rs/serde)
pour écrire sur un fichier l'etat des structures de données du jeu pour par
exemple arretter une partie en cas de `Ctrl-C`.

## Version 0.7.0 : Gestion du parsing rationnelle

Utilisez une bibliothèque de votre choix pour gérer le parsing le format de fichier des robots.

Telle que :

- [nom](https://github.com/Geal/nom)
- [pest](https://github.com/pest-parser/pest)
- [lalrpop](https://github.com/lalrpop/lalrpop)

## Version 0.8.0 : Industrialisation et lib de jeu vidéo

Choissisez une bibliothèque pour faire des jeux vidéo en Rust et faites vous plaisir
pour améliorer avec de la gestion des assets, 3D ou autre dans votre projet si vous en êtes
la je peux plus vous retenir.

Site pour choisir ses lib/framework de jeu: <https://arewegameyet.rs/>

Conseil de game-engine:

- [Bevy](https://bevyengine.org/)
- [Amethyst.rs](https://amethyst.rs/)

## Version 0.9.0 : Vous avez dit IA

Programmer ou entrainner une petite IA pour ce jeu. C'est pas simple c'est du gros
bonus. ;)
Le but d'une IA est de survivre en envoyant des commandes pour eviter d'être mise hors
service. Plusieurs IA devraient pouvoir jouer ensemble.
