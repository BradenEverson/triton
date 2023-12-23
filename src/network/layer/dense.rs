use crate::network::{matrix::Matrix, activations::{Activation, Activations}, input::Input};

use super::layers::Layer;
use serde::{Deserialize, Serialize};

///A Dense Neural Network Layer of a model, containing just nodes, weights, biases and an
///activation function
///Implements the Layer trait
#[derive(Serialize, Deserialize)]
pub struct Dense{
    pub weights: Matrix,   
    pub biases: Matrix,
    pub data: Matrix,
    loss: f32,

    pub activation_fn: Activations,
    learning_rate: f32,

    beta1: f32,
    beta2: f32,
    epsilon: f32,
    time: usize
}

impl Dense{
    pub fn new(layers: usize, layer_cols_before: usize, activation: Activations, learning_rate: f32) -> Dense{
        let mut res = Dense { 
            loss: 1.0,
            weights: Matrix::new_random(layer_cols_before, layers),
            biases: Matrix::new_random(layer_cols_before, 1),
            data: Matrix::new_random(0, 0),
            activation_fn: activation,
            learning_rate,
            beta1: 0.0,
            beta2: 0.0,
            epsilon: 0.0,
            time: 0
        };
        (res.beta1, res.beta2) = res.get_betas();
        res.epsilon = res.get_epsilon();
        res
    }
    fn get_betas(&self) -> (f32, f32){
        (0.9, 0.999)
    }
    fn get_epsilon(&self) -> f32{
        1e-10
    }
}

#[typetag::serde]
impl Layer for Dense{
    ///Moves the DNN forward through the weights and biases of this current layer
    ///Maps an activation function and then returns the resultant Matrix
    fn forward(&mut self, inputs: &Box<dyn Input>) -> Box<dyn Input> {
        self.data = (self.weights.clone() * &Matrix::from(inputs.to_param_2d()).transpose() + &self.biases)
            .map(self.activation_fn.get_function().function);

        Box::new(self.data.clone().transpose())
    }
    ///Does Back Propegation according to simple Dense network rules
    ///Finds the error of the previous layer and returns what the updated weights and biases should
    ///be in that layer, updates the gradients and errors to move backwards once
    ///
    ///Uses Adam optimization algorithm!
    fn backward(&mut self, inputs: &Matrix, gradients: &Matrix, errors: &Matrix, layer_prev: &Matrix, layer_prev_bias: &Matrix) -> (Matrix, Matrix, Matrix, Matrix){
        let mut gradients_mat = gradients.clone().dot_multiply(&errors).map(&|x| x * self.learning_rate);
        let new_layer_prev = layer_prev.clone() + &(gradients_mat.clone() * &self.data.clone().transpose());
        let new_biases = layer_prev_bias.clone() + &gradients_mat.clone();
        
        let errors_mat = layer_prev.clone().transpose() * errors;

        //set error of layer, should have something to do with possibly the MSE of errors_mat,
        //which we could call .to_param() on and iterate through like we do in the network accuracy
        //fn
        self.loss = 0.0;
        errors_mat.to_param().iter().for_each(|error| {
            self.loss += error.powi(2);
        });

        self.loss = self.loss / errors_mat.to_param().len() as f32;

        gradients_mat = self.data.map(self.activation_fn.get_function().derivative);
        (new_biases.clone(), new_layer_prev.clone(), gradients_mat, errors_mat)
    }
    fn get_cols(&self) -> usize {
        self.weights.columns
    }
    fn get_rows(&self) -> usize {
        self.weights.rows
    }
    fn get_weights(&self) -> Matrix {
        self.weights.clone()
    }
    fn set_weights(&mut self, new_weight: Matrix) {
        self.weights = new_weight;
    }
    fn get_bias(&self) -> Matrix {
        self.biases.clone()
    }
    fn set_bias(&mut self, new_bias: Matrix){
        self.biases = new_bias;
    }
    fn get_activation(&self) -> Option<Activations> {
        Some(self.activation_fn.clone())
    }
    fn shape(&self) -> (usize, usize, usize){
        (self.get_rows(), self.get_cols(), 0)
    }
    fn get_loss(&self) -> f32{
        self.loss
    }
}
