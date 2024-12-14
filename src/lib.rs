mod minet_display;
pub use minet_display::*;

mod minet_encoding;
pub use minet_encoding::*;

mod minet_activation;
use minet_activation::*;

mod minet_random;
pub use minet_random::*;

use rand::{seq::SliceRandom, Rng};

#[derive(Clone)]
pub struct Minet {
    pub genes: Vec<(f32, Vec<(usize, f32)>)>,
    input: usize, 
    hidden: usize, 
    output: usize, 
}

const WEIGHT_DELTA: f32 = 0.05;
const BIAS_DELTA: f32 = 0.01;
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
        let total_synapses = minet.synapses_max();
        
        while minet.synapse_count() < total_synapses {
            minet.synapse_connect_random();
        }
        minet
    }
    
    pub fn mutate (
        &mut self, 
    ) {
        self.mutate_weights();
        self.mutate_bias();
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(SYNAPSE_PROBABILITY) {
            self.synapse_swap();
        }
    }
    
    pub fn synapse_swap(
        &mut self,
    ) {
        self.synapse_remove_random();
        self.synapse_connect_random();
    }
    
    pub fn crossbreed(
        &self,
        other: &Self,
    ) -> Self {
        let mut new_genes: Vec<(f32, Vec<(usize, f32)>)> = Vec::new(); 
        let parent1 = self.genes.clone(); 
        let parent2 = other.genes.clone();
        let mut rng = rand::thread_rng();
        for i in 0..parent1.len() {
            let gene1 = parent1[i].clone();
            let gene2 = parent2[i].clone();
            let new_gene = if rng.gen_bool(0.5) {
                gene1
            } else {
                gene2
            };
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
    
    pub fn mutate_weights (
        &mut self,
    ) {
        let mut rng = rand::thread_rng();
        for gene in self.genes.iter_mut() {
            for synapse in gene.1.iter_mut() {
                synapse.1 += rng.gen_range(-WEIGHT_DELTA..WEIGHT_DELTA);
            }
        }
    }
    
    pub fn mutate_bias (
        &mut self,
    ) {
        let mut rng = rand::thread_rng();
        for gene in self.genes.iter_mut() {
            gene.0 += rng.gen_range(-BIAS_DELTA..BIAS_DELTA);
        }
    }
    
    pub fn forward(
        &self, inputs: Vec<f32>
    ) -> Vec<f32> {
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
        for i in (input_neurons + hidden_neurons)..(input_neurons + hidden_neurons + output_neurons) {
            // Add bias
            activation_map[i] += self.genes[i].0;
            // Apply Sigmoid
            activation_map[i] = sigmoid(activation_map[i]);
            outputs.push(activation_map[i]);
        }
        outputs
    }
    
    pub fn synapse_remove_random(&mut self) {
        let mut rng = rand::thread_rng();
        let connected_neurons: Vec<(usize, usize)> = self.genes.iter()
            .enumerate()
            .flat_map(|(i, gene)| gene.1.iter().enumerate().map(move |(j, _)| (i, j)))
            .collect();

        if let Some(&(neuron, synapse)) = connected_neurons.choose(&mut rng) {
            self.genes[neuron].1.remove(synapse);
        } else {
            println!("No synapses to remove.");
        }
    }
    
    pub fn synapse_connect_random(&mut self) {
        let mut rng = rand::thread_rng();
        let non_output = self.input + self.hidden;

        loop {
            let source = rng.gen_range(0..non_output);
            let target_candidates = self.synapse_candidates(source);
            if let Some(&target) = target_candidates.choose(&mut rng) {
                let weight = random_weight();
                self.genes[source].1.push((target, weight));
                break;
            }
        }
    }
    
    pub fn synapses_max(&self) -> usize {
        let max_synapses = (self.input + self.output) * self.hidden;
        max_synapses
    }
    
    fn synapse_candidates(&self, source: usize) -> Vec<usize> {
        if source >= self.genes.len() - self.output {
            return Vec::new();
        }

        self.genes.iter()
            .enumerate()
            .skip(source + 1)
            .filter(|&(i, _)| i >= self.input && !self.synapse_is_connected(source, i))
            .map(|(i, _)| i)
            .collect()
    }
    
    fn synapse_is_connected(&self, source: usize, target: usize) -> bool {
        self.genes[source].1.iter().any(|&(t, _)| t == target)
    }
   
    pub fn synapse_count(&self) -> usize {
        self.genes.iter().map(|gene| gene.1.len()).sum()
    }
}
