mod minet_display;
pub use minet_display::*;

mod minet_encoding;
pub use minet_encoding::*;

mod minet_activation;
use minet_activation::*;

mod minet_random;
pub use minet_random::*;

use rand::{seq::SliceRandom, Rng};

pub struct Minet {
    pub genes: Vec<(f32, Vec<(usize, f32)>)>,
    input: usize, 
    hidden: usize, 
    output: usize, 
}

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
    
    pub fn synapse_swap(
        &mut self,
    ) {
        self.synapse_remove_random();
        self.synapse_connect_random();
    }
    
    pub fn cross(
        &self,
        other: &Self,
    ) { 9
        
    }
    
    fn synapse_remove_random(&mut self) {
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
    
    fn synapse_connect_random(&mut self) {
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
   
    fn synapse_count(&self) -> usize {
        self.genes.iter().map(|gene| gene.1.len()).sum()
    }
}
