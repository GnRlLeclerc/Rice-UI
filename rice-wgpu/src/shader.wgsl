// Screen physical size
struct Screen {
    // Physical size
    size: vec2<f32>,
    // Logical to physical scaling factor
    scale: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) size: vec2<f32>,
    @location(1) offset: vec2<f32>,
    @location(2) color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> screen: Screen;

@vertex
fn vertex_shader(
    @location(0) vertex: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) offset: vec2<f32>,
    @location(3) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    // Pass physical size & position to fragment shader
    out.size = size * screen.scale;
    out.offset = offset * screen.scale;
    out.color = color;

    var position: vec2<f32> = (vertex * size + offset) * screen.scale / screen.size * 2.0 - 1.0;
    position.y *= -1.0; // Invert Y axis

    out.position = vec4<f32>(
        position,
        0.0,
        1.0
    );

    return out;
}


@fragment
fn fragment_shader(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return in.color;
}
