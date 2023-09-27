// pub struct Sprites {
//     pipeline: wgpu::RenderPipelinne,
//     groups: Vec<(wgpu::Testure, wgpu::Buffer, Vec<GPUSprite>)>
// }
// impl Sprites {
//     pub fn set_sprite_position(&mut self, group_id:usize, sprite_num: usize, pos:Vec2) {
//         self.groups[group_id].2[sprite_num].screen_rec.pos = pos;
//     }
//     pub fn get_sprite(&mut self, group_id:usize, sprite_num: usize, pos:Vec2) -> &mut GPUSprite{
//         self.groups[group_id].2[sprite_num];
//     }
    
//     pub fn new(gpu:&GPUState) -> Self {
//     // Load the shaders from disk
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: None,
//         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
//     });

//     let texture_bind_group_layout =
//         device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: None,
//             // It needs the first entry for the texture and the second for the sampler.
//             // This is like defining a type signature.
//             entries: &[
//                 // The texture binding
//                 wgpu::BindGroupLayoutEntry {
//                     // This matches the binding in the shader
//                     binding: 0,
//                     // Only available in the fragment shader
//                     visibility: wgpu::ShaderStages::FRAGMENT,
//                     // It's a texture binding
//                     ty: wgpu::BindingType::Texture {
//                         // We can use it with float samplers
//                         sample_type: wgpu::TextureSampleType::Float { filterable: true },
//                         // It's being used as a 2D texture
//                         view_dimension: wgpu::TextureViewDimension::D2,
//                         // This is not a multisampled texture
//                         multisampled: false,
//                     },
//                     count: None,
//                 },
//                 // The sampler binding
//                 wgpu::BindGroupLayoutEntry {
//                     // This matches the binding in the shader
//                     binding: 1,
//                     // Only available in the fragment shader
//                     visibility: wgpu::ShaderStages::FRAGMENT,
//                     // It's a sampler
//                     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
//                     // No count
//                     count: None,
//                 },
//             ],
//         });
//            // The camera binding
//     let camera_layout_entry = wgpu::BindGroupLayoutEntry {
//         // This matches the binding in the shader
//         binding: 0,
//         // Available in vertex shader
//         visibility: wgpu::ShaderStages::VERTEX,
//         // It's a buffer
//         ty: wgpu::BindingType::Buffer {
//             ty: wgpu::BufferBindingType::Uniform,
//             has_dynamic_offset: false,
//             min_binding_size: None,
//         },
//         // No count, not a buffer array binding
//         count: None,
//     };
//     let sprite_bind_group_layout = match SPRITES {
//         SpriteOption::Storage => {
//             device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                 label: None,
//                 entries: &[
//                     camera_layout_entry,
//                     wgpu::BindGroupLayoutEntry {
//                         // This matches the binding in the shader
//                         binding: 1,
//                         // Available in vertex shader
//                         visibility: wgpu::ShaderStages::VERTEX,
//                         // It's a buffer
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Storage { read_only: true },
//                             has_dynamic_offset: false,
//                             min_binding_size: None,
//                         },
//                         // No count, not a buffer array binding
//                         count: None,
//                     },
//                 ],
//             })
//         }
//         SpriteOption::Uniform => {
//             device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                 label: None,
//                 entries: &[
//                     camera_layout_entry,
//                     wgpu::BindGroupLayoutEntry {
//                         // This matches the binding in the shader
//                         binding: 1,
//                         // Available in vertex shader
//                         visibility: wgpu::ShaderStages::VERTEX,
//                         // It's a buffer
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Uniform,
//                             has_dynamic_offset: false,
//                             min_binding_size: wgpu::BufferSize::new(SPRITE_UNIFORM_SIZE),
//                         },
//                         // No count, not a buffer array binding
//                         count: None,
//                     },
//                 ],
//             })
//         }
//         SpriteOption::VertexBuffer => {
//             device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                 label: None,
//                 entries: &[camera_layout_entry],
//             })
//         }
//     };
//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: None,
//         bind_group_layouts: &[&sprite_bind_group_layout, &texture_bind_group_layout],
//         push_constant_ranges: &[],
//     });

//     let swapchain_capabilities = surface.get_capabilities(&adapter);
//     let swapchain_format = swapchain_capabilities.formats[0];

//     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: None,
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &shader,
//             entry_point: match SPRITES {
//                 SpriteOption::Storage => "vs_storage_main",
//                 SpriteOption::Uniform => "vs_uniform_main",
//                 SpriteOption::VertexBuffer => "vs_vbuf_main",
//             },
//             buffers: match SPRITES {
//                 SpriteOption::VertexBuffer => &[wgpu::VertexBufferLayout {
//                     array_stride: std::mem::size_of::<GPUSprite>() as u64,
//                     step_mode: wgpu::VertexStepMode::Instance,
//                     attributes: &[
//                         wgpu::VertexAttribute {
//                             format: wgpu::VertexFormat::Float32x4,
//                             offset: 0,
//                             shader_location: 0,
//                         },
//                         wgpu::VertexAttribute {
//                             format: wgpu::VertexFormat::Float32x4,
//                             offset: std::mem::size_of::<[f32; 4]>() as u64,
//                             shader_location: 1,
//                         },
//                     ],
//                 }],
//                 _ => &[],
//             },
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &shader,
//             entry_point: "fs_main",
//             targets: &[Some(swapchain_format.into())],
//         }),
//         primitive: wgpu::PrimitiveState::default(),
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState::default(),
//         multiview: None,
//     });
//     }
//     pub fn render(gpu:&GPUState, camera:GPUCamera) {
//         //..
//     }
// }