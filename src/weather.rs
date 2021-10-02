use crate::time::GameTime;

#[derive(Debug, PartialEq)]
pub enum WeatherType {
    Blizzards,
    ClearSkies,
    Clouds,
    DustStorms,
    FairSkies,
    Fog,
    Gales,
    Gloom,
    HeatWaves,
    Rain,
    Showers,
    Snow,
    Thunder,
    Thunderstorms,
    UmbralStatic,
    UmbralWind,
    Wind,
}

pub struct Weather {}

impl Weather {
    pub fn from_game_time(time: GameTime) -> u32 {
        let hour = time.hour();
        let inc = (hour + 8 - (hour % 8)) % 24;
        let days = time.days_since_epoch();
        let base = days * 0x64 + inc;
        let s1 = (base << 0xb) ^ base;
        let s2 = (s1 >> 8) ^ s1;
        s2 % 0x64
    }
}

#[cfg(test)]
mod tests {
    use super::Weather;
    use crate::time::GameTime;
    use chrono::prelude::*;

    #[test]
    fn it_converts_a_time_to_a_number() {
        let time = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);
        let game_time = GameTime::from_date_time(time);
        let n = Weather::from_game_time(game_time);
        assert_eq!(n, 92);
    }
}
