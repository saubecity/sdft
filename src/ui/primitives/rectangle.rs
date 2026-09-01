use wgpu::{BindGroupDescriptor, BindGroupLayout, BindGroupLayoutEntry, PipelineLayout, RenderPipeline, ShaderModule, ShaderModuleDescriptor, ShaderStages};

use crate::ui::{Color, Position, Rect};

pub struct Rectangle {
    rect: Rect,
    color: Color,
}

pub struct Renderer {
    pipeline: RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct RectangleVB {
    position: [f32; 4] /* x,y,h,w */,
    color: [f32; 4] /* r,g,b,a */,
}

impl Renderer {
    fn init_shader_module(device: &wgpu::Device) -> ShaderModule {
        device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Rectangle shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rectangle.wgsl").into()),
        })
    }

    fn

    pub fn new(device: &wgpu::Device) -> Self {
        let shaders = Self::init_shader_module(device);


        let bind_group_layouts = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("rectangle bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding:0,
                    visibility: Some(ShaderStages::VERTEX),
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: 0},
                    count: None,
                }]}
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("rectangle pipeline layout"),
            bind_group_layouts: (), immediate_size: 0 });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("rectangle primitive render"),
                layout: (), vertex: (), primitive: (), depth_stencil: (), multisample: (), fragment: (), multiview_mask: (), cache: () })
    }


}
