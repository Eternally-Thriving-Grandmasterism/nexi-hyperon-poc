nexi-hyperon-poc/
├── .gitmodules
├── Cargo.toml                   # Update workspace members + DAS deps
├── README.md                    # Add DAS setup instructions
├── examples/
│   ├── mercy_demo.metta
│   └── das_query.metta          # New: Test distributed mercy-gated queries
├── integration/                 # Existing: Mercy grounded atoms
├── repl/                        # Existing: Local REPL
├── das_node/                    # New crate: Secured DAS node binary
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs              # Runs Hyperon + DAS client with NEXi hooks
│       └── nexi_das_wrapper.rs  # Verification on atom ingress/egress
└── deps/
    ├── hyperon-experimental/
    ├── NEXi/
    └── das/                         # New submodule: singnet/das
