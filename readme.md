![Github2](https://github.com/Qubic-Solutions/rqiner-builds/assets/57864185/d9f30bed-530b-4040-9edf-1d43b44119fc)

# rqiner

**rqiner** is a highly optimized and 100% portable miner for Qubic made in Rust and is natively integrated with the Qubic-Solutions mining pool.
rqiner can run anywhere LLVM allows it to. Since cross compilation takes some time and the miner is constantly being optimized, some targets are not updated regularly. If you need a specific binary (e.g. x86-64(x86), aarch64(ARMv9, ARMv8, ARMv7), RISC-V) open an issue or DM `mineco` on Discord.

## Usage

To use rqiner you can refer to the --help output

```
High performance Qubic CPU miner powered by Rust

Usage: rqiner [OPTIONS] --threads <THREADS>

Options:
  -t, --threads <THREADS>  Amount of threads used for mining
  -b, --bench              Benchmarks your miner without submitting solutions
  -i, --id <ID>            Your payout Qubic ID (required for pool mining)
  -l, --label <LABEL>      Label used for identification of your miner on the pool
  -h, --help               Print help
  -V, --version            Print version
```

To benchmark the miner

`./rqiner -t <threads> --bench`

To start pool mining

`./rqiner -t <threads> -i <payout-id>`

The label is optional and can simply be added to the pool mining start command

`./rqiner -t <threads> -i <payout-id> --label <miner-label>`

Note that the label may not be more than 64 characters long.

## Discord

For further help with setting up the miner you can join our official Discord server

[![](https://img.shields.io/discord/1179806757204267090?color=5865F2&logo=Discord&style=flat-square)](https://discord.gg/zTrdShyQu2)
