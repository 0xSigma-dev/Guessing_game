use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;
use std::cmp::Ordering;
use std::env;
use std::io;
use std::fs;
use uuid::Uuid;
use rand;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    id: String,
    name: String,
    level: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    players: Vec<Player>
}


impl Game {
    fn new() -> Self {
        Self { players: Vec::new() }
    }

    fn add_player(&mut self, name: &str) {
        let name2 = name.to_string();
        let id = Uuid::new_v4().to_string();
        self.players.push(Player {id, name: name2, level: 1});
    }
    
    fn resume_game(&self, ide: &str) -> Option<Player> {
        for player in self.players.iter() {
            if player.id == ide {
                println!("Player found: {} - Starting from Level: {}", player.name, player.level);
                return Some(player.clone());
            }
        }
        None
    }

    fn remove_player(&mut self, id:&str) {
        for (index, player) in self.players.iter().enumerate() {
            if player.id == id {
                self.players.remove(index);
                return
            }
        }
    }

    fn level_up(&mut self, id: &str, level: i32) {
        for player in self.players.iter_mut() {
            if player.id == *id {
                player.level = level   
            }
        }
    }

    fn save_to_file(&self, filename: &str) {
        let data = serde_json::to_string_pretty(&self).expect("Cannot Serialize data");
        fs::write(filename, data).expect("Unable to write to file")
    }

    fn load_from_file(filename: &str) -> Self {
        if let Ok(data) = fs::read_to_string(filename) {
            serde_json::from_str(&data).unwrap_or_else(|_| Game::new())
        } else {
            Game::new()
        }
    }

    fn list_players(&self) {
        for (index, player) in self.players.iter().enumerate() {
            //let current = if player.id == id {"You"} else {"Other"};
            println!("{} {} - {} Level: {}", index + 1, player.id, player.name, player.level)
        }
    }

    fn fetchid(&self, name: &str) -> Option<String>{
        for player in self.players.iter() {
            if player.name == *name {
               let id = player.id.clone();
               return Some(id)
            }
         
        } 
        None
    }

}

fn main() {
    let filename = "players.json";
    let mut level;
    let mut random ;
    
    let mut game = Game::load_from_file(filename);

    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        println!("Usage: --  new - to start a new game
            resume - to resume from current level
            list - to list players
            remove - to remove player");
    }


    match args[1].as_str() {
        "new" => {
            println!("Welcome to the guessing game");
            println!("Please Enter your name below");
            level = 1;
            random = generate_number(level);
            
            
            let mut ide = String::new();
            let mut name = String::new();
            

            io::stdin().read_line(&mut name).expect("unable to read lines");
            let name = name.trim();
            game.add_player(&name);
            
            if let Some(id) = game.fetchid(&name) {
                ide = id;
            };
            game.save_to_file(filename);

            println!("Welcome {}", name);
            

            loop {
                
                println!("The Game will start now - Level {}", level);
                println!("A random number has been printed. Enter your guess below");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Unable to read guess");
                println!("guess: {}", guess);

                let guess: i32 = guess.trim().parse().expect("Please input a number");

                        match guess.cmp(&random) {
                            Ordering::Less => {
                                println!("Oops wrong guess. Your guess is lesser than the generated number");
                                println!("Please try again");
                                continue;
                            },
                            Ordering::Equal => {
                                println!("Wow you got it correctly.");
                                println!("You just levelled up");
                                level = level + 1;
                                game.level_up(&ide, level);
                                game.save_to_file(filename);
                                random = generate_number(level);
                                continue;

                            },
                            Ordering::Greater => {
                                println!("Oops wrong guess. Your guess is greater than the generated number");
                                println!("Please try again");
                                continue;

                            }

                        }

                    }
        },
        "resume" => {
            if args.len() < 3 {
                println!("please provide a valid user id");
            }

            let ide: &str = args[2].as_str();
            if let Some(player) = game.resume_game(ide) {
                random = generate_number(player.level);
                level = player.level;
                loop {
                    println!("The Game will start now - Level {}", level);
                    println!("A random number has been printed. Enter your guess below");
                    let mut guess = String::new();
                    io::stdin().read_line(&mut guess).expect("Unable to read guess");
                    println!("guess: {}", guess);
    
                    let guess: i32 = guess.trim().parse().expect("Please input a number");
    
                            match guess.cmp(&random) {
                                Ordering::Less => {
                                    println!("Oops wrong guess. Your guess is lesser than the generated number");
                                    println!("Please try again");
                                    continue;
                                },
                                Ordering::Equal => {
                                    println!("Wow you got it correctly.");
                                    println!("You just levelled up");
                                    level = level + 1;
                                    game.level_up(&ide, level);
                                    game.save_to_file(filename);
                                    random = generate_number(level);
                                    continue;
    
                                },
                                Ordering::Greater => {
                                    println!("Oops wrong guess. Your guess is greater than the generated number");
                                    println!("Please try again");
                                    continue;
    
                                }
    
                            } }
            } else {
                println!("Player not found");
                return
            }

        },
        "list" => {
            
            game.list_players();

        },
        "remove" => {
            if args.len() < 3 {
                println!("please provide a valid user id");
            }
            let ide: &str = args[2].as_str();
            game.remove_player(ide);
        },
        _ => {
            println!("Invalid command entered")

        }

    }  
    
}

fn generate_number(level: i32) -> i32 {
    let mut rng = rand::rng();

    if level < 10 {
        rng.random_range(0..100)
    } else {
        rng.random_range(0..1000)
    }
}
