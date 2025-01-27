mod minet_display;
use std::usize;

pub use minet_display::*;

mod minet_encoding;
pub use minet_encoding::*;

mod minet_activation;
use minet_activation::*;

mod minet_random;
pub use minet_random::*;

use rand::{seq::SliceRandom, thread_rng, Rng};
use rand_distr::{Distribution, Normal};

#[derive(Clone)]
pub struct Minet {
    pub genes: Vec<(f32, Vec<(usize, f32)>)>,
    pub input: usize,
    pub hidden: usize,
    pub output: usize,
}

const INITIAL_WEIGHT_STD_DEVIATION: f32 = 0.5;
const WEIGHT_STD_DEVIATION: f32 = 0.05;
const BIAS_STD_DEVIATION: f32 = 0.01;
const SYNAPSE_PROBABILITY: f64 = 0.1;


impl Minet {
    pub fn new(input: usize, hidden: usize, output: usize) -> Self {
        let total_neurons = input + hidden + output;
        let genes = vec![(0.0, Vec::new()); total_neurons];

        let mut minet = Minet {
            genes,
            input,
            hidden,
            output,
        };
        
        let non_output = input + hidden;

        for i in non_output..total_neurons {
            println!("Output Neuron: {}", i);
            let mut source = minet.connect_random_from(i);
            while source > (input - 1) { 
                source = minet.connect_random_from(source);
            }
        }
        
        minet
    }

    pub fn mutate(&mut self) {
        self.mutate_weights();
        self.mutate_bias();

        let mut rng = rand::thread_rng();
        if rng.gen_bool(SYNAPSE_PROBABILITY) {
            self.synapse_swap();
        }
    }

    pub fn synapse_swap(&mut self) {
        self.synapse_remove_random();
        self.synapse_connect_random();
    }

    pub fn crossbreed(&self, other: &Self) -> Self {
        let mut new_genes: Vec<(f32, Vec<(usize, f32)>)> = Vec::new();
        let parent1 = self.genes.clone();
        let parent2 = other.genes.clone();
        let mut rng = rand::thread_rng();
        for i in 0..parent1.len() {
            let gene1 = parent1[i].clone();
            let gene2 = parent2[i].clone();
            let new_gene = if rng.gen_bool(0.5) { gene1 } else { gene2 };
            new_genes.push(new_gene);
        }
        let child = Minet {
            genes: new_genes,
            input: self.input,
            hidden: self.hidden,
            output: self.output,
        };
        child
    }

    pub fn mutate_weights(&mut self) {
        let mut rng = rand::thread_rng();
        for gene in self.genes.iter_mut() {
            for synapse in gene.1.iter_mut() {
                synapse.1 += sample_normal(WEIGHT_STD_DEVIATION);
            }
        }
    }

    pub fn mutate_bias(&mut self) {
        let mut rng = rand::thread_rng();
        for gene in self.genes.iter_mut() {
            gene.0 += sample_normal(BIAS_STD_DEVIATION);
        }
    }

    pub fn forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        let input_neurons = self.input;
        let hidden_neurons = self.hidden;
        let output_neurons = self.output;
        let length = self.genes.len();

        let mut activation_map = vec![0.0; length];

        // Set input activations and propagate forward
        for i in 0..input_neurons {
            activation_map[i] = inputs[i];
            for &(target_idx, weight) in &self.genes[i].1 {
                activation_map[target_idx as usize] += inputs[i] * weight;
            }
        }

        // Process hidden layer
        for i in input_neurons..(input_neurons + hidden_neurons) {
            // Add bias
            activation_map[i] += self.genes[i].0;
            // Apply ReLU (assume relu(x) = max(0,x))
            activation_map[i] = relu(activation_map[i]);

            // Propagate hidden activations forward
            for &(target_idx, weight) in &self.genes[i].1 {
                activation_map[target_idx as usize] += activation_map[i] * weight;
            }
        }

        // Process output layer
        let mut outputs = Vec::with_capacity(output_neurons);
        for i in (input_neurons + hidden_neurons)..(input_neurons + hidden_neurons + output_neurons)
        {
            // Add bias
            activation_map[i] += self.genes[i].0;
            // Apply Sigmoid
            activation_map[i] = sigmoid(activation_map[i]);
            outputs.push(activation_map[i]);
        }
        outputs
    }
    
    /// Removes a random synapse from the genome, if any synapses exist
    pub fn synapse_remove_random(&mut self) {
        let mut rng = rand::thread_rng();
        let connected_neurons: Vec<(usize, usize)> = self
            .genes
            .iter()
            .enumerate()
            .flat_map(|(i, gene)| gene.1.iter().enumerate().map(move |(j, _)| (i, j)))
            .collect();

        if let Some(&(neuron, synapse)) = connected_neurons.choose(&mut rng) {
            self.genes[neuron].1.remove(synapse);
        } else {
            println!("No synapses to remove.");
        }
    }
    
    /// Connects two random, unconnected neurons in the forward direction. 
    pub fn synapse_connect_random(&mut self) {
        let mut rng = rand::thread_rng();
        let non_output = self.input + self.hidden;

        loop {
            let source = rng.gen_range(0..non_output);
            let target_candidates = self.synapse_candidates(source);
            if let Some(&target) = target_candidates.choose(&mut rng) {
                let weight = sample_normal(INITIAL_WEIGHT_STD_DEVIATION);
                self.genes[source].1.push((target, weight));
                break;
            }
        }
    }
    
    /// Connects a random neuron from an index lower than the to_index. 
    /// ie from an output neuron to a hidden or input neuron in the forward direction
    pub fn connect_random_from(&mut self, to_index: usize) -> usize {
        let mut rng = rand::thread_rng();
        
        let non_output = self.input + self.hidden;
        
        let from_index = rng.gen_range(0..(to_index.clamp(0, non_output)));
        
        let synapse_candidates = self.synapse_candidates(from_index);
        
        println!("{:?}", synapse_candidates);
        
        for i in 0..synapse_candidates.len() {
            let target = synapse_candidates[i];
            if target == to_index {
                let weight = sample_normal(INITIAL_WEIGHT_STD_DEVIATION);
                self.genes[from_index].1.push((to_index, weight));
            }
        }
        from_index
    }
    
    /// Generates candidates for synapse connections with the given criteria
    /// 1. If source is a output neuron, it returns an empty vec
    /// 2. Target index candidates must be larger than source index
    /// 3. Candidates must not already be connected
    /// 4. If the source is a input neuron, all candidates must be hidden or output neurons
    fn synapse_candidates(&self, source: usize) -> Vec<usize> {
        if source >= self.genes.len() - self.output {
            return Vec::new();
        }

        self.genes
            .iter()
            .enumerate()
            .skip(source + 1)
            .filter(|&(i, _)| i >= self.input && !self.synapse_is_connected(source, i))
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Returns true if the two selected neurons are connected by synapse
    fn synapse_is_connected(&self, source: usize, target: usize) -> bool {
        self.genes[source].1.iter().any(|&(t, _)| t == target)
    }

    /// Returns count of synapses in the genome
    pub fn synapse_count(&self) -> usize {
        self.genes.iter().map(|gene| gene.1.len()).sum()
    }
}

fn sample_normal(std_dev: f32) -> f32 {
    let normal = Normal::new(0.0, std_dev).expect("Invalid parameters for Normal distribution");
    let mut rng = thread_rng();
    normal.sample(&mut rng)
}