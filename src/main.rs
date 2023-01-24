use std::thread::sleep;
use rand::Rng;

use crate::toll::Toll;
use crate::vehicle::Vehicle;

mod toll;
mod vehicle;
mod gate;
mod toll_clock;
mod logger;
mod vt100;

/// Fonction principale du programme
/// Crée le péage, puis effectue une boucle infinie pour rajouter
/// des véhicules dans le péage à intervalles de temps aléatoires.
fn main() {
    vt100::init();
    println!("{}", "\n".repeat(7));
    let mut rng = rand::thread_rng();
    let mut toll = Toll::builder()
        .nb_gates(6)
        .acceleration_factor(60) // 1 seconde = 1 minute
        .set_logger("toll.sqlite")
        .build();
    loop {
        let vehicle = rng.gen::<Vehicle>();
        let time_until_next = toll.time_until_next_vehicle(&mut rng);
        toll.add_vehicle(vehicle);
        println!("{}", &toll);
        sleep(time_until_next / toll.clock.acceleration_factor);
        toll.clock.update();
    }
}
