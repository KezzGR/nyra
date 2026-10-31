struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>
}
struct TransformUniform {
    model: mat4x4<f32>,
    projection_view: mat4x4<f32>
}
struct MaterialUniform {
    base_color: vec4<f32>,
    uv_scale: vec2<f32>,
    uv_offset: vec2<f32>
}
@group(0) @binding(0) var<uniform> transform: TransformUniform;
@vertex fn vs_main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = transform.projection_view * transform.model * vec4<f32>(position, 1.0);
    output.normal = (transform.model * vec4<f32>(normal, 0.0)).xyz;
    output.uv = uv;
    return output;
}
@group(1) @binding(0) var<uniform> material: MaterialUniform;
@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var color_sampler: sampler;
@fragment fn fs_main(@location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let normal_dir = normalize(normal);
    let light_dir = normalize(vec3<f32>(1.0, 4.0, 0.0));
    let lambert = max(0.0, dot(normal_dir, light_dir));
    let brightness = 0.1 + 0.9 * lambert;
    let transformed_uv = uv * material.uv_scale + material.uv_offset;
    let albedo = textureSample(texture, color_sampler, transformed_uv) * material.base_color;
    return vec4<f32>(albedo.rgb * brightness, albedo.a);
}
