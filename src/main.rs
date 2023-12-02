use rand::prelude::*;
use rand_isaac::isaac::IsaacRng;
use sha3::{Digest, Sha3_256};
use std::env;
use std::fs;

struct Group {
    _hash: u64,
    priority: u64,
    name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut groups = Vec::new();

    for line in fs::read_to_string(&args[1]).unwrap().lines() {
        let mut hasher = Sha3_256::new();
        hasher.update(line.as_bytes());
        let result: &[u8] = &hasher.finalize()[..];
        let sum: u64 = result.iter().map(|&x| u64::from(x) * 1200077).sum();
        let mut rng = IsaacRng::seed_from_u64(sum);
        groups.push(Group {
            _hash: sum,
            priority: rng.gen(),
            name: line.to_string(),
        });
    }

    groups.sort_by(|a, b| a.priority.cmp(&b.priority));

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}: {} ({})", i + 1, &group.name, &group.priority);
    }
}
