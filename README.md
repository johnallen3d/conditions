# Conditions

A command line tool for getting the weather conditions at the current location.

## Weather Sources

- [open-meteo.com](https://open-meteo.com/)
- [weatherapi.com](https://www.weatherapi.com/) \*

\* requires an [api key](https://www.weatherapi.com/docs/#intro-authentication)

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

Here's how I'm using this with SketchyBar.

```bash
#!/bin/bash

conditions="$(conditions current)"
icon=$(echo "$conditions" | jq -r .icon)
temp=$(echo "$conditions" | jq -r .temp)

sketchybar -m \
  --set weather_logo icon="${icon}" \
  --set weather label="${temp}°F"
```

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
