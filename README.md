# Communities on Holochain

DNAs required for running Communities on Holochain.

! This is a work in progress and only limited functionality is available at this stage !

## Building and running Holochain instance

Instructions for installing Rust and building the holochain container can be found at [developer.holochain](https://developer.holochain.org/start.html).

After this is installed the DNA can be build using
`hc package`
and started using
`hc run`

## Running multiple instances for UI testing

### For 2 instances (run the following in three seperate sessions of nix-shells)

```
sim2h_server --port 9001

hc run --agent-name agent0 --port 3400 --networked sim2h --sim2h-server ws://localhost:9001

hc run --agent-name agent1 --port 3401 --networked sim2h --sim2h-server ws://localhost:9001
```

If you plan to develop the DNA code further the automated tests can be run using
`nix-shell --run 'npm run test'`


