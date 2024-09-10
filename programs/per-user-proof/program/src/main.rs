//! A program that takes the number of participants, number of winners, a random seed,
//! and a participation ID as input, and outputs a boolean indicating whether the user
//! is a winner.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::sol;
use alloy_sol_types::SolType;
use raffle_lib::raffle_naive;

pub fn main() {
    // Read the number of participants
    let num_participants = sp1_zkvm::io::read::<u32>();

    // Read the number of winners
    let num_winners = sp1_zkvm::io::read::<u32>();

    // Read a random seed
    let random_seed = sp1_zkvm::io::read::<u64>();

    // Read the participation ID
    let participation_id = sp1_zkvm::io::read::<u32>();

    println!("cycle-tracker-start: raffle");
    // Select the winners using a function from the workspace lib crate
    let winners = raffle_naive(num_participants, num_winners, random_seed);

    let is_winner = winners.contains(&participation_id);
    println!("cycle-tracker-end: raffle");

    // Encode the public values of the program
    let bytes = PubValStruct::abi_encode(&PubValStruct {
        num_participants,
        num_winners,
        random_seed,
        participation_id,
        is_winner,
    });

    // Commit to the public values of the program
    sp1_zkvm::io::commit_slice(&bytes);
}

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PubValStruct {
        uint32 num_participants;
        uint32 num_winners;
        uint64 random_seed;
        uint32 participation_id;
        bool is_winner;
    }
}
