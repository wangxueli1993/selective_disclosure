// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

//! Config of anonymous ciphertext voting (ACV) solution.

use wedpr_l_crypto_hash_keccak256::WedprKeccak256;
use wedpr_l_crypto_signature_secp256k1::WedprSecp256k1Recover;

lazy_static! {
    /// Shared signature algorithm reference for quick implementation replacement.
    pub static ref SIGNATURE: WedprSecp256k1Recover =
        WedprSecp256k1Recover::default();
    /// Shared hash algorithm reference for quick implementation replacement.
    pub static ref HASH: WedprKeccak256 = WedprKeccak256::default();
}

/// Poll result key for total ballots.
pub const POLL_RESULT_KEY_TOTAL_BALLOTS: &str = "Wedpr_voting_total_ballots";
