# holo-hylo

DNAs required for running Hylo on Holochain.

! This is a work in progress and only limited functionality is available at this stage !

## Building and running Holochain instance

Instructions for installing Rust and building the holochain container can be found at [developer.holochain](https://developer.holochain.org/start.html).

After this is installed the DNA can be build using
`hc package`
and started using
`holochain -c ./conductor-configs/conductor-config.toml`

If you plan to develop the DNA code further the automated tests can be run using
`nix-shell --run 'npm run test'`
