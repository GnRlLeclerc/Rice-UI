mod init;
mod pipeline;

use rice_dom::DOM;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use crate::{init::init_wgpu, pipeline::Pipeline};

pub async fn run(event_loop: EventLoop<()>, window: Window, dom: DOM) {
    let window = &window;
    let (device, queue, surface, format, mut config) = init_wgpu(window).await;

    let mut pipeline = Pipeline::new(&device, window, format, 100);
    pipeline.update_elements(&device, &queue, &dom.rects, &dom.styles);

    event_loop
        .run(move |event, target| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        pipeline.update_screen(&queue, window);
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let size = window.inner_size();
                        // DEBUG
                        println!(
                            "Redrawing with size: {}x{} - scale {}",
                            size.width,
                            size.height,
                            window.scale_factor()
                        );
                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        {
                            let mut rpass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        depth_slice: None,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            pipeline.bind(&mut rpass);
                            // TODO: count how many rects to draw
                            rpass.draw_indexed(0..6, 0, 0..1);
                        }

                        queue.submit(Some(encoder.finish()));
                        window.pre_present_notify();
                        frame.present();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();
}
