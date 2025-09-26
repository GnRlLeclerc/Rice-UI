//! Drawing pipeline manager

use std::borrow::Cow;

use rice_dom::{ComputedStyle, DOM};
use rice_layout::Rect;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};
use winit::window::Window;

use crate::write_buffer::{WriteBuffer, write_indexed_slice_to_buffer};

/// Vertex indices to draw a rectangle from 2 triangles
const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

/// Base vertices to draw a rectangle from 2 triangles
const VERTICES: &[f32] = &[
    -1.0, -1.0, // bottom left
    1.0, -1.0, // bottom right
    1.0, 1.0, // top right
    -1.0, 1.0, // top left
];

/// Rendering pipeline manager
pub struct Pipeline {
    pub pipeline: RenderPipeline,

    /// Amount of elements to draw
    pub n: u32,

    /// Screen size & conversion physical <-> logical
    pub screen_buffer: Buffer,
    /// Base rectangle vertex buffer
    pub vertex_buffer: Buffer,
    /// Instance buffer (positions & sizes)
    pub rects_buffer: Buffer,
    /// Instance buffer (styles)
    pub styles_buffer: Buffer,
    /// Base rectangle index buffer
    pub index_buffer: Buffer,

    /// Bind group for the timer & screen
    pub uniforms_group: BindGroup,

    /// Max amount of rects that can fit in the vertex & rects buffers
    pub size: usize,
}

impl Pipeline {
    /// Create a new bind group manager with the given max amount of rects
    pub fn new(
        device: &Device,
        window: &Window,
        swapchain_format: TextureFormat,
        size: usize,
    ) -> Self {
        // ***************************************** //
        //                  BUFFERS                  //
        // ***************************************** //
        let screen = window.inner_size();
        let screen_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Screen Buffer"),
            contents: bytemuck::cast_slice(&[
                screen.width as f32,
                screen.height as f32,
                window.scale_factor() as f32,
                0.0f32, // padding
            ]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: BufferUsages::INDEX,
        });
        let rects_buffer = Self::create_rects_buffer(device, size);
        let styles_buffer = Self::create_styles_buffer(device, size);

        // ***************************************** //
        //             BIND GROUP LAYOUTS            //
        // ***************************************** //
        let uniforms_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Uniforms Bind Group Layout"),
        });
        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        };
        let rects_layout = VertexBufferLayout {
            array_stride: Rect::SIZE as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // Size
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 1,
                },
                // Position
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 2,
                },
            ],
        };
        let styles_layout = VertexBufferLayout {
            array_stride: ComputedStyle::SIZE as BufferAddress,
            step_mode: VertexStepMode::Instance,
            // RGBA color
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 3,
            }],
        };

        // ***************************************** //
        //                 BIND GROUPS               //
        // ***************************************** //
        let uniforms_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniforms_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
            label: Some("Uniforms Bind Group"),
        });

        // ***************************************** //
        //                  SHADERS                  //
        // ***************************************** //
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        // ***************************************** //
        //                   PIPELINE                //
        // ***************************************** //
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniforms_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Main Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[vertex_layout, rects_layout, styles_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: swapchain_format,
                    // enable alpha blending
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::OVER,
                        alpha: wgpu::BlendComponent::OVER,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            n: 0,

            screen_buffer,
            vertex_buffer,
            rects_buffer,
            styles_buffer,
            index_buffer,

            uniforms_group,

            size,
        }
    }

    /// Render the pipeline
    pub fn render(&self, render_pass: &mut RenderPass) {
        // Pipeline
        render_pass.set_pipeline(&self.pipeline);

        // Uniforms
        render_pass.set_bind_group(0, &self.uniforms_group, &[]);

        // Index buffer
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

        // Vertex buffers
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.rects_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.styles_buffer.slice(..));

        // Draw call
        render_pass.draw_indexed(0..6, 0, 0..self.n);
    }

    /// Update screen size & scale
    pub fn update_screen(&self, queue: &Queue, window: &Window) {
        queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::cast_slice(&[
                window.inner_size().width as f32,
                window.inner_size().height as f32,
                window.scale_factor() as f32,
                0.0f32, // padding
            ]),
        );
    }

    /// Update rect data in internal buffers
    pub fn update_elements(&mut self, device: &Device, queue: &Queue, dom: &DOM) {
        assert!(dom.rects.len() == dom.styles.len());
        assert!(dom.rects.len() > 0);

        let n = dom.redraw.len();

        // Reallocate buffers if needed
        if n > self.size {
            self.rects_buffer = Self::create_rects_buffer(device, n * 2);
            self.styles_buffer = Self::create_styles_buffer(device, n * 2);
            self.size = dom.rects.len() * 2;
        }
        self.n = n as u32;

        // Write rects to buffer
        write_indexed_slice_to_buffer(&dom.rects, &dom.redraw, &self.rects_buffer, queue);

        // Write styles to buffer
        write_indexed_slice_to_buffer(&dom.styles, &dom.redraw, &self.styles_buffer, queue);
    }

    // ************************************************* //
    //                  ALLOCATION UTILS                 //
    // ************************************************* //

    fn create_rects_buffer(device: &Device, size: usize) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Rect Data Buffer"),
            size: (size * Rect::SIZE) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_styles_buffer(device: &Device, size: usize) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Styles Buffer"),
            size: (size * ComputedStyle::SIZE) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
