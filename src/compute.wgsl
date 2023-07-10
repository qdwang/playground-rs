@group(0) @binding(0)
var<storage> input: array<u32>; 

@group(0) @binding(1)
var<storage, read_write> output: array<f32>; 

@group(0) @binding(2)
var<storage> shared_data: array<f32>; 

@compute @workgroup_size(2)
fn main(@builtin(local_invocation_index) index: u32, @builtin(workgroup_id) group_id: vec3<u32>) {
    var u32_value = input[index];
    var f32_values = unpack2x16unorm(u32_value);
    output[index * 2u] = f32_values.x * 2.0 + shared_data[0];
    output[index * 2u + 1u] = f32_values.y * 2.0 + shared_data[0];
}