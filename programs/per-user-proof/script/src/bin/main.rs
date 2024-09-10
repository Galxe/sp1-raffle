use clap::Parser;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};

use alloy_sol_types::sol;
use alloy_sol_types::SolType;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "100")]
    num_participants: u32,

    #[clap(long, default_value = "10")]
    num_winners: u32,

    #[clap(long, default_value = "12345")]
    random_seed: u64,

    #[clap(long, default_value = "1")]
    participation_id: u32,
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

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.num_participants);
    stdin.write(&args.num_winners);
    stdin.write(&args.random_seed);
    stdin.write(&args.participation_id);

    println!("Number of participants: {}", args.num_participants);
    println!("Number of winners: {}", args.num_winners);
    println!("Random seed: {}", args.random_seed);
    println!("Participation ID: {}", args.participation_id);

    // Generate and verify the proof
    let client = ProverClient::new();

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PubValStruct::abi_decode(output.as_slice(), true).unwrap();
        let PubValStruct {
            num_participants,
            num_winners,
            random_seed,
            participation_id,
            is_winner,
        } = decoded;

        // Verify that the output matches the input
        assert_eq!(
            num_participants, args.num_participants,
            "Number of participants mismatch"
        );
        assert_eq!(num_winners, args.num_winners, "Number of winners mismatch");
        assert_eq!(random_seed, args.random_seed, "Random seed mismatch");
        assert_eq!(
            participation_id, args.participation_id,
            "Participation ID mismatch"
        );
        println!("All input values match the output values.");

        println!("Is winner: {}", is_winner);

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else if args.prove {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);

        let proof = client.prove(&pk, stdin).run().unwrap();

        client.verify(&proof, &vk).expect("verification failed");

        // Test a round trip of proof serialization and deserialization.
        proof
            .save("per-user-proof.bin")
            .expect("saving proof failed");
        let deserialized_proof =
            SP1ProofWithPublicValues::load("per-user-proof.bin").expect("loading proof failed");

        // Verify the deserialized proof.
        client
            .verify(&deserialized_proof, &vk)
            .expect("verification failed");

        println!("successfully generated and verified proof for the program!");
    } else {
        println!("Please specify either --execute or --prove");
    }
}
