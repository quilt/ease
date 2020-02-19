# Ease
This repo contains core libraries needed for your EE to interact with a simulated eth2 node.

## Quick Start

Instructions to get started developing EEs for Ethereum 2.0:

1. Install Rust:

MacOS:
```bash
$ brew install rustup
$ rustup-init
```

2. Update and add WebAssembly target
```bash
$ rustup update
$ rustup target add wasm32-unknown-unknown
```

3. Install ease subcommand:
```bash
$ cargo install --force cargo-ease
```

4. Create a new EE project:
```bash
$ cargo ease
❔  Project Name: foo
⛏   Creating project  `foo`...
🦄   Boom! New project created /Users/satoshi/Documents/projects/foo   
```

5. Build 
```bash
$ cargo build
```

6. Test
```bash
$ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running target/debug/deps/bar-328e4eb7ecac9fcb

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/ee_tests-fad549fce44d7362

running 1 test
pre root: [129, 27, 131, 201, 26, 96, 234, 50, 183, 111, 122, 129, 48, 159, 130, 184, 169, 215, 152, 134, 111, 75, 4, 72, 68, 230, 36, 238, 155, 143, 103, 11]
post root: [218, 116, 40, 127, 249, 115, 196, 89, 157, 35, 204, 41, 3, 48, 181, 204, 110, 193, 221, 157, 90, 96, 134, 190, 85, 129, 41, 221, 60, 230, 32, 68]
pre root  => "811b83c91a60ea32b76f7a81309f82b8a9d798866f4b044844e624ee9b8f670b"
post root => "da74287ff973c4599d23cc290330b5cc6ec1dd9d5a6086be558129dd3ce62044"
expected  => "da74287ff973c4599d23cc290330b5cc6ec1dd9d5a6086be558129dd3ce62044"
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

7. Run
```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/bar`
pre root  => "811b83c91a60ea32b76f7a81309f82b8a9d798866f4b044844e624ee9b8f670b"
post root => "da74287ff973c4599d23cc290330b5cc6ec1dd9d5a6086be558129dd3ce62044"
expected  => "da74287ff973c4599d23cc290330b5cc6ec1dd9d5a6086be558129dd3ce62044"
```

You now have a simple transfer EE built and running.  Modify the source to create your own custom EE!!

> Note: The EE uses a Binary Merkle Tree as the proof backend [oof](https://github.com/quilt/oof), but you can swap it out with your own implementation.

## Relatated Repos
- [cargo-ease](https://github.com/quilt/cargo-ease): cargo subcommand to generate an EE project template
- [ease-template](https://github.com/quilt/ease-template): template used by `cargo ease` to create an EE project template
- [oof](https://github.com/quilt/oof): Simple proof backend
- [proof](https://github.com/lightclient/proof): Interface to proof backends
- [eWasm-rt](https://github.com/quilt/ewasm-rt): WebAssembly runtime for Ethereum 2.0
- [simulation](https://github.com/quilt/simulation): Ethereum 2.0 execution simulation


## Additional Resources
- [project roadmap](docs/roadmap.md)



