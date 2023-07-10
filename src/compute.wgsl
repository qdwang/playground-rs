@group(0)
@binding(0)
var<storage, read_write> input_output: array<u32>; 

@group(0)
@binding(1)
var<storage, read_write> shared_data: array<u32>; 

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    input_output[id.x] = input_output[id.x] + shared_data[0];
}