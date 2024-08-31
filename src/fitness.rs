pub fn fitness(target: &str, agent: &String) -> usize {
    let mut fitness: usize = 0;

    for (i, c) in target.chars().enumerate() {
        if c == agent.chars().nth(i).unwrap() {
            fitness += 1;
        }
    }

    return fitness;
}