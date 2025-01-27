use crate::Minet;

impl Minet {
    pub fn display(
        &self,
    ) {
        println!("===== GENOME =====");
        let inputs = self.input as usize;
        let outputs = self.output as usize;
        let length = self.genes.len();
        let hidden_space = length - outputs;
        let synapses_count = self.synapse_count();
        for (i, gene) in self.genes.iter().enumerate() {
            let mut neuron_type: &str = "";
            if i < inputs { 
                neuron_type = "Input";
            }
            if i >= inputs && i < hidden_space {
                neuron_type = "Hidden";
            } 
            if i >= hidden_space {
                neuron_type = "Output";
            }
            println!(
                "{} | {} :: Bias: {}, Synapses: {:.2?}",
                i , neuron_type, gene.0, gene.1
            );

        }
        println!("Synapses: {}", synapses_count);
        println!("Fitness: {}", self.fitness);
    }
    
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph Minet {\n");
        dot.push_str("    // Use left-to-right orientation\n");
        dot.push_str("    rankdir=LR;\n");
        dot.push_str("    graph [layout=dot, fontname=\"Helvetica\", fontsize=12];\n");
        dot.push_str("    node [fontname=\"Helvetica\", fontsize=10];\n");
        dot.push_str("    edge [fontname=\"Helvetica\", fontsize=10];\n\n");
    
        let input_start = 0;
        let input_end = self.input;
        let hidden_start = self.input;
        let hidden_end = self.input + self.hidden;
        let output_start = hidden_end;
        let output_end = hidden_end + self.output;
    
        // Input Layer
        dot.push_str("    subgraph cluster_inputs {\n");
        dot.push_str("        style=filled;\n");
        dot.push_str("        color=\"#cceeff\";\n");
        dot.push_str("        penwidth=1.5;\n");
        dot.push_str("        label=\"Input Layer\";\n");
        dot.push_str("        labelloc=\"top\";\n");
        dot.push_str("        labeljust=\"center\";\n");
        dot.push_str("        rank=same;\n");
        for i in input_start..input_end {
            let bias = self.genes[i].0;
            dot.push_str(&format!(
                "        neuron_{} [shape=box, style=filled, fillcolor=white, penwidth=1.5, label=\"In:{}\\nBias={:.2}\"];\n",
                i, i, bias
            ));
        }
        dot.push_str("    }\n\n");
    
        // Hidden Layer
        dot.push_str("    subgraph cluster_hidden {\n");
        dot.push_str("        style=filled;\n");
        dot.push_str("        color=\"#e6e6e6\";\n");
        dot.push_str("        penwidth=1.5;\n");
        dot.push_str("        label=\"Hidden Layer\";\n");
        dot.push_str("        labelloc=\"top\";\n");
        dot.push_str("        labeljust=\"center\";\n");
        for i in hidden_start..hidden_end {
            let bias = self.genes[i].0;
            dot.push_str(&format!(
                "        neuron_{} [shape=ellipse, style=filled, fillcolor=white, penwidth=1.5, label=\"H:{}\\nBias={:.2}\"];\n",
                i, i, bias
            ));
        }
        dot.push_str("    }\n\n");
    
        // Output Layer
        dot.push_str("    subgraph cluster_outputs {\n");
        dot.push_str("        style=filled;\n");
        dot.push_str("        color=\"#ccffcc\";\n");
        dot.push_str("        penwidth=1.5;\n");
        dot.push_str("        label=\"Output Layer\";\n");
        dot.push_str("        labelloc=\"top\";\n");
        dot.push_str("        labeljust=\"center\";\n");
        dot.push_str("        rank=same;\n");
        for i in output_start..output_end {
            let bias = self.genes[i].0;
            dot.push_str(&format!(
                "        neuron_{} [shape=doublecircle, style=filled, fillcolor=white, penwidth=1.5, label=\"Out:{}\\nBias={:.2}\"];\n",
                i, i, bias
            ));
        }
        dot.push_str("    }\n\n");
    
        // Edges
        dot.push_str("    // Edges\n");
        dot.push_str("    edge [color=\"#555555\", penwidth=1.2];\n");
        for (src, (_, synapses)) in self.genes.iter().enumerate() {
            for &(tgt, weight) in synapses {
                dot.push_str(&format!(
                    "    neuron_{} -> neuron_{} [label=\"{:.2}\", fontcolor=\"#333333\"];\n",
                    src, tgt, weight
                ));
            }
        }
    
        dot.push_str("}\n");
        dot
    }
    
    pub fn dot_to_file(
        &self, 
        filename: &str
    ) -> std::io::Result<()> {
        let dot_representation = self.to_dot();
        std::fs::write(filename, dot_representation)
    }
    
    pub fn display_genome(
        &self,
    ) {
        let encoded = self.encode();
        let piece_length = encoded.len() / self.hidden;
        let mut pieces = Vec::new();
        for i in 0..self.hidden {
            let start = i * piece_length;
            let end = start + piece_length;
            pieces.push(&encoded[start..end]);
        }
        for i in 0..pieces.len() {
            println!("{}",pieces[i]);
        }
    }

}