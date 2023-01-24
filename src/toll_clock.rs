//! Sert à contenir les structures de données propres à représenter
//! le temps qui s'écoule au sein de la simulation


use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

/// Représente l'heure et le jour qu'il est dans la simulation
#[derive(Debug, Clone)]
pub struct SimpleTime {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32
}

impl Default for SimpleTime {
    /// Par défaut, SimpleTime vaut 07h 00m 00s au jour 0
    /// ```
    /// let t = SimpleTime {
    ///     day: 0,
    ///     hour: 7,
    ///     minute: 0,
    ///     second: 0,
    /// };
    /// assert_eq!(t, SimpleTime::default())
    /// ```
    fn default() -> Self {
        Self {
            day: 0,
            hour: 7,
            minute: 0,
            second: 0,
        }
    }
}

impl Add<Duration> for SimpleTime {
    type Output = SimpleTime;

    fn add(self, rhs: Duration) -> Self::Output {
        let mut output = self;
        output += rhs;
        output
    }
}

impl AddAssign<Duration> for SimpleTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.second += rhs.as_secs() as u32;
        if self.second > 59 {
            self.minute += self.second / 60;
            self.second %= 60;
        }
        if self.minute > 59 {
            self.hour += self.minute / 60;
            self.minute %= 60;
        }
        if self.hour > 23 {
            self.day += self.hour / 24;
            self.hour %= 24;
        }
    }
}

impl Display for SimpleTime {
    /// Affiche au format :
    /// - si day == 0 : "{hour}h{minute}m{second}"
    /// - si day == 1 : "{day} jour {hour}h{minute}m{second}"
    /// - si day > 1 : "{day} jours {hour}h{minute}m{second}"
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        if self.day > 0 {
            buffer.push_str((self.day.to_string() + " jour").as_str());
            if self.day > 1 {
                buffer.push('s');
            }
            buffer.push(' ');
        }
        buffer.push_str(
            format!(
                "{:02}h{:02}m{:02}s\n", self.hour, self.minute, self.second
            ).as_str()
        );
        f.write_str(buffer.as_str())
    }
}

impl SimpleTime {
    /// Renvoie un String au format {day}T{hour}:{minute}:{second}
    pub fn to_timestamp(&self) -> String {
        format!(
            "{}T{:02}:{:02}:{:02}",
            self.day, self.hour, self.minute, self.second
        )
    }
}

/// Sert à conserver le temps qui s'écoule depuis la création
/// ou la dernière mise à jour de l'objet
#[derive(Debug, Clone)]
pub struct TollClock {
    /// A quel point la simulation est plus rapide que le temps réel
    pub acceleration_factor: u32,
    /// L'objet SimpleTime qui indique l'heure dans la simulation
    pub clock: SimpleTime,
    /// Temps réel écoulé depuis la création où la dernière mise à jour
    last_tick: Instant,
}

impl TollClock {
    /// Met à jour le temps indiqué par l'horloge de cet objet.
    /// Le nouveau temps indiqué sera l'ancien temps plus le temps écoulé
    /// multiplié par le facteur d'accélération
    pub fn update(&mut self) {
        let elapsed = self.last_tick.elapsed();
        self.clock += elapsed * self.acceleration_factor;
        self.last_tick = Instant::now();
    }

    /// Renvoie un objet SimpleTime représentant l'heure qu'il est
    /// dans la simulation par rapport à cet objet
    /// ```
    /// let clock = TollClock {
    ///     acceleration_factor: 2,
    ///     clock: SimpleTime::default(),
    ///     last_tick: Instant::now()
    /// }
    /// thread::sleep(Duration::from_secs(2));
    /// let now = clock.now();
    /// assert_eq!(now, SimpleTime::default() + Duration::from_secs(4));
    /// ```
    pub fn now(&self) -> SimpleTime {
        let elapsed = self.last_tick.elapsed();
        self.clock.clone() + elapsed * self.acceleration_factor
    }
}

impl Default for TollClock {
    fn default() -> Self {
        Self {
            acceleration_factor: 10,
            clock: SimpleTime::default(),
            last_tick: Instant::now(),
        }
    }
}


