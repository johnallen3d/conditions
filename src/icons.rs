use std::collections::HashMap;

#[derive(PartialEq)]
pub enum TimeOfDay {
    Night,
    Day,
}

/// Provides a unicode character (icon) representing the weather condition
/// (via numeric code provided by weatherapi.com and time of day).
///
/// ```
/// let time_of_day = TimeOfDay::Day;
///
/// let icon = icon_for(time_of_day, 1006);
///
/// assert_eq!(icon, " ".to_string());
/// ```
pub fn icon_for(time_of_day: TimeOfDay, code: i32) -> String {
    match time_of_day {
        TimeOfDay::Day => day_icon_for(code),
        TimeOfDay::Night => night_icon_for(code),
    }
}

fn day_icon_for(code: i32) -> String {
    let icons = HashMap::from([
        (1000, " ".to_string()), // Sunny/113
        (1003, " ".to_string()), // Partly cloudy/116
        (1006, " ".to_string()), // Cloudy/119
        (1009, " ".to_string()), // Overcast/122
        (1030, " ".to_string()), // Mist/143
        (1063, " ".to_string()), // Patchy rain possible/176
        (1066, " ".to_string()), // Patchy snow possible/179
        (1069, " ".to_string()), // Patchy sleet possible/182
        (1072, " ".to_string()), // Patchy freezing drizzle possible/185
        (1087, " ".to_string()), // Thundery outbreaks possible/200
        (1114, " ".to_string()), // Blowing snow/227
        (1117, " ".to_string()), // Blizzard/230
        (1135, " ".to_string()), // Fog/248
        (1147, " ".to_string()), // Freezing fog/260
        (1150, " ".to_string()), // Patchy light drizzle/263
        (1153, " ".to_string()), // Light drizzle/266
        (1168, " ".to_string()), // Freezing drizzle/281
        (1171, " ".to_string()), // Heavy freezing drizzle/284
        (1180, " ".to_string()), // Patchy light rain/293
        (1183, " ".to_string()), // Light rain/296
        (1186, " ".to_string()), // Moderate rain at times/299
        (1189, " ".to_string()), // Moderate rain/302
        (1192, " ".to_string()), // Heavy rain at times/305
        (1195, " ".to_string()), // Heavy rain/308
        (1198, " ".to_string()), // Light freezing rain/311
        (1201, " ".to_string()), // Moderate or heavy freezing rain/314
        (1204, " ".to_string()), // Light sleet/317
        (1207, " ".to_string()), // Moderate or heavy sleet/320
        (1210, " ".to_string()), // Patchy light snow/323
        (1213, " ".to_string()), // Light snow/326
        (1216, " ".to_string()), // Patchy moderate snow/329
        (1219, " ".to_string()), // Moderate snow/332
        (1222, " ".to_string()), // Patchy heavy snow/335
        (1225, " ".to_string()), // Heavy snow/338
        (1237, " ".to_string()), // Ice pellets/350
        (1240, " ".to_string()), // Light rain shower/353
        (1243, " ".to_string()), // Moderate or heavy rain shower/356
        (1246, " ".to_string()), // Torrential rain shower/359
        (1249, " ".to_string()), // Light sleet showers/362
        (1252, " ".to_string()), // Moderate or heavy sleet showers/365
        (1255, " ".to_string()), // Light snow showers/368
        (1258, " ".to_string()), // Moderate or heavy snow showers/371
        (1261, " ".to_string()), // Light showers of ice pellets/374
        (1264, " ".to_string()), // Moderate or heavy showers of ice pellets/377
        (1273, " ".to_string()), // Patchy light rain with thunder/386
        (1276, " ".to_string()), // Moderate or heavy rain with thunder/389
        (1279, " ".to_string()), // Patchy light snow with thunder/392
        (1282, " ".to_string()), // Moderate or heavy snow with thunder/395
    ]);

    match icons.get(&code) {
        Some(val) => val.to_string(),
        None => "?".to_string(),
    }
}

fn night_icon_for(code: i32) -> String {
    let icons = HashMap::from([
        (1000, ""), // Clear/113
        (1003, ""), // Partly cloudy/116
        (1006, ""), // Cloudy/119
        (1009, ""), // Overcast/122
        (1030, ""), // Mist/143
        (1063, ""), // Patchy rain possible/176
        (1066, ""), // Patchy snow possible/179
        (1069, ""), // Patchy sleet possible/182
        (1072, ""), // Patchy freezing drizzle possible/185
        (1087, ""), // Thundery outbreaks possible/200
        (1114, ""), // Blowing snow/227
        (1117, ""), // Blizzard/230
        (1135, ""), // Fog/248
        (1147, ""), // Freezing fog/260
        (1150, ""), // Patchy light drizzle/263
        (1153, ""), // Light drizzle/266
        (1168, ""), // Freezing drizzle/281
        (1171, ""), // Heavy freezing drizzle/284
        (1180, ""), // Patchy light rain/293
        (1183, ""), // Light rain/296
        (1186, ""), // Moderate rain at times/299
        (1189, ""), // Moderate rain/302
        (1192, ""), // Heavy rain at times/305
        (1195, ""), // Heavy rain/308
        (1198, ""), // Light freezing rain/311
        (1201, ""), // Moderate or heavy freezing rain/314
        (1204, ""), // Light sleet/317
        (1207, ""), // Moderate or heavy sleet/320
        (1210, ""), // Patchy light snow/323
        (1213, ""), // Light snow/326
        (1216, ""), // Patchy moderate snow/329
        (1219, ""), // Moderate snow/332
        (1222, ""), // Patchy heavy snow/335
        (1225, ""), // Heavy snow/338
        (1237, ""), // Ice pellets/350
        (1240, ""), // Light rain shower/353
        (1243, ""), // Moderate or heavy rain shower/356
        (1246, ""), // Torrential rain shower/359
        (1249, ""), // Light sleet showers/362
        (1252, ""), // Moderate or heavy sleet showers/365
        (1255, ""), // Light snow showers/368
        (1258, ""), // Moderate or heavy snow showers/371
        (1261, ""), // Light showers of ice pellets/374
        (1264, ""), // Moderate or heavy showers of ice pellets/377
        (1273, ""), // Patchy light rain with thunder/386
        (1276, ""), // Moderate or heavy rain with thunder/389
        (1279, ""), // Patchy light snow with thunder/392
        (1282, ""), // Moderate or heavy snow with thunder/395
    ]);

    match icons.get(&code) {
        Some(val) => val.to_string(),
        None => "?".to_string(),
    }
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
