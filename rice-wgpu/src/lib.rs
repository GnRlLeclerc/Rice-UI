mod init;
mod pipeline;
mod write_buffer;

use rice_dom::DOM;
use winit::{
    dpi::LogicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use crate::{init::init_wgpu, pipeline::Pipeline};

pub async fn run(event_loop: EventLoop<()>, window: Window, mut dom: DOM) {
    let window = &window;
    let (device, queue, surface, format, mut config) = init_wgpu(window).await;

    let mut pipeline = Pipeline::new(&device, window, format, 100);
    dom.dirty.push(dom.root); // Mark the root node as dirty

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
                        dom.dirty.push(dom.root); // Mark the root node as dirty
                    }
                    WindowEvent::RedrawRequested => {
                        // TODO : decide of redrawing, depending on dirty elements, screen changed, etc
                        if dom.dirty.is_empty() {
                            return;
                        }
                        dom.compute_redraw();
                        pipeline.update_elements(&device, &queue, &dom);
                        let n = dom.redraw.len();
                        dom.dirty.clear();
                        dom.redraw.clear();

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
                                            load: wgpu::LoadOp::Load,
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            pipeline.render(&mut rpass);
                        }

                        queue.submit(Some(encoder.finish()));
                        window.pre_present_notify();
                        frame.present();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        let position: LogicalPosition<i32> =
                            position.to_logical(window.scale_factor());
                        let mouse = [position.x, position.y];

                        dom.handle_mouse_moved(mouse);
                        window.request_redraw();
                    }
                    WindowEvent::CursorLeft { .. } => {
                        dom.reset_mouse();
                        if dom.dirty.len() > 0 {
                            window.request_redraw();
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => match state {
                                ElementState::Pressed => {
                                    dom.handle_mouse_clicked(true);
                                }
                                ElementState::Released => {
                                    dom.handle_mouse_clicked(false);
                                }
                            },
                            _ => {}
                        };
                        if dom.dirty.len() > 0 {
                            window.request_redraw();
                        }
                    }
                    _ => {}
                };
            }
        })
        .unwrap();
}
