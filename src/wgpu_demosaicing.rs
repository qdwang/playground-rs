use std::time::Instant;

use image::{ImageBuffer, ImageFormat, Rgb};
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, ComputePipeline};

fn main() {
    let t = Instant::now();
    let data = std::fs::read("test_data").unwrap();
    pollster::block_on(init(bytemuck::cast_slice(&data)));
    println!("{:?}ms", (Instant::now() - t).as_millis());
}

fn gen_pipeline_and_bindgroup(
    device: &Device,
    shader_str: &str,
    buffers: &[(&Buffer, bool)],
) -> (ComputePipeline, BindGroup) {
    let mut bind_group_layout_entries = vec![];
    let mut bind_group_entries = vec![];

    for (index, (buffer, read_only)) in buffers.into_iter().enumerate() {
        bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding: index as u32,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: *read_only,
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        bind_group_entries.push(wgpu::BindGroupEntry {
            binding: index as u32,
            resource: buffer.as_entire_binding(),
        });
    }

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &bind_group_layout_entries,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &bind_group_entries,
    });

    let compute_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(shader_str.into()),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
        ),
        module: &compute_module,
        entry_point: "main",
    });

    (pipeline, bind_group)
}

async fn init(data: &[u16]) {
    env_logger::init();

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits {
                    max_buffer_size: 1073741824,
                    max_storage_buffer_binding_size: 1073741824,
                    ..Default::default()
                },
            },
            None,
        )
        .await
        .unwrap();

    // Gets the size in bytes of the buffer.
    let output_buffer_size = (data.len() * 3 * std::mem::size_of::<f32>()) as u64;

    let buffer_input = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(data),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let buffer_output = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: output_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let (compute_pipeline, bind_group) = gen_pipeline_and_bindgroup(
        &device,
        include_str!("compute.wgsl"),
        &[
            (&buffer_input, true),
            (&buffer_output, false),
        ],
    );

    let batch_size = 252u32;
    let batch_count = data.len() as u32 / 2 / batch_size;

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(batch_count, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    let buffer_result = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: output_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    encoder.copy_buffer_to_buffer(&buffer_output, 0, &buffer_result, 0, output_buffer_size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice = buffer_result.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver.receive().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result: Vec<u16> = bytemuck::cast_slice(&data)
            .into_iter()
            .map(|x| {
                let v = (x * u16::MAX as f32) as u16;
                v * 4
            })
            .collect::<Vec<_>>();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        buffer_result.unmap(); // Unmaps buffer from memory
                               // If you are familiar with C++ these 2 lines can be thought of similarly to:
                               //   delete myPointer;
                               //   myPointer = NULL;
                               // It effectively frees the memory

        // Returns data from buffer
        
        let img : ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::from_raw(6048, 4024, result).unwrap();
        img.save_with_format("dump.tif", ImageFormat::Tiff).unwrap();
    } else {
        panic!("failed to run compute on gpu!")
    }
}
