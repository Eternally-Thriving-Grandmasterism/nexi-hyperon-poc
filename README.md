# NEXi-Hyperon PoC

**License Update — April 11 2026**  
This project was previously under MIT. As of April 11 2026, it is now under the **Autonomicity Games Sovereign Mercy License (AG-SML)**. New code and future distributions are protected. Past MIT forks remain MIT. Commercial / enterprise use requires a paid license from Autonomicity Games Inc. Individuals may continue using it freely for personal, educational, and daily-living purposes.

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
