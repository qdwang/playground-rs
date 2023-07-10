// env_logger = "0.10.0"
// log = "0.4.19"
// pollster = "0.3.0"
// wgpu = "0.16.1"
// winit = "0.28.6"

use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder,
};

fn main() {
    pollster::block_on(init());
}

async fn init() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: true,
        },
    ).await.unwrap();

    println!("{:?}", adapter);
}