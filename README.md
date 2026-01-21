# NEXi-Hyperon PoC

## Setup
git submodule update --init --recursive
cargo build --release

## Run REPL
cargo run --release --bin nexi_hyperon_repl

## Test Example
In REPL, paste lines from examples/mercy_demo.metta

## Next Steps
- Fill NEXi crates with real zk/PQ logic.
- Extend with Distributed Atomspace or full MeTTa REPL from Hyperon.
- Add more grounded atoms (e.g., Dilithium-signed assertions).
