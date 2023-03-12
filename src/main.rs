mod runst;
//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    
    //let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9]];
    let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9], vec![2.3], vec![2.9], vec![2.3], vec![2.9]];
    //let inputs: Vec<Vec<f32>> = vec![vec![0.5, 0.5], vec![2.3, 0.5], vec![2.9, 0.5], vec![2.3, 0.5], vec![2.9, 0.5], vec![2.3, 0.5], vec![2.9, 0.5]];
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

    println!("\n\nA l'envers :");

    for i in 0..weights.len() {
        // for every layers of weigths
        println!("\nDans la couche de poids numéro {} :", weights.len() - i);

        let mut layers_counter: usize = weights.len() - i -1;

        for y in 0..inputs.len() {
            // for every propagations
            println!("Les données de la propagation numéro {} sont :", y + 1);
            //println!("lors de la propagation numéro {:?}", network_predictions[layers_counter]);

            let vec_layer: &Vec<f32> = &network_predictions[layers_counter];

            for j in 0..vec_layer.len() {
                println!("{}", vec_layer[j]);
            }
            
            layers_counter += weights.len();
        }
    }

    //println!("\n\nLes données en brut (à l'endroit) :");
    //println!("{:?}", network_predictions);

    println!("\n\nLes données en brut (à l'endroit en un seul vecteur) :");
    let network_predictions_concat: &Vec<f32> = &network_predictions.concat();
    println!("{:?}, cela fait {} données", network_predictions_concat, network_predictions_concat.len());

    //for i in 0..net.network_struct.len() - 1 {
    
    let mut number_neurons: usize = 0;
    for i in 0..weights.len() {
        // for every layer but the first one
        // or
        // for every layers of weights
        number_neurons += net.network_struct[i + 1];
    }
    println!("\n\nThe number of neurons in this network is {}", number_neurons);

    
    for i in 0..number_neurons {
        // for every neurons of the network
        let mut vec_neuron: Vec<f32> = Vec::new();

        for y in 0..observed_values.len() {
            // for every propagation
            vec_neuron.push(network_predictions_concat[i + y * number_neurons]);
        }

        println!("The neuron number {} have those values as output: {:?}", i + 1, vec_neuron);
    }

    println!("\n\nA l'envers :");
    for i in 0..number_neurons {
        // for every neurons of the network
        let mut vec_neuron: Vec<f32> = Vec::new();

        for y in 0..observed_values.len() {
            // for every propagation
            let test: usize = i + y * number_neurons;
            let test_reverse: usize = network_predictions_concat.len() - 1 - i - y * number_neurons;
            //println!("{}, reverse : {}", test, test_reverse);

            vec_neuron.push(network_predictions_concat[test_reverse]);
        }

        println!("The neuron number {} have those values as output: {:?}", number_neurons - i, vec_neuron);
    }

}