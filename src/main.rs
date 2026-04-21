fn main() {
    //println!("Hello, world!");
    let inputs: [f32; 3] = [1.0,2.0,3.0];
    let weights: [f32; 3] = [0.2, 0.8, -0.5];
    let bias: f32 = 2.0;
}

fn single_neuron() {
    let inputs: [f32; 3] = [1.0,2.0,3.0];
    let weights: [f32; 3] = [0.2, 0.8, -0.5];
    let bias: f32 = 2.0;

    let result = inputs[0]*weights[0]+inputs[1]*weights[1]+inputs[2]*weights[2]+bias;

    println!("{result}");
}

fn multiple_neurons()