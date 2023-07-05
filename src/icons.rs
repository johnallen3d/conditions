use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum TimeOfDay {
    Night,
    Day,
}

impl From<bool> for TimeOfDay {
    fn from(is_day: bool) -> Self {
        if is_day {
            TimeOfDay::Day
        } else {
            TimeOfDay::Night
        }
    }
}

impl TimeOfDay {
    /// Provides a unicode character (icon) representing the weather condition
    /// (via numeric code provided by weatherapi.com and time of day).
    ///
    /// ```
    /// use conditions::icons::*;
    ///
    /// let time_of_day = TimeOfDay::Day;
    ///
    /// let icon = icon_for(time_of_day, 1006);
    ///
    /// assert_eq!(icon, " ".to_string());
    /// ```
    pub fn icon(&self, code: i32) -> String {
        match self {
            TimeOfDay::Day => DAY_ICONS.get(&code).unwrap_or(&"?").to_string(),
            TimeOfDay::Night => {
                NIGHT_ICONS.get(&code).unwrap_or(&"?").to_string()
            }
        }
    }
}

lazy_static! {
    static ref DAY_ICONS: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1000, " "); // Clear/113
        m.insert(1003, " "); // Partly cloudy/116
        m.insert(1006, " "); // Cloudy/119
        m.insert(1009, " "); // Overcast/122
        m.insert(1030, " "); // Mist/143
        m.insert(1063, " "); // Patchy rain possible/176
        m.insert(1066, " "); // Patchy snow possible/179
        m.insert(1069, " "); // Patchy sleet possible/182
        m.insert(1072, " "); // Patchy freezing drizzle possible/185
        m.insert(1087, " "); // Thundery outbreaks possible/200
        m.insert(1114, " "); // Blowing snow/227
        m.insert(1117, " "); // Blizzard/230
        m.insert(1135, " "); // Fog/248
        m.insert(1147, " "); // Freezing fog/260
        m.insert(1150, " "); // Patchy light drizzle/263
        m.insert(1153, " "); // Light drizzle/266
        m.insert(1168, " "); // Freezing drizzle/281
        m.insert(1171, " "); // Heavy freezing drizzle/284
        m.insert(1180, " "); // Patchy light rain/293
        m.insert(1183, " "); // Light rain/296
        m.insert(1186, " "); // Moderate rain at times/299
        m.insert(1189, " "); // Moderate rain/302
        m.insert(1192, " "); // Heavy rain at times/305
        m.insert(1195, " "); // Heavy rain/308
        m.insert(1198, " "); // Light freezing rain/311
        m.insert(1201, " "); // Moderate or heavy freezing rain/314
        m.insert(1204, " "); // Light sleet/317
        m.insert(1207, " "); // Moderate or heavy sleet/320
        m.insert(1210, " "); // Patchy light snow/323
        m.insert(1213, " "); // Light snow/326
        m.insert(1216, " "); // Patchy moderate snow/329
        m.insert(1219, " "); // Moderate snow/332
        m.insert(1222, " "); // Patchy heavy snow/335
        m.insert(1225, " "); // Heavy snow/338
        m.insert(1237, " "); // Ice pellets/350
        m.insert(1240, " "); // Light rain shower/353
        m.insert(1243, " "); // Moderate or heavy rain shower/356
        m.insert(1246, " "); // Torrential rain shower/359
        m.insert(1249, " "); // Light sleet showers/362
        m.insert(1252, " "); // Moderate or heavy sleet showers/365
        m.insert(1255, " "); // Light snow showers/368
        m.insert(1258, " "); // Moderate or heavy snow showers/371
        m.insert(1261, " "); // Light showers of ice pellets/374
        m.insert(1264, " "); // Moderate or heavy showers of ice pellets/377
        m.insert(1273, " "); // Patchy light rain with thunder/386
        m.insert(1276, " "); // Moderate or heavy rain with thunder/389
        m.insert(1279, " "); // Patchy light snow with thunder/392
        m.insert(1282, " "); // Moderate or heavy snow with thunder/395
        m
    };

    static ref NIGHT_ICONS: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1000, ""); // Clear/113
        m.insert(1003, ""); // Partly cloudy/116
        m.insert(1006, ""); // Cloudy/119
        m.insert(1009, ""); // Overcast/122
        m.insert(1030, ""); // Mist/143
        m.insert(1063, ""); // Patchy rain possible/176
        m.insert(1066, ""); // Patchy snow possible/179
        m.insert(1069, ""); // Patchy sleet possible/182
        m.insert(1072, ""); // Patchy freezing drizzle possible/185
        m.insert(1087, ""); // Thundery outbreaks possible/200
        m.insert(1114, ""); // Blowing snow/227
        m.insert(1117, ""); // Blizzard/230
        m.insert(1135, ""); // Fog/248
        m.insert(1147, ""); // Freezing fog/260
        m.insert(1150, ""); // Patchy light drizzle/263
        m.insert(1153, ""); // Light drizzle/266
        m.insert(1168, ""); // Freezing drizzle/281
        m.insert(1171, ""); // Heavy freezing drizzle/284
        m.insert(1180, ""); // Patchy light rain/293
        m.insert(1183, ""); // Light rain/296
        m.insert(1186, ""); // Moderate rain at times/299
        m.insert(1189, ""); // Moderate rain/302
        m.insert(1192, ""); // Heavy rain at times/305
        m.insert(1195, ""); // Heavy rain/308
        m.insert(1198, ""); // Light freezing rain/311
        m.insert(1201, ""); // Moderate or heavy freezing rain/314
        m.insert(1204, ""); // Light sleet/317
        m.insert(1207, ""); // Moderate or heavy sleet/320
        m.insert(1210, ""); // Patchy light snow/323
        m.insert(1213, ""); // Light snow/326
        m.insert(1216, ""); // Patchy moderate snow/329
        m.insert(1219, ""); // Moderate snow/332
        m.insert(1222, ""); // Patchy heavy snow/335
        m.insert(1225, ""); // Heavy snow/338
        m.insert(1237, ""); // Ice pellets/350
        m.insert(1240, ""); // Light rain shower/353
        m.insert(1243, ""); // Moderate or heavy rain shower/356
        m.insert(1246, ""); // Torrential rain shower/359
        m.insert(1249, ""); // Light sleet showers/362
        m.insert(1252, ""); // Moderate or heavy sleet showers/365
        m.insert(1255, ""); // Light snow showers/368
        m.insert(1258, ""); // Moderate or heavy snow showers/371
        m.insert(1261, ""); // Light showers of ice pellets/374
        m.insert(1264, ""); // Moderate or heavy showers of ice pellets/377
        m.insert(1273, ""); // Patchy light rain with thunder/386
        m.insert(1276, ""); // Moderate or heavy rain with thunder/389
        m.insert(1279, ""); // Patchy light snow with thunder/392
        m.insert(1282, ""); // Moderate or heavy snow with thunder/395
        m
    };
}

#[test]
fn valid_code_for_day() {
    let icon = icon_for(TimeOfDay::Day, 1006);

    assert_eq!(icon, " ".to_string());
}

#[test]
fn valid_code_for_night() {
    let icon = icon_for(TimeOfDay::Night, 1195);

    assert_eq!(icon, "".to_string());
}

#[test]
fn invalid_code_for() {
    let icon = icon_for(TimeOfDay::Night, 9999);

    assert_eq!(icon, "?".to_string());
}
