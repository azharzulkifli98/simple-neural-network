// A module that can generate a forward feed neural network
// using a more concrete object based approach to how neurons
// interact with each other in the system
pub mod neuron_approach
{
    // This will contain the weights and biases going between neurons
    struct Synapse
    {
        weight: f64,
        previous_weight: f64,
        from_neuron: Neuron,
        to_neuron: Neuron
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
        id: u32,
        neurons: Vec<Neuron>
    }

    // The full set of inputs to outputs that can be trained
    // and tested using neural layers
    struct NeuralNetwork
    {
        feature_layer: NeuralLayer,
        label_layer: NeuralLayer,
        hidden_layers: Vec<NeuralLayer>
    }

    // function implementations begin here
    impl Synapse
    {
        fn update_weight(&self, learning_rate: f64, delta: f64)
        {
            self.previous_weight = self.weight;
            self.weight += learning_rate * delta;
        }

        fn is_from_neuron(id: u32) -> bool
        {
            self.from_neuron.id == id
        }

        fn get_output(&self) -> f64
        {
            self.from_neuron.calculate_output()
        }
    }

    impl Neuron
    {
        fn calculate_input(&self) -> f64
        {
            let sum = 0.0

            for i in 0 .. input_synapses.len()
            {
                sum += input_synapses[i].weight * self.output_value;
            }

            sum
        }

        fn calculate_output(&self, input: f64) -> f64
        {
            // using Rectified activation
            if input > 0.0
            {
                input
            }
            else
            {
                0.0
            }
        }

        pub fn push_output_from_input()
        {
            let value = calculate_input(self.input_synapses);

            let out = calculate_output(value);

            for i in 0 .. self.output_synapses.len()
            {
                //self.output_synapses[i].
            }
        }
    }

    impl NeuralLayer
    {
        fn update_from_layer()
        {
            //
        }

        fn pass_to_layer()
        {
            //
        }
    }

    impl NeuralNetwork
    {
        pub fn setup_network()
        {
            //
        }

        pub fn train()
        {
            //
        }

        // push_input_values

        // push_expected_values

        // calculate_total_error

        // handle_output_layer

        fn forward_propogation()
        {
            //
        }

        fn backward_propogation()
        {
            //
        }
    }
}
