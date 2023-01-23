use std::fmt::{Display, Formatter, write};
use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

/// Sert à contenir les structures de données propres à représenter
/// le temps qui s'écoule au sein de la simulation

#[derive(Debug, Clone)]
pub struct SimpleTime {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32
}

impl Default for SimpleTime {
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

#[derive(Debug, Clone)]
pub struct TollClock {
    pub acceleration_factor: u32,
    pub clock: SimpleTime,
    last_tick: Instant,
}

impl TollClock {
    pub fn update(&mut self) {
        let elapsed = self.last_tick.elapsed();
        self.clock += elapsed * self.acceleration_factor;
        self.last_tick = Instant::now();
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


