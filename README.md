# Miniaturized Neural Evolutionary Topology
minet_ai is a minimalistic Rust library that evolves small neural networks via a genetic algorithm. Inspired by the NEAT (NeuroEvolution of Augmenting Topologies) algorithm, it focuses on producing compact, adaptive networks that grow or shrink to solve the problems they face. These networks are small, fast, and memory-efficient—ideal for NPC AI or autonomous agents where you might need hundreds or thousands of evolving neural networks running simultaneously.

## Differences Between MiNET and NEAT
### Fixed Hidden Neurons
A MiNET network starts with a fixed number of hidden neurons, but it may only use a subset of them. For example, a network with 12 hidden neurons might converge on a solution that uses only 8, leaving 4 neurons disconnected. This design puts emphasis on synapse mutations, speeding up convergence.

### Gene Structure
Genes are defined per neuron (including its outgoing synapses). Every network in a population has the same number of genes (one per neuron), but each gene can vary in length depending on how many synapses it carries.

### Equal Gene Swapping
When crossbreeding, MiNET networks evenly swap genes, where each child gets half its genes from each parent. This can lead to children having more or fewer total synapses than either parent, depending on the selected gene connections.

### Single Topological Mutation
During crossbreeding, there is a rare topological mutation that removes one synapse and adds another randomly. Bias and weight mutations, however, occur every time crossbreeding occurs.

### Indexed Neurons
Neurons have ascending IDs, where inputs have the lowest and outputs the highest. Synapses can only connect to neurons with a higher ID, allowing lower-index hidden neurons to have broader connectivity, and higher-index neurons to have smaller connectively (connecting largely to output neurons). This allows fast, efficient forward passes, and deeper connectivity among nuerons. 
## Usage
Getting started and using `minet_ai` is extremely simple.
### Create a New Network
```rust
    let mut new_network = minet::new(input_neurons, hidden_neurons, output_neurons);
    let mut new_network = minet::new(3, 8, 1);
```
### Updating Fitness
  Networks store their own fitness values, as a f32 where higher is better. Children of crossbreeding have 0 initial fitness.  
```rust
    new_network.fitness += 0.1; 
```
### Running a Forward Pass
```rust
    let input: Vec<f32> = vec![1.0, 2.0, 3.0];
    let mut output: Vec<f32> = Vec::with_capacity(1);
    let output = new_network.forward(input);
```
### Crossbreeding
```rust
    let parent_1 = minet::new(3, 8, 1);
    let parent_2 = minet::new(3, 8, 1);
    
    let child = parent_1.crossbreed(&parent_2);
```
### Creating a Population of Networks
```rust
    let network_population = minet::initialize_population(population_size, inputs, hidden, outputs);
    let network_population = minet::initialize_population(100, 3, 8, 1);
```
### Crossbreed a Population of Networks
Crossbreed a population of minet networks, retatining an elite population where surival_rate% survive and crossbreed amongst themselves to create a new population of target_population size. 
Survival of the elite population is based upon their individual fitness score (minet_network.fitness).
```rust
    let new_network_population = minet::crossbreed_population(network_population, survival_rate, target_population);
    let new_network_population = minet::crossbreed_population(network_population, 0.1, 105);
```
### Visualize A Network as a DOT File

