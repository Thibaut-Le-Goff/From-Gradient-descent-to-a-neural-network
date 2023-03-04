/*
use crate::runst::DataSet;
use crate::runst::Network;

pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) {

    println!("\n\nL'itération à l'envers :");
    for i in 0..observed_values.len() {
        // for every propagation
        
        println!("\nlors de la propagation numéro {}", i+1);

        // starter of the iteration of the outputs of the propagation i+1:
        let starter_predictions_layers: usize = i * (network_predictions.len() / observed_values.len());
        println!("L'itération commence normalement à : {}", starter_predictions_layers);

        let backward_starter_predictions_layers: usize = (observed_values.len() - i - 1) * (network_predictions.len() / observed_values.len());
        println!("\nMais comme les données sont à l'envers l'itération commence en faite à : {}", backward_starter_predictions_layers);

        for y in 0..weights.len() {
            // for every layers exepte the first because not usefull for the propagation
            // or
            // for every layers of weight

            println!("la sortie des neurones de la couche numéro {} sont :", net.network_struct.len() - y);
            println!("{:?}", network_predictions[backward_starter_predictions_layers + (net.network_struct.len() - y - 2)]);

            for j in 0..weights[y].len() {


                let (weight, bias): (f32, f32) = grad_descent(net, observed_values, j, weights[t][y], bias[t][y]);            
            }
        }
    }
}
*/