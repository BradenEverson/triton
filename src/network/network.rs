use super::layer::layers::{Layer, LayerTypes};
use super::{matrix::Matrix, activations::Activations, modes::Mode};
use super::input::Input;

pub struct Network {
    pub layer_sizes: Vec<usize>,
    pub loss: f32,
    pub layers: Vec<Box<dyn Layer>>,
    uncompiled_layers: Vec<LayerTypes>
}

const ITERATIONS_PER_EPOCH: usize = 10000;

impl Network{
    ///Creates a new neural network that is completely empty
    ///
    ///Example:
    ///```
    ///let mut new_net = Network::new();
    ///```
    pub fn new() -> Network{
        Network{
            layer_sizes: vec![],
            loss: 1.0,
            layers: vec![],
            uncompiled_layers: vec![]
        }
    } 
    ///Adds a new Layer to the queue of a neural network
    ///
    ///# Arguments
    ///* `layer` - An enum depicting the options available from the Layers that exist(Dense,
    ///Convolutional, etc)
    ///
    ///# Example
    ///
    ///```
    ///let mut new_net = Network::new();
    ///new_new.add_layer(LayerTypes::Dense(4, Activations::SIGMOID, 0.01));
    ///```
    ///Adds a new Dense layer of 4 nodes with the sigmoid activation and a learning rate of 0.01
    pub fn add_layer(&mut self, layer: LayerTypes){
        self.layer_sizes.push(layer.get_size());
        self.uncompiled_layers.push(layer);
    }
    ///Compiles a network by constructing each of its layers accordingly
    ///Must be done after all layers are added as the sizes of layer rows depends on the columns of
    ///the next layer
    pub fn compile(&mut self){
        for i in 0..self.uncompiled_layers.len() - 1 {
            let layer = self.uncompiled_layers[i].to_layer(self.layer_sizes[i+1]);
            self.layers.push(layer);
        }
        //println!("{:?}", self.layer_sizes);

    }
    ///Travels through a neural network's abstracted Layers and returns the resultant vector at the
    ///end
    ///
    ///# Arguments
    ///* `input_obj` - Any structure that implements the Input trait to act as an input to the data
    ///# Returns
    ///A vector at the end of the feed forward
    ///
    ///# Examples
    ///
    ///```
    ///let new_net = Network::New();
    ///new_new.add_layer(LayerTypes::Dense(2, Activations::SIGMOID, 0.01));
    ///new_new.add_layer(LayerTypes::Dense(3, Activations::SIGMOID, 0.01));
    ///new_new.add_layer(LayerTypes::Dense(4, Activations::SIGMOID, 0.01));
    ///new_new.add_layer(LayerTypes::Dense(2, Activations::TANH, 0.01));
    ///new_new.add_layer(LayerTypes::Dense(1, Activations::SIGMOID, 0.01));
    ///
    ///new_net.compile()
    ///
    ///let res = new_net.feed_forward(vec![1.0, 0.54]);
    ///```
    pub fn feed_forward<Param: Input>(&mut self, input_obj: &Param) -> Vec<f32> {
        let inputs = input_obj.to_param();
        if inputs.len() != self.layer_sizes[0] {
            panic!("Invalid number of inputs");
        }
        
        let mut data_at: Matrix = Matrix::from(vec![inputs.clone()]).transpose();
        for i in 0..self.layers.len(){
            let data_in: Box<dyn Input> = Box::new(data_at);
            data_at = Matrix::from(self.layers[i].forward(&data_in).to_param_2d());
        }
        data_at.transpose().data[0].to_owned()
    }
    ///Travels backwards through a neural network and updates weights and biases accordingly
    ///
    ///The backward behavior is different depending on the layer type, and therefore the weight and
    ///bias updating is different as well
    ///
    ///When constructing a neural network, be cautious that your layers behave well with each other
    fn back_propegate<Param: Input>(&mut self, outputs: Vec<f32>, target_obj: &Param) {
        let targets = target_obj.to_param();
        if targets.len() != self.layer_sizes[self.layer_sizes.len()-1]{
            panic!("Output size does not match network output size");
        }
        let mut parsed = Matrix::from(vec![outputs]).transpose();
        
        let mut errors = Matrix::from(vec![targets.clone()]) - &parsed; 
        
        if let None = self.layers[self.layers.len()-1].get_activation() {
            panic!("Output layer is not a dense layer");
        }

        let mut gradients = parsed.map(self.layers[self.layers.len()-1].get_activation().unwrap().get_function().derivative);
        let target_matrix = Matrix::from(vec![targets.clone()]);
        let mut new_weights = Matrix::new_random(0,0);
        let mut new_bias = Matrix::new_random(0,0);
        for i in (0..self.layers.len() - 1).rev() {
            let layers_prev = self.layers[i+1].get_weights();
            let bias_prev = self.layers[i+1].get_bias();
            (new_bias, new_weights, gradients, errors) = self.layers[i].backward(&target_matrix, &gradients, &errors, &layers_prev, &bias_prev);
            self.layers[i+1].set_weights(new_weights);
            self.layers[i+1].set_bias(new_bias);
        }
    }
    ///Trains a neural network by iteratively feeding forward a series of inputs and then doing
    ///back propegation based on the outputs supplied
    ///
    ///# Arguments
    ///* `train_in` - A vector of objects that implement the Input trait, used as the training
    ///input
    ///* `train_out` - A vector of objects that implement the Input trait, used as the results
    ///compared to what is actually derived during back propegation
    ///* `epochs` - How many epochs you want your model training for
    ///
    pub fn fit<Param: Input>(&mut self, train_in: Vec<Param>, train_out: Vec<Param>, epochs: usize){
        for _ in 0..epochs {
            for _ in 0..ITERATIONS_PER_EPOCH{
                for input in 0..train_in.len(){
                 let outputs = self.feed_forward(&train_in[input]);
                 self.back_propegate(outputs, &train_out[input])
                }
            }
        }
        println!("Trained");
    }
}
