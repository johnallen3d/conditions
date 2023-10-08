# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/johnallen3d/conditions/compare/v0.1.3...v0.2.0) - 2023-10-08

### Added
- *(current)* allow optional region ([#125](https://github.com/johnallen3d/conditions/pull/125))
- *(location)* add caching ([#123](https://github.com/johnallen3d/conditions/pull/123))

### Other
- remove usage of `eprintln!` ([#126](https://github.com/johnallen3d/conditions/pull/126))
- *(deps)* bump ureq from 2.7.1 to 2.8.0
- *(deps)* bump clap from 4.4.5 to 4.4.6 ([#118](https://github.com/johnallen3d/conditions/pull/118))
- *(deps)* bump thiserror from 1.0.48 to 1.0.49 ([#117](https://github.com/johnallen3d/conditions/pull/117))
- *(deps)* bump clap from 4.4.4 to 4.4.5 ([#116](https://github.com/johnallen3d/conditions/pull/116))
- make clippy enforce pedantic lints ([#115](https://github.com/johnallen3d/conditions/pull/115))
- *(deps)* bump clap from 4.4.3 to 4.4.4 ([#114](https://github.com/johnallen3d/conditions/pull/114))
- *(deps)* bump clap from 4.4.2 to 4.4.3 ([#111](https://github.com/johnallen3d/conditions/pull/111))
- always squash the merges!
- skip coverage for dependabot
- add a crates.io badge ([#110](https://github.com/johnallen3d/conditions/pull/110))
- improve unit test coverage ([#107](https://github.com/johnallen3d/conditions/pull/107))
- add code coverage tracking ([#108](https://github.com/johnallen3d/conditions/pull/108))
- prevent release-plz running when dependabot ([#104](https://github.com/johnallen3d/conditions/pull/104))
- *(deps)* bump tibdex/github-app-token from 1 to 2 ([#102](https://github.com/johnallen3d/conditions/pull/102))
- *(deps)* bump actions/checkout from 3 to 4 ([#101](https://github.com/johnallen3d/conditions/pull/101))
- *(deps)* bump rustls-webpki from 0.100.1 to 0.100.2 ([#97](https://github.com/johnallen3d/conditions/pull/97))
- configure dependabot for actions ([#100](https://github.com/johnallen3d/conditions/pull/100))
- run dependabot checks daily
- fix dependabot auto merge
- enable dependabot auto merge ([#96](https://github.com/johnallen3d/conditions/pull/96))
- *(deps)* bump serde from 1.0.179 to 1.0.188 ([#94](https://github.com/johnallen3d/conditions/pull/94))
- towards (more) simplicity ([#93](https://github.com/johnallen3d/conditions/pull/93))
- update CI configuration ([#92](https://github.com/johnallen3d/conditions/pull/92))
- extract an http client trait ([#85](https://github.com/johnallen3d/conditions/pull/85))

## [0.1.3](https://github.com/johnallen3d/conditions/compare/v0.1.2...v0.1.3) - 2023-09-06

### Fixed
- *(weather)* typo and invalid enum ([#87](https://github.com/johnallen3d/conditions/pull/87))

## [0.1.2](https://github.com/johnallen3d/conditions/compare/v0.1.1...v0.1.2) - 2023-09-05

### Added
- *(weather)* add fallback ([#83](https://github.com/johnallen3d/conditions/pull/83))

### Other
- add installation instructions ([#81](https://github.com/johnallen3d/conditions/pull/81))

## [0.1.1](https://github.com/johnallen3d/conditions/compare/v0.1.0...v0.1.1) - 2023-09-04

### Other
- *(bin)* setup binary release ([#77](https://github.com/johnallen3d/conditions/pull/77))
- *(deps)* bump thiserror from 1.0.47 to 1.0.48 ([#70](https://github.com/johnallen3d/conditions/pull/70))

## [0.1.0](https://github.com/johnallen3d/conditions/releases/tag/v0.1.0) - 2023-02-13

### Added
- *(config)* add commands to view config info
- allow user to set temp unit (#19)
- allow user to store location (#17)
- allow user to view stored token (#16)
- [**breaking**] add support for setting token (#12)
- read weatherapi token from config (#9)
- *(bin)* initial commit

### Fixed
- disambiguate the term `conditions`

### Other
- *(deps)* bump clap from 3.2.23 to 4.1.4
- add dependabot configuration
- move from release-please to release-plz
- better help for unit set
- _slightly_ better error handling (#23)
- *(unit)* use enum for unit values
- move current function to struct
- *(config)* move to module
- fix release-please permissions (#10)
- add GitHub Actions configuration (#6)
- add a license
- link to shell script inspiration
- *(usage)* add a note about usage with SketchyBar
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
