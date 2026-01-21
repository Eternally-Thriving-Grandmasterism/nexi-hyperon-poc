use hyperon::{Interpreter, Space};
use nexi_integration::nexi_mercy_space;
use metta_bus_client::DasClient;
use crate::nexi_das_wrapper::{verify_and_gate_atom, sign_atom};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mercy_space = nexi_mercy_space();
    let mut interp = Interpreter::new(mercy_space);

    // Init DAS client (configure with node address/port from env)
    let das_client = DasClient::new("0.0.0.0:50051".to_string())?;  // Example local

    // Hook: On query response, verify/gate before adding to local space
    das_client.on_query_response(move |atoms_with_sigs| {
        for (atom, sig, prov) in atoms_with_sigs {
            if verify_and_gate_atom(&atom, &sig, &prov).is_ok() {
                interp.space().add(atom);  // Only add merciful, verified atoms
            }
        }
    });

    // Example: Sign and broadcast a local atom
    let local_atom = Atom::sym("share eternal thriving");
    let sig = sign_atom(&local_atom);
    das_client.broadcast_atom(local_atom, sig);

    // Keep running (add REPL or server loop)
    println!("NEXi-secured DAS node running...");
    loop {}  // Or integrate full MeTTa runner
}
