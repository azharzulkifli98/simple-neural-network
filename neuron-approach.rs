// A module that can generate a forward feed neural network
// using a more concrete object based approach to how neurons
// interact with each other in the system
mod neuron_approach
{
    // This will contain the weights and biases going between neurons
    struct Synapse
    {
        weight: f64,
        previous_weight: f64,
        from_neuron: u32,
        to_neuron: u32
    }

    // This will make up the activation layers for the network
    struct Neuron
    {
        id: u32,
        output_value: f64,
        input_synapses: Vec<Synapse>,
        output_synapses: Vec<Synapse>
    }

    // This will contain a layer of neurons and can control flow
    struct NeuralLayer
    {
        id: u32;
        neurons: Vec<Neuron>
    }

    struct NeuralNetwork
    {
        feature_layer: NeuralLayer,
        label_layer: NeuralLayer,
        hidden_layers: Vec<NeuralLayer>
    }

    impl Neuron
    {
        fn area(&self) -> f64
        {
            0.0
        }
    }
}


// placeholder
fn main()
{
    println!("This is a WIP.");
}