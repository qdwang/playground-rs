@group(0) @binding(0)
var<storage> input: array<u32>; 

@group(0) @binding(1)
var<storage, read_write> output: array<f32>; 

fn get_pixels(offset: u32) -> vec4<f32> {
    let v0 = unpack2x16unorm(input[offset - 1u]).y;
    let v12 = unpack2x16unorm(input[offset]);
    let v1 = v12.x;
    let v2 = v12.y;
    let v3 = unpack2x16unorm(input[offset + 1u]).x;

    return vec4(v0, v1, v2, v3);
}

@compute @workgroup_size(252)
fn main(@builtin(local_invocation_index) index: u32, @builtin(workgroup_id) group_id: vec3<u32>) {
    let width = 3024u;
    let row = group_id.x / (width / 252u);

    let offset = group_id.x * 252u + index;

    let r1 = get_pixels(offset - width);
    let r2 = get_pixels(offset);
    let r3 = get_pixels(offset + width);
    
    let start = offset * 6u;
    
    if row % 2u == 0u {
        if index % 2u == 0u {
            // R
            output[start] = r2.y;
            output[start + 1u] = (r1.y + r2.x + r2.z + r3.y) / 4f;
            output[start + 2u] = (r1.x + r1.z + r3.x + r3.z) / 4f;

            output[start + 3u] = (r2.y + r2.w) / 2f;
            output[start + 4u] = r2.z;
            output[start + 5u] = (r1.z + r3.z) / 2f;
        } else {
            // G1
            output[start] = (r2.x + r2.z) / 2f;
            output[start + 1u] = r2.y;
            output[start + 2u] = (r1.y + r3.y) / 2f;

            output[start + 3u] = r2.z;
            output[start + 4u] = (r1.z + r2.y + r2.w + r3.z) / 4f;
            output[start + 5u] = (r1.y + r1.w + r3.y + r3.w) / 4f;
        }
    } else {
        if index % 2u == 0u {
            // G2
            output[start] = (r1.y + r1.y) / 2f;
            output[start + 1u] = r2.y;
            output[start + 2u] = (r2.x + r2.z) / 2f;

            output[start + 3u] = (r1.y + r1.w + r3.y + r3.w) / 4f;
            output[start + 4u] = (r1.z + r2.y + r2.w + r3.z) / 4f;
            output[start + 5u] = r2.z;
        } else {
            // B
            output[start] = (r1.x + r1.z + r3.x + r3.z) / 4f;
            output[start + 1u] = (r1.y + r2.x + r2.z + r3.y) / 4f;
            output[start + 2u] = r2.y;

            output[start + 3u] = (r1.z + r3.z) / 2f;
            output[start + 4u] = r2.z;
            output[start + 5u] = (r2.y + r2.w) / 2f;
        }
    }
}