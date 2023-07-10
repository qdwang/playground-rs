fn main() {
    pollster::block_on(init());
}

async fn init() {
    env_logger::init();

    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();

    println!("{:?}", adapter);
}