mod fitness;
mod generate_pop;
mod mate;
mod mutate;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    // Variables
    let target: &str = "Hello world!";
    let len: usize = target.len();

    let pop_size: usize = 100;
    let mut gen: usize = 0;
    let mut population: Vec<String> = Vec::with_capacity(pop_size);

    let seed: u64 = 0;
    let mut rng: StdRng = StdRng::seed_from_u64(seed);

    // Initialize population
    generate_pop::generate_pop(&mut population, pop_size, len, &mut rng);

    println!("Target: {}", target);
    println!("Population: {:?}", population);

    // Main loop
    loop {
        gen += 1;
        let fitnesses: Vec<usize> = population.iter().map(|agent| fitness::fitness(target, agent)).collect();
        let best_agent = *fitnesses.iter().max().unwrap();

        println!("Best agent: {}", population[best_agent]);

        if fitnesses[best_agent] == len {
            break;
        }

        let mut new_population: Vec<String> = Vec::with_capacity(pop_size);

        for _ in 0..pop_size {
            // Select two agents
            let mut distr: rand::distributions::WeightedIndex<usize> = rand::distributions::WeightedIndex::new(&fitnesses).unwrap();
            let a1_index: usize = rng.sample(distr.clone());
            let agent1: &String = &population[a1_index];
            distr.update_weights(&[(a1_index, &0)]).unwrap();
            let agent2: &String = &population[rng.sample(distr)];

            // Mate
            let child: &mut String = &mut mate::mate(agent1, agent2, &mut rng);

            if rng.gen_bool(0.5){
                mutate::mutate(child, &mut rng);
            }

            new_population.push(child.to_string());
        }

        population = new_population;
    }
    println!("Found in {} generations", gen);
    println!("Final population: {:?}", population);
}
