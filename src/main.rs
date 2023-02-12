use std::io;

enum poke_ball {
    PokeBall,
    MasterBall,
    UltraBall
}
fn chose_ball_type() -> poke_ball {

}

fn main() {
    println!("Welcome to Poke' Tracker");
    println!("What is your name?");

    let mut poke_trainer_name = String::new();
    io::stdin().read_line(&mut poke_trainer_name).unwrap();

    println!("Hello {} welcome to the Poke Trainer Tracker.", poke_trainer_name);


}
