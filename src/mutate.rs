use rand::{rngs::StdRng, Rng};

pub fn mutate(agent: &mut String, rng: &mut StdRng) {
    let len = agent.len();
    let index = rng.gen_range(0..len);
    let char = rng.gen_range(32..127) as u8 as char;

    agent.replace_range(index..index+1, &char.to_string());
}