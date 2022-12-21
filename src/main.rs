mod gaussian;
mod data;
mod loss;
mod activation;
mod vectors;
mod neuralnetwork;

use data::XnorDataset;
use neuralnetwork::NeuralNetwork;


fn main() {  
    let data: XnorDataset = XnorDataset::new(200);
    data.print(5);  // print some samples
    let mut nn = NeuralNetwork::new(vec![2, 3, 1]);
    nn.training(data.x, data.y, 1000, false);  // set verbose true to see training loss

    println!("\n---------------- Post-training parameters ----------------");
    for n in 0..nn.parameters.weights.len() {
        println!("\nWeights[{}]: ", n);
        nn.parameters.weights[n].print();

        println!("\nBias[{}]: ", n); 
        nn.parameters.biases[n].print();
    }
}