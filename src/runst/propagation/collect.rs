// linking the functions(FunType) to their name(&str):

use crate::runst::propagation::activ_fun::*;
use crate::runst::FunType;

pub fn activ_fun() -> Vec<(FunType, &'static str)> {

    let mut activ_fun_list: Vec<(FunType, &str)> = Vec::new();

    activ_fun_list.push((Box::new(none), "none"));
    activ_fun_list.push((Box::new(relu), "relu"));
    activ_fun_list.push((Box::new(leaky_relu), "leaky_relu"));
    activ_fun_list.push((Box::new(silu), "silu"));
    activ_fun_list.push((Box::new(softplus), "softplus"));
    activ_fun_list.push((Box::new(sigmoid), "sigmoid"));
    activ_fun_list.push((Box::new(softmax), "softmax"));

    activ_fun_list
}