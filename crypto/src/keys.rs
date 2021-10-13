mod diversifier;
pub use diversifier::{Diversifier, DIVERSIFIER_LEN_BYTES};

mod nullifier;
pub use nullifier::{NullifierDerivingKey, NullifierPrivateKey, NK_LEN_BYTES};

mod proof;
pub use proof::ProofAuthKey;

mod spend;
pub use spend::{SpendKey, SpendSeed, SPENDSEED_LEN_BYTES};

mod transmission;
pub use transmission::TransmissionKey;

mod view;
pub use view::{
    FullViewingKey, IncomingViewingKey, OutgoingViewingKey, IVK_LEN_BYTES, OVK_LEN_BYTES,
};

pub use decaf377_rdsa::{Binding, SigningKey, SpendAuth, VerificationKey};

#[cfg(test)]
mod tests {
    use super::*;

    use rand_core::OsRng;

    use crate::addresses::PaymentAddress;

    // Remove this when the code is more complete; it's just
    // a way to exercise the codepaths.
    #[test]
    fn scratch_complete_key_generation_happy_path() {
        let mut rng = OsRng;
        let diversifier = Diversifier::generate(&mut rng);
        let sk = SpendKey::generate(&mut rng);
        let proof_auth_key = sk.proof_authorization_key();
        let fvk = sk.full_viewing_key();
        let ivk = IncomingViewingKey::derive(&proof_auth_key.ak, &fvk.nk);
        let pk_d = ivk.derive_transmission_key(&diversifier);
        let _dest = PaymentAddress::new(diversifier, pk_d);
    }
}