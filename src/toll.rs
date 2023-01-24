use std::fmt::{Display, Formatter};
use std::time::Duration;
use rand::Rng;
use rand::prelude::*;
use crate::gate::{Gate, WaitingVehicle};
use crate::logger::TollDatabase;
use crate::toll_clock::TollClock;
use crate::vehicle::Vehicle;

/// Péage
pub struct Toll {
    /// Portes du péage
    pub gates: Vec<Gate>,
    /// Horloge interne de la simulation du péage
    pub clock: TollClock,
    /// Objet TollDatabase gérant les enregistrements en base de données
    /// des flux de voitures de ce péage.
    /// Si logger vaut None, aucun enregistrement n'a lieu
    pub logger: Option<TollDatabase>,
}

impl Toll {
    pub fn builder() -> TollBuilder {
        TollBuilder::default()
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        let gates = match vehicle.carpooling() {
            true => self.gates.as_slice(),
            false => self.gates.split_last().unwrap().1
        };
        let less_crowded_gate = gates.iter()
            .filter(|&gate| !gate.empty())
            .min_by(|a, b| a.nb_cars().cmp(&b.nb_cars()));
        let gate = match less_crowded_gate {
            None => gates.get(0).unwrap(),
            Some(gate) => match gate.nb_cars() < 5 {
                true => gate,
                false => gates.iter()
                    .find(|gate| gate.empty())
                    .unwrap_or(gate)
            }
        };
        gate.queue
            .lock()
            .unwrap()
            .push_back(WaitingVehicle {
                vehicle,
                arrival: self.clock.clone(),
            });
        gate.cond.notify_all();
    }

    /// Renvoie le temps qui s'écoulera avant l'arrivée de la porchaine voiture.
    /// Le temps en question est modélisé par une loi exponentielle
    /// dont l'espérance change à chaque heure pour représenter
    /// le différence de fréquentation en fonction du moment de la journée.
    ///
    /// Cette fonction doit être appelée le plus tôt possible après la création
    /// de la dernière voiture.
    ///
    /// ```
    /// let toll = Toll::builder().build(); // Péage par défaut
    /// let mut rng = rand::thread_rng();
    ///
    /// loop {
    ///     let v = Vehicle::new();
    ///     let waiting_time = toll.time_until_next_vehicle(&mut rng);
    ///     toll.add_vehicle(v);
    ///     thread::sleep(waiting_time);
    /// }
    /// ```
    pub fn time_until_next_vehicle<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        static LAMBDAS: [f64; 24] = [
            0.0125, 0.01, 0.00909, 0.0125, 0.0125, 0.014285,
            0.033333, 0.066666, 0.232558, 0.2, 0.05, 0.04,
            0.04, 0.033333, 0.028571, 0.04, 0.1, 0.25,
            0.1, 0.033333, 0.025, 0.02, 0.016666, 0.014285
        ];
        let current_lambda = LAMBDAS[self.clock.clock.hour as usize];
        let generator = rand_distr::Exp::new(current_lambda).unwrap();
        Duration::from_secs((generator.sample(rng)) as u64)
    }
}

impl Display for Toll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = self.clock.clock.to_string();
        for (i, gate) in self.gates.iter().enumerate() {
            buffer.push_str(i.to_string().as_str());
            buffer.push_str(" | ");
            if gate.empty() {
                buffer.push('X');
            }
            for v in gate.queue.lock().unwrap().iter() {
                buffer.push_str(v.vehicle.type_num().to_string().as_str());
            }
            buffer.push('\n');
        }
        f.write_str(buffer.as_str())
    }
}

#[derive(Default)]
pub struct TollBuilder {
    gates: Vec<Gate>,
    clock: TollClock,
    /// nom du fichier de la base de données sqlite
    /// Si cette variable vaut None au moment de l'appel de la méthode
    /// `.build()`, l'objet `Toll` ainsi construit n'effectuera aucun
    /// enregistrement en base de données
    logger_name: Option<String>,
}

impl TollBuilder {
    /// Construit et retourne l'objet Toll correspondant à ce constructeur.
    /// Lance également les threads en arrière-plan liés à ce péage
    /// (threads des portes et thread d'enregistrement en db).
    ///
    /// Si la méthode `.set_logger()` n'a pas été appelée,
    /// le thread d'enregistrement n'est pas lancé.
    pub fn build(mut self) -> Toll {
        let logger = match self.logger_name {
            None => None,
            Some(name) => {
                let db = TollDatabase::new(name.as_str())
                    .unwrap_or_else(|_| panic!("Failed to open database {}", name));
                self.gates.iter_mut()
                    .for_each(|gate| gate.log_sender = Some(db.sender.clone()));
                Some(db)
            }
        };
        self.gates.iter_mut().for_each(|gate| gate.launch_thread());
        Toll {
            gates: self.gates,
            clock: self.clock,
            logger,
        }
    }

    /// Nombre de portes du péage.
    /// Si cette méthode n'est pas appelée, le nombre par défaut est 10
    #[allow(unused)]
    pub fn nb_gates(mut self, nb_gates: usize) -> Self {
        if !self.gates.is_empty() {
            self.gates.clear();
        }
        self.gates.extend((0..nb_gates).map(|_| Gate::new()));
        self
    }

    /// heure de départ de la simulation
    /// Si cette méthode n'est pas appelée, l'heure par défaut est 7h00
    #[allow(unused)]
    pub fn start_hour(mut self, hour: TollClock) -> Self {
        self.clock = hour;
        self
    }

    /// Facteur d'accélération de la simulation.
    #[allow(unused)]
    pub fn acceleration_factor(mut self, factor: u32) -> Self {
        self.clock.acceleration_factor = factor;
        self
    }

    /// Spécifie que les opérations au péage seront enregistrées dans la
    /// base de données dont le nom est spécifié en argument
    /// Il n'est pas obligé de renseigner l'extension de la base de données.
    ///
    /// Si cette méthode n'est pas appelée, aucun enregistrement n'aura lieu
    /// lors de la simulation.
    ///
    /// Si cette méthode est appelée plusieurs fois, une seule base de données
    /// est crée, avec le nom donné lors du dernier appel
    ///
    /// ```
    /// let toll = Toll::builder()
    ///     .set_logger("db") // cette db ne sera jamais ouverte
    ///     .set_logger("other_db")
    ///     .build();
    /// ```
    #[allow(unused)]
    pub fn set_logger(mut self, s: &str) -> Self {
        let mut name = match self.logger_name {
            None => "".to_string(),
            Some(mut name) => {
                name.clear();
                name
            }
        };
        name.push_str(s);
        if !s.ends_with(".sqlite") && s != ":memory:" {
            name.push_str(".sqlite");
        }
        self.logger_name = Some(name);
        self
    }
}