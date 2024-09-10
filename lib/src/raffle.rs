use sha2_v0_9_8::{Digest as Digest_9_8, Sha256 as Sha256_9_8};

/// Naive raffle implementation
pub fn raffle_naive(num_participants: u32, num_winners: u32, random_seed: u64) -> Vec<u32> {
    let mut winners = Vec::new();
    let mut seed = random_seed;
    let n = num_participants;
    let m = num_winners;

    for _ in 0..m {
        let new_winner = (seed % n as u64) as u32;
        if winners.contains(&new_winner) {
            seed = hash(seed, new_winner as u64);
            continue;
        } else {
            winners.push(new_winner);
            seed = hash(seed, new_winner as u64);
        }
    }

    winners
}

/// Raffle implementation using the Fisher-Yates shuffle
pub fn raffle_fisher_yates(num_participants: u32, num_winners: u32, random_seed: u64) -> Vec<u32> {
    let mut participants: Vec<u32> = (0..num_participants).collect();
    let mut seed = random_seed;

    for i in (1..num_participants).rev() {
        seed = hash(seed, i as u64);
        let j = (seed % (i + 1) as u64) as usize;
        participants.swap(i as usize, j);
    }

    participants
        .into_iter()
        .take(num_winners as usize)
        .collect()
}

fn hash(seed: u64, value: u64) -> u64 {
    let mut sha256 = Sha256_9_8::new();
    sha256.update(seed.to_le_bytes());
    sha256.update(value.to_le_bytes());
    let result = sha256.finalize();
    u64::from_le_bytes(result[..8].try_into().unwrap())
}
