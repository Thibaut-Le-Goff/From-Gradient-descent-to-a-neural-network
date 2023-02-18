/* 
use crate::runst::DataSet;
use crate::runst::Network;

pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) {

    for i in 0..observed_values.len() {
        // for every propagation
        //for j in 0..net.network_struct.len() - 1 {
            // for every layers but :
            // -1 to avoid the inputs layers because it deosn't have a bias and weights in front of it       
        for j in 0..weight.len() {
            // for every matrix of weight, because there is one between each layers
            let t: usize = weight.len() - 1;

            for y in 0..net.network_struct[t + 1].len() {
                // t + 1 because if t = 0 it would take the first layer
                // for every neurons at the layer j 
                //let layer_predictions: 

                let x: usize = network_predictions.len() - 1;

                let (weight, bias): (f32, f32) = grad_descent(net, observed_values, network_predictions[x][y], weights[t][y], bias[t][y]);
            }
        }
    }
}*/