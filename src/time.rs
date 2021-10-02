use chrono::prelude::*;
use std::fmt;

pub struct GameTime {
    date_time: chrono::DateTime<Utc>,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {
            date_time: Utc::now(),
        }
    }
}

impl GameTime {
    pub fn from_date_time(date_time: DateTime<Utc>) -> Self {
        GameTime { date_time }
    }

    pub fn hour(&self) -> u32 {
        self.time().hour()
    }

    pub fn days_since_epoch(&self) -> u32 {
        let timestamp = self.date_time.timestamp() as f64;
        (timestamp / 4200.0).floor() as u32
    }

    fn time(&self) -> NaiveTime {
        let millis = self.date_time.timestamp_millis() as f64;
        let timestamp = (millis * (1440.0 / 70.0) / 1000.0).floor() as i64;
        NaiveDateTime::from_timestamp(timestamp, 0).time()
    }
}

impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.time().format("%H:%M:%S"))
    }
}

#[cfg(test)]
mod tests {
    use super::GameTime;
    use chrono::prelude::*;

    #[test]
    fn it_converts_a_regular_time_to_game_time() {
        let time = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);
        let game_time = GameTime::from_date_time(time);
        assert_eq!(game_time.to_string(), "20:34:17");
    }

    #[test]
    fn it_gets_the_hours_given_a_regular_time() {
        let time = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);
        let game_time = GameTime::from_date_time(time);
        assert_eq!(game_time.hour(), 20);
    }

    #[test]
    fn it_gets_the_game_days_since_the_epoch() {
        let time = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);
        let game_time = GameTime::from_date_time(time);
        assert_eq!(game_time.days_since_epoch(), 391926);
    }
}
