use std::env;
use rand::Rng;

const CHARACTERS : [&str; 7] = ["Professor", "Mayor", "Courier", "Scientist", "Archeologist", "Explorer", "Inspector"];
const MONSTERS : [&str; 6] = ["Dracula", "The Creature", "Frankenstein and the Bride", "The Invisible Man", "The Mummy", "Wolfman"];


fn pick_rand<'a>(options : &'a[&'a str], npicks : i32) -> Vec<&'a str> {
    let mut options_vector : Vec<&str> = options.to_vec();
    let mut output_vector : Vec<&str> = vec![];
    let mut rng = rand::rng();

    for _n in 0..npicks {
        let rand_index = rng.random_range(0..options_vector.len());
        output_vector.push(options_vector.swap_remove(rand_index));
    }
    return output_vector
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let nplayers : i32 = args[1].parse().unwrap();
    if nplayers > 5 {
        panic!("Maximum number of players is 5");
    }

    let nmonsters : i32 = args[2].parse().unwrap();
    if nmonsters < 2 {
        panic!("Minimum number of monsters is 2");
    }
    if nmonsters > 4 {
        panic!("Maximum number of monsters is 4");
    }



    // Should validate input parameters

    let players = pick_rand(&CHARACTERS, nplayers);
    let monsters = pick_rand(&MONSTERS, nmonsters);
    

    for (i, p) in players.iter().enumerate() {
        println!("Player {} will play as {}", i+1, p);
    }

    println!("Monsters are: ");
    for m in monsters {
        println!(" - {}", m);
    }

}