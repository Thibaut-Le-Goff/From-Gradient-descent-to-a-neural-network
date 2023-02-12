/* 
use crate::runst::DataSet;
use crate::runst::Network;

pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) {
    for i in 0..net.network_struct.len() - 1 {
        // -1 to avoid the inputs layers because it deosn't have a bias and weights in front of it
        grad_descent(inputs: &Vec<f32>, net: &Network, observed_values: &Vec<f32>, weights: &Vec<f32>, bias: &Vec<f32>)
    }
}
*/