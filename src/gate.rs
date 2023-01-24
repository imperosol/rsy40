use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use crate::toll_clock::{SimpleTime, TollClock};
use crate::vehicle::Vehicle;

/// Véhicule en train d'attendre son tour pour payer le péage
#[derive(Debug)]
pub struct WaitingVehicle {
    pub vehicle: Vehicle,
    pub arrival: TollClock
}

/// Véhicule qui a fini de payer et a quitté le péage
#[derive(Debug)]
pub struct DepartedVehicle {
    pub vehicle: Vehicle,
    /// Heure d'arrivée du véhicule au péage
    pub arrival: SimpleTime,
    /// Heure de départ du véhicule depuis le péage
    pub departure: SimpleTime
}

/// Porte du péage.
/// A chaque porte est associé un thread qui fait payer le
/// véhicule en tête de la file d'attente, dort pendant le temps
/// représentant le temps passé à payer, passe à la voiture suivante.
/// Si la file est vide, le thread s'endort jusqu'à l'arrivée d'une nouvelle
/// voiture
#[derive(Debug)]
pub struct Gate {
    /// File de véhicules en attente pour payer le péage
    pub queue: Arc<Mutex<VecDeque<WaitingVehicle>>>,
    /// Condition servant à réveiller le thread de la porte du péage
    /// lorsqu'une voiture sur une voie auparavant vide
    pub cond: Arc<Condvar>,
    /// Sender servant à envoyer au thread d'enregistrement en db
    /// du péage une voiture qui vient de compléter son paiement.
    pub log_sender: Option<Sender<DepartedVehicle>>
}

impl Gate {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
            cond: Arc::new(Condvar::new()),
            log_sender: None
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    /// lance le thread de la porte
    /// Celui-ci continuera indéfiniment jusqu'à l'arrêt du programme.
    pub fn launch_thread(&self) {
        let queue = self.queue.clone();
        let cond = self.cond.clone();
        let log_sender = self.log_sender.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let mut lock = queue.lock().unwrap();
                while lock.is_empty() {
                    lock = cond.wait(lock).unwrap();
                }
                let next_vehicle = lock.pop_front().unwrap();
                drop(lock);
                let vehicle = next_vehicle.vehicle;
                let clock = next_vehicle.arrival;
                thread::sleep(
                    vehicle.payment_duration(&mut rng) / clock.acceleration_factor
                );
                if let Some(ref sender) = log_sender {
                    sender.send(DepartedVehicle {
                        vehicle,
                        arrival: clock.clock.clone(),
                        departure: clock.now(),
                    }).unwrap();
                }
            }
        });
    }

    pub fn nb_cars(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}
