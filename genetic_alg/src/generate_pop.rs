use rand::{rngs::StdRng, Rng};

pub fn generate_pop(population: &mut Vec<String>, pop_size: usize, len: usize, rng: &mut StdRng) {

    for _ in 0..pop_size {
        let mut agent: String = String::with_capacity(len);

        for __ in 0..len {
            let char = rng.gen_range(32..127) as u8 as char;
            agent.push(char);
        }

        population.push(agent);
    }

}