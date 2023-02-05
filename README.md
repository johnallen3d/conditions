# Conditions

A command line tool for getting the weather conditions at the current location.

## Usage

```sh
> conditions | jq
{
  "code": 1003,
  "temp": 57,
  "is_day": false,
  "icon": ""
}
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

I wanted to learn rust and had a (cargo-culted) shell script that performed this task for usage with [sketchybar](https://github.com/FelixKratz/SketchyBar).
