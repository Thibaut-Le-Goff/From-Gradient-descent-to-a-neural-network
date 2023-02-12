mod runst;
//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    
    let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9]];
    let observed_values: Vec<Vec<f32>> = vec![vec![1.4], vec![1.9], vec![3.2]];

    /* 
    let  datas = runst::DataSet {
        inputs : vec![vec![0.5], vec![2.3], vec![2.9]],
        observed_values : vec![vec![1.4], vec![1.9], vec![3.2]],
    };
    */

    ///////////// Network settings ///////////////////

    let net = runst::Network {
        network_struct : vec![1, 1],
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

    let network_predictions: Vec<f32> = runst::propagation::propagation(&net, &inputs ,&weights, &bias);

    println!("{:?}", network_predictions);
    ////////////////////// BACK-PROPAGATION ////////////////////////////////////

    //let (mut trained_weights, mut trained_bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::back_prop::back_prop(&net, &observed_values, &network_predictions ,&weights, &bias);

    ///////////////////// MONTRE LES DONNÉES À L'ENVERS ////////////////////
    /*let outputs_sum_bias: usize = network_outputs_sum_bias.len();
    let outputs_neurons: usize = network_outputs_neurons.len();
    let couche_totale: usize = network_struct.len();
    // pour un nombre qui est :
    //   network_struct.len() = le nombre de couches dans le réseau
    //   network_struct.len() *  1 = multiplier par le nb de données enregistrées 
    //                      sum_bias et activ_fun pour un autre vecteur
    //   network_struct.len() = moins la donnée n'existants pas
    //                     à la couche input

    println!("\n\nCe que le réseaux me donne à l'enver :");
    for prop in 0..inputs.len() {
        // pour chaque propagation
        println!("\n\nÀ la propagation numéro {} :", (inputs.len() - 1) - prop);

        println!("Dans les neurones de la couche 1 à 2 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale))]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_neurons - ((prop * couche_totale))]);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + 1)]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + 1)]);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        println!("{:?}\n", inputs[inputs.len() - prop + 1]);

         
        for couche in 0..couche_totale {
            // pour chaque couches, network_struct.len()
            println!("Les neurons :");
            for neuron in 0..network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche + 1)].len() {
                // pour chaque neuron de la couche j de l'itération i
                println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche + 1)][neuron]);
            }
            println!("Les sum_bias :");
            for sum_bias in 0..network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche + 2)].len() {
                // pour chaque sum_bias de la couche j de l'itération i
                println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche + 2)][sum_bias]);
            }
        }
        println!("Les neurons :");
        println!("{:?}\n", inputs[(inputs.len()) - (prop + 1)]);
    }  */
}