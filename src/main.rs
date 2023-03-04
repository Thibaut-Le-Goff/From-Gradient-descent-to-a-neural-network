mod runst;
//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    
    //let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9]];
    let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9], vec![2.3], vec![2.9], vec![2.3], vec![2.9]];
    //let observed_values: Vec<Vec<f32>> = vec![vec![1.4], vec![1.9], vec![3.2]];
    let observed_values: Vec<Vec<f32>> = vec![vec![1.4], vec![1.9], vec![3.2], vec![1.9], vec![3.2], vec![1.9], vec![3.2]];

    /* 
    let  datas = runst::DataSet {
        inputs : vec![vec![0.5], vec![2.3], vec![2.9]],
        observed_values : vec![vec![1.4], vec![1.9], vec![3.2]],
    };
    */

    ///////////// Network settings ///////////////////

    let net = runst::Network {
        network_struct : vec![1, 1, 2, 3, 4, 5, 6],
        //network_struct : vec![1, 1],
        distrib : String::from("he_normal_dis"),
    
        hidden_activ_fun : String::from("none"),
        // useless in a 1-1 neural network because 
        //there is no hidden layers

        out_activ_fun : String::from("none"),
    };

    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network

    let (mut weights, mut bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::net_init::net_init(&net);

    ////////////////////// PROPAGATION ////////////////////////////////////
    
    let network_predictions: Vec<Vec<f32>> = runst::propagation::propagation(&net, &inputs ,&weights, &bias);
    //let network_predictions: Vec<f32> = runst::propagation::propagation(&net, &inputs ,&weights, &bias);
    
    ////////////////////// BACK-PROPAGATION ////////////////////////////////////

    //let (mut trained_weights, mut trained_bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::back_prop::back_prop(&net, &observed_values, &network_predictions ,&weights, &bias);

    ///////////////////// MONTRE LES DONNÉES À L'ENVERS ////////////////////
    println!("\nLes données en brut (à l'endroit) :");
    for i in 0..network_predictions.len() {
        println!("{:?}", network_predictions[i]);
    }


    println!("\n\nL'itération à l'endroit :");
    //let network_layers_backprop_iterator: 
    for i in 0..inputs.len() {
        // for every propagation

        println!("\nlors de la propagation numéro {}", i+1);

        // starter of the iteration of the outputs of the propagation i+1:
        let starter_predictions_layers: usize = i * (network_predictions.len() / inputs.len());
        println!("L'itération commence à : {}\n", starter_predictions_layers);

        for y in 0..net.network_struct.len() - 1 {
            // for every layer exepte the first because not usefull for the propagation

            println!("la sortie des neurones de la couche numéro {} sont :", y + 2);
            println!("{:?}", network_predictions[starter_predictions_layers + y]);
        }
    }

    

    println!("\n\nL'itération à l'envers :");
    for i in 0..inputs.len() {
        // for every propagation
        
        println!("\nlors de la propagation numéro {}", i+1);

        // starter of the iteration of the outputs of the propagation i+1:
        let starter_predictions_layers: usize = i * (network_predictions.len() / inputs.len());
        println!("L'itération commence normalement à : {}", starter_predictions_layers);

        let backward_starter_predictions_layers: usize = (inputs.len() - i - 1) * (network_predictions.len() / inputs.len());
        println!("\nMais comme les données sont à l'envers l'itération commence en faite à : {}", backward_starter_predictions_layers);

        for y in 0..weights.len() {
            // for every layers exepte the first because not usefull for the propagation
            // or
            // for every layers of weight

            println!("la sortie des neurones de la couche numéro {} sont :", weights.len() - y + 1);
            println!("{:?}", network_predictions[backward_starter_predictions_layers + (weights.len() - y - 1)]);
        }
    }


    println!("\n\n\nMais les données doivent être gérées autrement :");
    for i in 0..inputs.len() {
        // for every propagation
        
        println!("\nlors de la propagation numéro {}", i+1);

        // starter of the iteration of the outputs of the propagation i+1:
        let starter_predictions_layers: usize = i * (network_predictions.len() / inputs.len());
        println!("L'itération commence normalement à : {}", starter_predictions_layers);

        let backward_starter_predictions_layers: usize = (inputs.len() - i - 1) * (network_predictions.len() / inputs.len());
        println!("\nMais comme les données sont à l'envers l'itération commence en faite à : {}", backward_starter_predictions_layers);

        for y in 0..weights.len() {
            // for every layers exepte the first because not usefull for the propagation
            // or
            // for every layers of weight

            println!("la sortie des neurones de la couche numéro {} sont :", weights.len() - y + 1);
            println!("{:?}", network_predictions[backward_starter_predictions_layers + (weights.len() - y - 1)]);
        }
    }
}