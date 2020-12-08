use anyhow::{Context, Result};
use futures::executor::block_on;
use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Display {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

impl Display {
    pub async fn new(window: &Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&Default::default()).await
            .context("Unable to find valid adapter!")?;
        let (device, queue) = adapter.request_device(&Default::default(), None).await?;
        let sc_desc = wgpu::SwapChainDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            present_mode: wgpu::PresentMode::Fifo,
            width: window.inner_size().width,
            height: window.inner_size().height,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        Ok(Self {
            device,
            queue,
            surface,
            sc_desc,
            swap_chain,
        })
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

pub trait Demo: Sized {
    fn new(display: &Display) -> Result<Self>;
    fn resize(&mut self, display: &Display);
    fn device_event(&mut self, event: &DeviceEvent) -> bool;
    fn render(
        &mut self, 
        display: &Display,
        frame: &wgpu::SwapChainFrame, 
        encoder: &mut wgpu::CommandEncoder,
        dt: std::time::Duration,
    );
}

pub fn run<D: Demo + 'static>(title: &str) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_visible(false)
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)?;
    let mut display = block_on(Display::new(&window))?;
    let mut demo = D::new(&display)?;
    let mut last_render_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                demo.device_event(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        display.resize(*physical_size);
                        demo.resize(&display);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        display.resize(**new_inner_size);
                        demo.resize(&display);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                match display.swap_chain.get_current_frame() {
                    Ok(frame) => {
                        let mut encoder = display.device.create_command_encoder(&Default::default());
                        demo.render(&display, &frame, &mut encoder, dt);
                        display.queue.submit(Some(encoder.finish()));
                    },
                    Err(wgpu::SwapChainError::Lost) => {
                        display.resize(window.inner_size())
                    },
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    });
}

#[allow(dead_code)]
fn main() {
    println!("This is here only so the project compiles")
}