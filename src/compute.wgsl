@group(0) @binding(0)
var<storage> input: array<u32>; 

@group(0) @binding(1)
var<storage, read_write> output: array<f32>; 

@group(0) @binding(2)
var<storage> shared_data: array<f32>; 

// p1, p2
@compute @workgroup_size(2)
fn main(@builtin(local_invocation_index) index: u32, @builtin(workgroup_id) group_id: vec3<u32>) {
    var v0 = unpack2x16unorm(input[index - 1u]).y;
    var u32_value = unpack2x16unorm(input[index]);
    var v1 = u32_value.x;
    var v2 = u32_value.y;
    var v3 = unpack2x16unorm(input[index + 1u]).x;

    output[index * 2u] = (v0 + v1 + v2) / select(3.0, 2.0, v0 == 0.0);
    output[index * 2u + 1u] = (v1 + v2 + v3) / select(3.0, 2.0, v3 == 0.0);
}