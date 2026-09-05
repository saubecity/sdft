use std::default;

use wgpu::{
    BindGroupDescriptor, BindGroupLayout, BindGroupLayoutEntry, Device, PipelineCache,
    PipelineCompilationOptions, PipelineLayout, RenderPipeline, ShaderModule,
    ShaderModuleDescriptor, ShaderStages, SurfaceConfiguration,
};

use crate::ui::{Color, Position, Rect};

pub struct Rectangle {
    rect: Rect,
    color: Color,
}

pub struct Renderer {
    pipeline: RenderPipeline,
    //bind_group_layout: wgpu::BindGroupLayout,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
struct RectangleVB {
    position: [f32; 4], /* x,y,h,w */
    color: [f32; 4],    /* r,g,b,a */
}

impl Renderer {
    fn create_shader_module(device: &wgpu::Device) -> ShaderModule {
        device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Rectangle shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rectangle.wgsl").into()),
        })
    }

    fn create_pipeline(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        cache: PipelineCache,
    ) -> RenderPipeline {
        let shader = Self::create_shader_module(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Rectangle pipeline layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rectangle render pipeline"),
            vertex: wgpu::VertexState {
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
                module: &shader,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                cull_mode: Some(wgpu::Face::Back),
                front_face: wgpu::FrontFace::Ccw,
                strip_index_format: None,
                unclipped_depth: false,
                conservative: false,
            },
            layout: Some(&render_pipeline_layout),
            cache: Some(&cache),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
        })
    }

    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        pipeline_cache: PipelineCache,
    ) -> Self {
        let pipeline = Self::create_pipeline(device, surface_config, pipeline_cache);

        Self { pipeline }
    }
}
