# Genetic Algorithm String Matcher

This project implements a genetic algorithm to evolve a population of strings to match a target string. It's written in Rust and demonstrates basic concepts of genetic algorithms such as population generation, fitness evaluation, selection, crossover (mating), and mutation.

## Project Structure

The project is organized into several Rust files:

- `src/main.rs`: The entry point of the program, containing the main loop of the genetic algorithm.
- `src/generate_pop.rs`: Handles the initial population generation.
- `src/fitness.rs`: Implements the fitness function to evaluate how close a string is to the target.
- `src/mate.rs`: Contains the crossover (mating) function to create new offspring.
- `src/mutate.rs`: Implements the mutation function to introduce random changes.

## How It Works

1. The program starts by generating an initial population of random strings.
2. In each generation:
   - The fitness of each string is calculated based on how many characters match the target string.
   - The best-fitting string is printed.
   - If a perfect match is found, the program terminates.
   - Otherwise, a new population is created through selection, mating, and mutation.
3. This process repeats until a perfect match is found.

## Key Components

### Population Generation
- Function: `generate_pop::generate_pop()`
- Creates a population of random strings of the specified length.

### Fitness Evaluation
- Function: `fitness::fitness()`
- Calculates how many characters in a string match the target string.

### Mating (Crossover)
- Function: `mate::mate()`
- Creates a new string by randomly selecting characters from two parent strings.

### Mutation
- Function: `mutate::mutate()`
- Randomly changes a character in a string to introduce variation.

## Running the Program

To run the program, use the following command in the project directory:

```
cargo run
```

The program will output the target string, the best-matching string in each generation, and the final population when a match is found.

## Customization

You can customize the genetic algorithm by modifying the following variables in `main.rs`:

- `target`: The target string to evolve towards.
- `pop_size`: The size of the population in each generation.
- `seed`: The random seed for reproducibility.

## Note

This is a basic implementation of a genetic algorithm and may not be optimized for large-scale or complex problems. It serves as an educational example of how genetic algorithms work.