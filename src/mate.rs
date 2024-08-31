use rand::{rngs::StdRng, Rng};

pub fn mate(agent1: &String, agent2: &String, rng: &mut StdRng) -> String {
    let len = agent1.len();
    let mut child = String::with_capacity(len);

    for i in 0..len {
        if rng.gen_bool(0.5) {
            child.push(agent1.chars().nth(i).unwrap());
        }
        else {
            child.push(agent2.chars().nth(i).unwrap());
        }
    }

    return child;
}