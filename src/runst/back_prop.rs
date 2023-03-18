/*
use crate::runst::DataSet;
use crate::runst::Network;

pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) {

    for i in 0..weights.len() {
    //for i in 0..weights.len() {
            // for every layers 
        println!("\nDans la couche de poids numéro {} :", weights.len() - i);

        let mut layers_counter: usize = weights.len() - i -1;

        for y in 0..inputs.len() {
            // for every propagations
            println!("Les poids de la propagation numéro {} sont :", y + 1);
            println!("lors de la propagation numéro {:?}", network_predictions[layers_counter]);


            layers_counter += weights.len();
        }
    }
        let (weight, bias): (f32, f32) = grad_descent(net, observed_values, j, weights[t][y], bias[t][y]);            

}
*/