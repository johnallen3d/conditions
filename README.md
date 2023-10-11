# Conditions
![Build Status](https://github.com/johnallen3d/conditions/actions/workflows/ci.yml/badge.svg) [![Coverage Status](https://coveralls.io/repos/github/johnallen3d/conditions/badge.svg?branch=feature/track-coverage)](https://coveralls.io/github/johnallen3d/conditions?branch=feature/track-coverage) ![Crate Status](https://img.shields.io/crates/v/conditions.svg)

A command line tool for getting the weather conditions at the current location.

## Weather Sources

- [open-meteo.com](https://open-meteo.com/)
- [weatherapi.com](https://www.weatherapi.com/) \*

\* requires an [api key](https://www.weatherapi.com/docs/#intro-authentication)

## Installation

At this time it is necessary to compile and install the crate locally. The simplest way to do this is to [install the Rust toolchain](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then use `cargo` to build and install from [crates.io](https://crates.io/crates/conditions)

```bash
cargo install conditions
```

## Usage

By default the Open-Meteo weather provider will be used along with a location determined by the machine's ip address.

```sh
> conditions current | jq
{
  "temp": 57,
  "icon": ""
}
```

### Weather API

In order to use the Weather API provider create an account and then an API Key. Then persist the api key in settings.

```bash
conditions weather-api-key set [your-api-key]
```

### Location

If you prefer to set a specific location you can do so via a postal code:

```bash
conditions location set "[postal-code], [country]"
```

For example:

```bash
conditions location set "10001, usa"
```

### SketchyBar

[SketchyBar](https://github.com/FelixKratz/SketchyBar) is a very customizable bar app for macOS that is deliberately light on resource usage (written in C vs more common WebView approach). Widgets in SketchyBar are refreshed (if need be) via "plugins" (typically shell scripts).

#### Manual

Here's how I'm using `conditions` with SketchyBar in the more traditional plugin approach.

```bash
#!/bin/bash

conditions="$(conditions current)"
icon=$(echo "$conditions" | jq -r .icon)
temp=$(echo "$conditions" | jq -r .temp)

sketchybar -m \
  --set weather_logo icon="${icon}" \
  --set weather label="${temp}°F"
```

#### Direct Integration

Alternatively `conditions` can directly update the running instance of SketchyBar without making calls from a shell script using `sketchybar -m`. In order to use this experimental version, the `sketchybar` feature must be enabled on install.

```bash
cargo install conditions --features sketchybar
```

Then when setting up a weather widget in `sketchybarrc`, `conditions` can be invoked directly.

```bash
sketchybar \
  --add item weather right \
  update_freq=1800 \
  icon.drawing=off \
  script="$HOME/.cargo/bin/conditions current --icon="

sketchybar \
  --add item weather_logo right \
  icon.font="Hack Nerd Font:Regular:14.0" \
  label.drawing=off \
```

Note, at this time `conditions` is hard-coded with widgets named `weather` and `weather_logo`:

```rust
let message = format!(
    "--set weather_logo icon=\"{}\" --set weather label=\"{}°{}\"",
    conditions.icon,
    conditions.temp_f as i32,
    self.config.unit.as_char().to_ascii_uppercase()
);
```

In my bar this results in:

![example SketchyBar](./assets/sketchybar-example.png)

## Tasks

Run tasks from this directory via: `xc [task-name]`

### check

```sh
cargo build
```

### build

```sh
cargo build
```

### run

```sh
cargo run
```

### install

```sh
cargo install --path .
```

## Dependencies

| Project             | Version |
| ------------------- | ------- |
| rust-lang.org       | ^1.6    |
| rust-lang.org/cargo | ^0.66   |
| git-scm.org         | ^2.38   |
| xcfile.dev          | ^0      |

## Why

I wanted to learn rust and had a [(cargo-culted) shell script](https://github.com/johnallen3d/dotfiles/blob/16054c903bc8cc0ca939c279382ec6b15eb1bc7c/dot_config/sketchybar/plugins/executable_weather.sh#L1) that performed this task for usage with [sketchybar](https://github.com/FelixKratz/SketchyBar).
