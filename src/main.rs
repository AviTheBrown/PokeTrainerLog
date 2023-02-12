use std::io;
#[derive(Debug)]


struct PokeTrainer {
    active: bool,
    name: String,
    age: i8,
    town: String
}
impl PokeTrainer {
    fn newTrainer() -> PokeTrainer {
        let mut pokeTraner = PokeTrainer {
            active: true,
            name: "".to_string(),
            age: 0,
            town: "".to_string(),
        };
        pokeTraner.fillFields();
        pokeTraner
    }

    fn fillFields(&mut self) {
        println!("Please enter your name poke' trainer.");
        io::stdin().read_line(&mut self.name).unwrap();

        println!("Please enter your age.");
        let mut age = String::new();
        io::stdin().read_line(&mut age).unwrap();
        self.age = age.trim().parse().unwrap();

        println!("Please enter where you are from.");
        io::stdin().read_line(&mut self.town).unwrap();
    }
}



enum poke_ball {
    PokeBall,
    MasterBall,
    UltraBall
}
fn main() {
    let trainer1 = PokeTrainer::newTrainer();
    println!("{:?}", trainer1)

}
