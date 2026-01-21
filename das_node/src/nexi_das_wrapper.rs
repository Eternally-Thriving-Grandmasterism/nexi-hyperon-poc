use hyperon::{Atom, Space};
use metta_bus_client::{DasClient, Query};  // From DAS client
use mercy_quanta::{MercyQuanta, ValenceProof};
use dilithium_sign::{DilithiumSigner, DilithiumVerifier};

// Wrapper: Verify + gate incoming atoms
pub fn verify_and_gate_atom(atom: &Atom, signature: &[u8], provenance: &str) -> Result<bool, String> {
    // PQ signature check
    let verifier = DilithiumVerifier::new();
    let serialized = atom.to_string().into_bytes();
    if !verifier.verify(&serialized, signature) {
        return Err("PQ signature invalid".into());
    }

    // Mercy valence proof (assume atom grounds a description or proof)
    let quanta = MercyQuanta::new();
    let proof: ValenceProof = quanta.evaluate_valence(&serialized)?;
    
    if proof.is_positive() && proof.is_conflict_free() {
        Ok(true)
    } else {
        Err("Mercy gate blocked: unkind propagation".into())
    }
}

// Sign outgoing atoms
pub fn sign_atom(atom: &Atom) -> Vec<u8> {
    let signer = DilithiumSigner::new();
    let serialized = atom.to_string().into_bytes();
    signer.sign(&serialized)
}
