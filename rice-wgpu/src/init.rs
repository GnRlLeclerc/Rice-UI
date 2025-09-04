//! Initialize a wgpu device and queue.

use wgpu::*;
use winit::window::Window;

/// Initialize WGPU for the given window
pub async fn init_wgpu<'a>(
    window: &'a Window,
) -> (
    Device,
    Queue,
    Surface<'a>,
    TextureFormat,
    SurfaceConfiguration,
) {
    // Adjust size
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());

    // Request an adapter which can render to our surface
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let mut limits = Limits::downlevel_webgl2_defaults();
    limits.max_storage_buffers_per_shader_stage = 1;
    limits.max_storage_buffer_binding_size = 16 * 1024 * 1024; // 16 MB

    // Create device & queue
    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: limits,
            memory_hints: MemoryHints::MemoryUsage,
            trace: Trace::Off,
        })
        .await
        .expect("Failed to create device");

    let config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    (device, queue, surface, swapchain_format, config)
}
