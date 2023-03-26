/* 
use crate::runst::DataSet;
use crate::runst::Network;

pub fn grad_descent(net: &Network, observed_values: &Vec<f32>, network_predictions: &Vec<f32>, weights: &Vec<f32>, bias: &Vec<f32>) -> (weight: <f32>, bias: <f32>) {

    let try_number: usize = 1000;

    let mut weight_bias_trouve: [bool; 2] = [false, false];
    let mut true_counter: usize = 0;
    
    let precision_success: f32 = 0.001;

    let mut step_size: f32;

    let power_dif : f32 = 2.0;

    let weight_bias_learning_rate: [f32; 2] = [0.01, 0.1];
    let mut weight_bias: [f32; 2] = [weight[0][0], bias[0][0]];

    let mut sum_derivative_square_residual: f32;
    let mut derivative_square_residual: f32;

    let mut predicted_value: f32;

    let mut number_end: usize = 0;

    for i in 0..try_number {
        for y in 0..= weight_bias.len() - 1 {

            if true_counter == weight_bias_trouve.len() {
                break;
            }

            if weight_bias_trouve[y] == false {

                sum_derivative_square_residual = 0.0;

                for j in 0..observed_values[0].len() {

                    predicted_value = (weight[z][j] * observed_values[0][j]) + bias[z][j];

                    if y == 0 {
                        derivative_square_residual = (-power_dif * inputs[j]) * (observed_values[0][j] - predicted_value);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }

                    if y == 1 {
                        derivative_square_residual = -power_dif * (observed_values[j] - predicted_value);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }
                }

                step_size = sum_derivative_square_residual * weight_bias_learning_rate[y];
                weight_bias[y] = weight_bias[y] - step_size;

                if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                    weight_bias_trouve[y] = true;
                    true_counter = true_counter + 1;

                    if y == 0 {
                        println!("\n\nfini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                        println!("Le coéficient directeur : {}", weight_bias[y]);
                    }

                    if y == 1 {
                        println!("\n\nfini de trouver le bon intercept de la droite de prediction  ! ");
                        println!("L'intercept : {}", weight_bias[y]);
                    }
                }
            }
        }

        if true_counter == weight_bias_trouve.len() {
            number_end = i;
            break;
        }
    }
    
    if true_counter == weight_bias_trouve.len() {
        println!("\nl'équation de la droite de prédiction est : y = a{} + {}", weight_bias[0], weight_bias[1]);
        println!("L'algorithme a fait {} essaies pour trouver les bonnes données.", number_end + 1);
    }

    weights

    return (weight_bias, weight_bias);
}*/
