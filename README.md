# stage_25m
A Skyline plugin designed to patch both code and data used for the 75 m stage in Super Smash Bros. Ultimate to better represent 25 m. This plugin is meant to be used alongside this stage [mod](https://gamebanana.com/mods/309758).

Built releases can be found [here](https://github.com/ThatNintendoNerd/stage_25m/releases/).

## Credits
* __jam1garner__ for guiding me through the initial development of this plugin, the byte-searching code, and other minor help.
* __Peter___ for providing me with the base of the register modifying code and other minor help.
* __Raytwo__ for the offset searching macro used in the plugin.

## Build from Source
### Prerequisites
- Rust environment with [cargo-skyline](https://github.com/jam1garner/cargo-skyline)

### Build Steps
Download the master ZIP file or clone the repository with [Git](https://git-scm.com/downloads).
```bash
# to build the 25 m Skyline plugin
# the resulting build is found in /target/aarch64-skyline-switch/release/libstage_25m.nro
cargo skyline build --release
```