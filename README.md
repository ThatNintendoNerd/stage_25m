# stage_25m

A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate designed to patch both code and data used in the 75 m stage to better represent 25 m. This plugin is meant to be used alongside this stage [mod](https://gamebanana.com/mods/309758).

The latest release can be found [here](https://github.com/ThatNintendoNerd/stage_25m/releases/).

## Build from Source

If you would like to build the plugin starting from the source code, you can build the NRO file using the standard command for compiling Skyline plugins:

```
cargo skyline build --release
```

The resulting build is found under `/target/aarch64-skyline-switch/release/libstage_25m.nro`

### Prerequisites

- Stable [Rust](https://www.rust-lang.org/) environment
  - [cargo-skyline](https://github.com/jam1garner/cargo-skyline)

## Credits

* __jam1garner__ for guiding me through the initial development of this plugin, the byte-searching code, and other minor help.
* __Raytwo__ for the offset-searching macro used in the plugin.
* __Peter___ for providing me with the base of the register-modifying code and other minor help.
