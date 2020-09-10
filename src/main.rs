use std::io;
use std::fs;

#[macro_use] extern crate serde_derive;


// Constants
const JSON_FILENAME: &str = "wods.json";

#[derive(Serialize, Deserialize, Debug)]
struct Wod { 
    exercises: Vec<Movement>,
    date : String,
    comment : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Movement {
    reps: u32,
    weight: u32,
    name: String,
}

fn read_exercice() -> Movement {
    println!("Please input the movement name:");

    let mut mov_name = String::new();
    io::stdin()
        .read_line(&mut mov_name)
        .expect("Failed to read line");
    // .pop to remove the last charcter. Here it's \n
    mov_name.pop();

    let mut reps_input = String::new();

    //TODO : do not trust input ! Fail gracefuly if parsing number is not possible.
    println!("How many reps ?");
    io::stdin()
        .read_line(&mut reps_input)
        .expect("Failed to read line");

    let reps = reps_input.trim().parse().unwrap();

    let mut weight_input = String::new();

    println!("What weight ?");
    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read line");

    let weight = weight_input.trim().parse().unwrap();

    let move1 = Movement {
        reps: reps,
        weight: weight,
        name: mov_name
    };


    println!("Movement entered :");
    println!("{} - {} reps, with {} Kg.", move1.name, move1.reps, move1.weight);

    move1
}

fn read_wod() -> Wod {
    
    println!("How many exercises did you do ?");

    let mut exercises_number_input = String::new();
    io::stdin()
        .read_line(&mut exercises_number_input)
        .expect("Failed to read line");
    let exercises_number = exercises_number_input.trim().parse().unwrap();


    let mut exercises_vector: Vec<Movement> = Vec::new();
    for _ex in 0..exercises_number {
        
        exercises_vector.push(read_exercice());
    }

    println!("Total exercises entered : {}", exercises_vector.len());

    
    println!("Care to add a comment about that WOD ?");
    let mut comment = String::new();
    io::stdin()
        .read_line(&mut comment)
        .expect("Failed to read line");
    comment.pop();

    //Todo handle date
    let wod = Wod {
        exercises : exercises_vector,
        date: String::from("today"),
        comment : comment
    };
    
    wod
}

// Todo handle result, success or fail ?
fn save_json(data: Vec<Wod>, filename: &str) {

    println!("Saving wod in {}", filename);
    let serialized = serde_json::to_string(&data).unwrap();

    fs::write(filename, serialized)
    .expect("Unable to write file");
}

fn load_json(filename: &str) -> Vec<Wod> {
    
    println!("Importing data from {}", filename);
    let serialized_data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let wods: Vec<Wod> = serde_json::from_str(&serialized_data).unwrap();
    wods
}


fn main() {

    println!("Welcome to the rudimentary WOD logger!");

    //TODO : if the file doesn't exist or cannot be desialized, an empty Vec should be initialised.
    let mut wods = load_json(JSON_FILENAME);
    println!("{} wods are logged.", wods.len());

    let mut another = true;

    while another {
        
        let wod = read_wod();
        wods.push(wod);

        println!("Do you want to add another workout? [y/n]");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer.pop();

        another = answer == "y";
    }

    //todo wod recap
    println!("Well done !");

    save_json(wods, JSON_FILENAME)
}
