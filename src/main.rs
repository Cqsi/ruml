fn main() {
    multiple_neurons();
}

fn single_neuron() {
    let inputs: [f32; 3] = [1.0,2.0,3.0];
    let weights: [f32; 3] = [0.2, 0.8, -0.5];
    let bias: f32 = 2.0;

    let result = inputs[0]*weights[0]+inputs[1]*weights[1]+inputs[2]*weights[2]+bias;

    println!("{result}");
}

fn multiple_neurons() {
    let inputs: [f32; 4] = [1.0,2.0,3.0,2.5];
    let weights: [[f32; 4]; 3] = [[0.2, 0.8, -0.5, 1.0],[0.5, -0.91, 0.26, -0.5],[-0.26, -0.27, 0.17, 0.87]];
    let biases: [f32; 3] = [2.0,3.0,0.5];

    let mut layer_outputs = vec![];
    for i in 0..weights.len(){
        //println!("{}", layer_outputs[i]);

        let mut neuron_output: f32 = 0.0;

        for j in 0..weights[i].len(){
            neuron_output += inputs[j]*weights[i][j];
        }

        neuron_output += biases[i];
        layer_outputs.push(neuron_output);
    }

    for output in layer_outputs {
        println!("{}", output)
    }
}