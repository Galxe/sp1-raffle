//! A program that selects winners for a raffle and generates a Merkle root of the winners

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::sol;
use alloy_sol_types::SolType;
use raffle_lib::raffle_naive;
use sha2_v0_9_8::{Digest as Digest_9_8, Sha256 as Sha256_9_8};

pub fn main() {
    // Read the raffle event data
    let num_participants = sp1_zkvm::io::read::<u32>();
    let num_winners = sp1_zkvm::io::read::<u32>();
    let random_seed = sp1_zkvm::io::read::<u64>();
    let pub_val: PubValStruct;

    // Run the raffle and get winners

    if num_winners > num_participants / 2 {
        let winners = raffle_naive(num_participants, num_winners, random_seed);
        let winners_merkle_root = calculate_merkle_root(&winners);
        pub_val = PubValStruct {
            num_participants,
            num_winners,
            random_seed,
            merkle_root: alloy_sol_types::private::FixedBytes(winners_merkle_root),
        };
    } else {
        let losers = raffle_naive(num_participants, num_winners, random_seed);
        let losers_merkle_root = calculate_merkle_root(&losers);
        pub_val = PubValStruct {
            num_participants,
            num_winners,
            random_seed,
            merkle_root: alloy_sol_types::private::FixedBytes(losers_merkle_root),
        };
    }

    let bytes = PubValStruct::abi_encode(&pub_val);

    // Commit to the public values of the program
    sp1_zkvm::io::commit_slice(&bytes);
}

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PubValStruct {
        uint32 num_participants;
        uint32 num_winners;
        uint64 random_seed;
        bytes32 merkle_root;
    }
}

fn calculate_merkle_root(winners: &[u32]) -> [u8; 32] {
    let mut hashes: Vec<[u8; 32]> = winners
        .iter()
        .map(|&w| {
            let mut hasher = Sha256_9_8::new();
            hasher.update(w.to_le_bytes());
            hasher.finalize().into()
        })
        .collect();

    while hashes.len() > 1 {
        let mut new_hashes = Vec::new();
        for chunk in hashes.chunks(2) {
            let mut hasher = Sha256_9_8::new();
            hasher.update(&chunk[0]);
            if chunk.len() > 1 {
                hasher.update(&chunk[1]);
            }
            new_hashes.push(hasher.finalize().into());
        }
        hashes = new_hashes;
    }

    hashes[0]
}
