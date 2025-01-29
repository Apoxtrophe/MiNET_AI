use minet_ai::*;
use rand::Rng;

const POPULATION: usize = 100;
const SURVIVAL_RATE: f32 = 0.1;
const GENERATIONS: usize = 1000;
const ITERATIONS: usize = 100; 
const PERFECT_FITNESS_THRESHOLD: f32 = 0.99999;


// Define the XOR problem data
const TABLE_INPUTS: &[&[f32]] = &[
    &[0.0, 0.0, 0.0],
    &[0.0, 0.0, 1.0],
    &[0.0, 1.0, 0.0],
    &[0.0, 1.0, 1.0],
    &[1.0, 0.0, 0.0],
    &[1.0, 0.0, 1.0],
    &[1.0, 1.0, 0.0],
    &[1.0, 1.0, 1.0],
];

const TABLE_EXPECTED: &[&[f32]] = &[
    &[0.0, 0.0],
    &[1.0, 0.0],
    &[1.0, 0.0],
    &[0.0, 1.0],
    &[1.0, 0.0],
    &[0.0, 1.0],
    &[0.0, 1.0],
    &[1.0, 1.0],
];

fn main() {

    
    let mut generation_sum = 0;
    for iteration in 0..ITERATIONS {
        let mut population = initialize_population(POPULATION, 3, 5, 2);
        print_header();

        for generation in 1..=GENERATIONS {
            evaluate_population_fitness(&mut population, TABLE_INPUTS, TABLE_EXPECTED);
    
            if let Some(best) = get_best_network(&population) {
    
                if best.fitness >= PERFECT_FITNESS_THRESHOLD {
                    println!(
                        "\nPerfect network found at generation {} with fitness {:.5}.",
                        generation, best.fitness
                    );
                    print_generation(generation, best, TABLE_EXPECTED, TABLE_INPUTS);
                    if iteration == ITERATIONS - 1 {
                        best.dot_to_file("best_network.dot");
                    }


                    break;
                }
            }
            generation_sum += 1;
            population = crossbreed_population(population, SURVIVAL_RATE, POPULATION);
        }
    }
    println!("!!!   Average Generations to Perfection:: {}", generation_sum / ITERATIONS);
}

/// Initializes the population with random neural networks.
fn initialize_population(pop_size: usize, inputs: usize, hidden: usize, outputs: usize) -> Vec<Minet> {
    (0..pop_size).map(|_| Minet::new(inputs, hidden, outputs)).collect()
}

/// Evaluates and assigns fitness scores to each network in the population.
fn evaluate_population_fitness(population: &mut Vec<Minet>, inputs: &[&[f32]], expected: &[&[f32]]) {
    population.iter_mut().for_each(|network| {
        let outputs: Vec<f32> = inputs.iter().map(|&input| network.forward(input.to_vec())[0]).collect();
        network.fitness = calculate_fitness(&outputs, &extract_first_column(expected));
    });
}

/// Extracts the first element from each expected output pair.
fn extract_first_column(data: &[&[f32]]) -> Vec<f32> {
    data.iter().map(|&pair| pair[0]).collect()
}

/// Retrieves the network with the highest fitness in the population.
fn get_best_network(population: &[Minet]) -> Option<&Minet> {
    population.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
}

/// Prints the table header.
fn print_header() {
    println!(
        "{:<10} | {:<8} | {:<30} | {:<30}",
        "Generation", "Fitness", "Expected Output", "Actual Output"
    );
    println!("{}", "-".repeat(85));
}

/// Prints the details of the current generation's best network.
fn print_generation(
    generation: usize,
    network: &Minet,
    expected: &[&[f32]],
    inputs: &[&[f32]],
) {
    let best_outputs: Vec<String> = inputs
        .iter()
        .map(|&input| format!("{:.2}", network.forward(input.to_vec())[0]))
        .collect();
    let expected_str = extract_first_column(expected)
        .iter()
        .map(|&val| format!("{:.2}", val))
        .collect::<Vec<_>>()
        .join(", ");

    let actual_str = best_outputs.join(", ");

    println!(
        "{:<10} | {:<8.5} | {:<30} | {:<30}",
        generation, network.fitness, expected_str, actual_str
    );
}

/// Calculates the fitness score based on Mean Squared Error (MSE).
fn calculate_fitness(outputs: &[f32], expected: &[f32]) -> f32 {
    let mse: f32 = outputs
        .iter()
        .zip(expected.iter())
        .map(|(o, e)| (o - e).powi(2))
        .sum::<f32>()
        / outputs.len() as f32;
    1.0 / (1.0 + mse) // Higher fitness for lower MSE
}
