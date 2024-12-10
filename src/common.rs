use std:: {iter, mem, vec };
use cgmath::*;
//use futures::sink::Buffer;
use wgpu::{util::DeviceExt, BindGroup};
use winit::{
    event::*,
    window::Window,
    event_loop::{ControlFlow, EventLoop},
};
use bytemuck:: {Pod, Zeroable, cast_slice};
use std::collections::HashMap;
use image::GenericImageView;
use rust_embed::RustEmbed;

#[path="../src/transforms.rs"]
mod transforms;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

const ANIMATION_SPEED:f32 = 1.0;
const IS_PERSPECTIVE:bool = true;

pub struct GameData {
    pub objects: Vec<Vec<Vertex>>,
    pub positions: Vec<(f32, f32, f32)>,
    pub camera_position: Point3<f32>,
    pub camera_rotation: Point3<f32>,
}
impl GameData {
    pub fn new() -> Self {
        GameData {
            objects: Vec::new(),
            positions: Vec::new(),
            camera_position: (-10.0, 5.0, 0.0).into(),
            camera_rotation: (0.0, 0.0, 0.0).into(),
        }
    }

    pub fn add_object(&mut self, item: Vec<Vertex>, position: (f32, f32, f32)) {
        self.objects.push(item);
        self.positions.push(position);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    color: [f32; 4],
    specular_color : [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

pub fn light(c:[f32; 3], sc:[f32;3], ai: f32, di: f32, si: f32, ss: f32) -> Light {
    Light {
        color:[c[0], c[1], c[2], 1.0],
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ai,
        diffuse_intensity: di,
        specular_intensity: si,
        specular_shininess: ss,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
    pub uv: [f32; 4],
}

#[allow(dead_code)]
pub fn vertex(p:[f32;3], n:[f32; 3], c:[f32; 3], u:[f32; 2]) -> Vertex {
    Vertex {
        position: [p[0], p[1], p[2], 1.0],
        normal: [n[0], n[1], n[2], 1.0],
        color: [c[0], c[1], c[2], 1.0],
        uv: [u[0], u[1], 0.0, 0.0],
    }
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4, 3=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

struct State {
    init: transforms::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffers: Vec<wgpu::Buffer>,
    uniform_bind_groups: Vec<wgpu::BindGroup>,
    vertex_uniform_buffers: Vec<wgpu::Buffer>,
    project_mat: Matrix4<f32>,
    num_vertices: Vec<u32>,
    game_data: GameData
}

impl State {
    fn create_object(
        game_data: &GameData, init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, i: usize) -> (BindGroup, wgpu::Buffer, wgpu::Buffer, u32) {
        // create vertex uniform buffer
        // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
       
        // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light and eye positions
        let light_position:&[f32; 3] = &Point3::new(-10.0, 64.0, -10.0).into();
        let eye_position:&[f32; 3] = &Point3::new(-10.0, 64.0, -10.0).into();
        init.queue.write_buffer(&fragment_uniform_buffer, 0, bytemuck::cast_slice(light_position));
        init.queue.write_buffer(&fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(&light_uniform_buffer, 0, bytemuck::cast_slice(&[light_data]));

        let texture_data = Assets::get("textures/blocks/atlas.png").expect("Failed to load embedded texture");
        let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        let texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = init.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Uniform Bind Group"),
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&game_data.objects[i]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = game_data.objects[i].len() as u32;

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices)
    }

    async fn new(window: &Window, game_data: GameData, light_data: Light) -> Self {        
        let init =  transforms::InitWgpu::init_wgpu(window).await;

        let shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // uniform data
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        
        let (_, project_mat, _) = transforms::create_view_projection(game_data.camera_position, look_direction, up_direction, 
            init.config.width as f32 / init.config.height as f32, IS_PERSPECTIVE);

        let uniform_bind_group_layout = init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Uniform Bind Group Layout"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            //depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });

        let mut vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut num_vertices: Vec<u32> = Vec::new();
        let mut uniform_bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        let mut vertex_uniform_buffers: Vec<wgpu::Buffer> = Vec::new();

        for i in 0..game_data.objects.len() {
            let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object(&game_data, &init, light_data, &uniform_bind_group_layout, i);
            vertex_buffers.push(vertex_buffer);
            num_vertices.push(num_vertices_);
            uniform_bind_groups.push(uniform_bind_group);
            vertex_uniform_buffers.push(vertex_uniform_buffer);
        }

        Self {
            init,
            pipeline,
            vertex_buffers,
            uniform_bind_groups,
            vertex_uniform_buffers,
            project_mat,
            num_vertices,
            game_data
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.instance.poll_all(true);
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init.surface.configure(&self.init.device, &self.init.config);
            self.project_mat = transforms::create_projection(new_size.width as f32 / new_size.height as f32, IS_PERSPECTIVE);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self, dt: std::time::Duration, keys_down: &HashMap<&str, bool>, mouse_movement: &Vec<f64>) {
        let forward = Vector3::new(
            self.game_data.camera_rotation[1].cos() * self.game_data.camera_rotation[0].cos(),
            self.game_data.camera_rotation[0].sin(),
            self.game_data.camera_rotation[1].sin() * self.game_data.camera_rotation[0].cos(),
        ).normalize();
        let right = Vector3::new(
            self.game_data.camera_rotation[1].sin(),
            0.0,
            -self.game_data.camera_rotation[1].cos(),
        ).normalize();

        if let Some(is_pressed) = keys_down.get("w") {
            if is_pressed == &true {
                self.game_data.camera_position[0] += 0.1 * forward[0];
                self.game_data.camera_position[1] += 0.1 * forward[1];
                self.game_data.camera_position[2] += 0.1 * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("s") {
            if is_pressed == &true {
                self.game_data.camera_position[0] -= 0.1 * forward[0];
                self.game_data.camera_position[1] -= 0.1 * forward[1];
                self.game_data.camera_position[2] -= 0.1 * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("a") {
            if is_pressed == &true {
                self.game_data.camera_position[0] += 0.1 * right[0];
                self.game_data.camera_position[1] += 0.1 * right[1];
                self.game_data.camera_position[2] += 0.1 * right[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("d") {
            if is_pressed == &true {
                self.game_data.camera_position[0] -= 0.1 * right[0];
                self.game_data.camera_position[1] -= 0.1 * right[1];
                self.game_data.camera_position[2] -= 0.1 * right[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("right") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] += 0.02;
            }
        }
        if let Some(is_pressed) = keys_down.get("left") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] -= 0.02;
            }
        }

        self.game_data.camera_rotation[1] -= mouse_movement[0] as f32 * 0.001;
        self.game_data.camera_rotation[0] += mouse_movement[1] as f32 * 0.001;
        self.game_data.camera_rotation[0] = self.game_data.camera_rotation[0].clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

        let up_direction = cgmath::Vector3::unit_y();
        let (view_mat, project_mat, _) = transforms::create_view_rotation(
            self.game_data.camera_position, self.game_data.camera_rotation[1], self.game_data.camera_rotation[0], 
            up_direction, self.init.config.width as f32 / self.init.config.height as f32, IS_PERSPECTIVE);

        // update uniform buffer
        let _dt = ANIMATION_SPEED * dt.as_secs_f32(); 
        let view_project_mat = project_mat * view_mat;
        let view_projection_ref:&[f32; 16] = view_project_mat.as_ref();

        for i in 0..self.game_data.objects.len() {
            let position = self.game_data.positions[i];
            let model_mat = transforms::create_transforms([position.0, position.1, position.2], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
            let normal_mat = (model_mat.invert().unwrap()).transpose();
            let model_ref:&[f32; 16] = model_mat.as_ref();
            let normal_ref:&[f32; 16] = normal_mat.as_ref();

            self.init.queue.write_buffer(&self.vertex_uniform_buffers[i], 0, bytemuck::cast_slice(model_ref));
            self.init.queue.write_buffer(&self.vertex_uniform_buffers[i], 64, bytemuck::cast_slice(view_projection_ref));
            self.init.queue.write_buffer(&self.vertex_uniform_buffers[i], 128, bytemuck::cast_slice(normal_ref));
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //let output = self.init.surface.get_current_frame()?.output;
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .init.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                //depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pipeline);
            
            for i in 0..self.game_data.objects.len() {
                render_pass.set_vertex_buffer(0, self.vertex_buffers[i].slice(..));           
                render_pass.set_bind_group(0, &self.uniform_bind_groups[i], &[]);
                render_pass.draw(0..self.num_vertices[i], 0..1);
            }
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn run(game_data: GameData, light_data: Light, title: &str) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(title);

    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
        eprintln!("Failed to lock the cursor: {:?}", err);
    }
    window.set_cursor_visible(false);

    let mut state = pollster::block_on(State::new(&window, game_data, light_data));    
    let render_start_time = std::time::Instant::now();

    let mut keys_down: HashMap<&str, bool> = HashMap::new();
    keys_down.insert("w", false);
    keys_down.insert("a", false);
    keys_down.insert("s", false);
    keys_down.insert("d", false);
    keys_down.insert("space", false);
    let mut mouse_movement: Vec<f64> = vec![0.0, 0.0];
    let mut mouse_locked = true;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                    state: key_state,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        } => {
                            match key_state {
                                ElementState::Pressed => {
                                    match &keycode {
                                        &VirtualKeyCode::W => { keys_down.insert("w", true); }
                                        &VirtualKeyCode::A => { keys_down.insert("a", true); }
                                        &VirtualKeyCode::S => { keys_down.insert("s", true); }
                                        &VirtualKeyCode::D => { keys_down.insert("d", true); }
                                        &VirtualKeyCode::Space => { keys_down.insert("space", true); }
                                        &VirtualKeyCode::Right => { keys_down.insert("right", true); }
                                        &VirtualKeyCode::Left => { keys_down.insert("left", true); }
                                        &VirtualKeyCode::Escape => {
                                            mouse_locked = false;
                                            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                                eprintln!("Failed to unlock the cursor: {:?}", err);
                                            }
                                            window.set_cursor_visible(true);
                                        }
                                        _ => {}
                                    }
                                }
                                ElementState::Released => {
                                    match &keycode {
                                        &VirtualKeyCode::W => { keys_down.insert("w", false); }
                                        &VirtualKeyCode::A => { keys_down.insert("a", false); }
                                        &VirtualKeyCode::S => { keys_down.insert("s", false); }
                                        &VirtualKeyCode::D => { keys_down.insert("d", false); }
                                        &VirtualKeyCode::Space => { keys_down.insert("space", false); }
                                        &VirtualKeyCode::Right => { keys_down.insert("right", false); }
                                        &VirtualKeyCode::Left => { keys_down.insert("left", false); }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            if mouse_locked {
                                let window_size = window.inner_size();
                                let center_x = window_size.width as f64 / 2.0;
                                let center_y = window_size.height as f64 / 2.0;
                                mouse_movement[0] = center_x - position.x;
                                mouse_movement[1] = center_y - position.y;
                                window.set_cursor_position(winit::dpi::PhysicalPosition::new(center_x, center_y)).expect("Failed to set cursor position");
                            } else {
                                mouse_movement[0] = 0.0;
                                mouse_movement[1] = 0.0;
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    match button {
                                        MouseButton::Left => {
                                            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                                                eprintln!("Failed to lock the cursor: {:?}", err);
                                            }
                                            window.set_cursor_visible(false);
                                            mouse_locked = true;
                                        }
                                        MouseButton::Right => {
                                            return
                                        }
                                        MouseButton::Middle => {
                                            return
                                        }
                                        _ => {}
                                    }
                                }
                                ElementState::Released => {
                                    return
                                }
                            }
                        }
                        WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - render_start_time;
                state.update(dt, &keys_down, &mouse_movement);

                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}