use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

use crate::toll::Toll;
use crate::toll_clock::TollClock;
use crate::vehicle::Vehicle;

mod toll;
mod vehicle;
mod gate;
mod toll_clock;

fn main() {
    let mut rng = rand::thread_rng();
    let mut toll = Toll::builder()
        .nb_gates(10)
        .acceleration_factor(100)
        .build();
    let clock = TollClock::default();
    println!("{:?}", clock);
    for gate in toll.gates.iter_mut() {
        gate.launch_thread();
    }
    loop {
        let vehicle = rng.gen::<Vehicle>();
        let time_until_next = toll.time_until_next_vehicle(&mut rng);
        toll.add_vehicle(vehicle);
        println!("{}", &toll);
        sleep(time_until_next / toll.clock.acceleration_factor);
        toll.clock.update();
    }
}
