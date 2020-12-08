use anyhow::{Result};
use winit::event::*;

mod framework;
use framework::Display;

// Things that I need automated
// * vertex buffer descriptors
// * bind groups / bind group layouts

// crate::include_pipeline!{
//     "Comp" => "particles.comp",
// }

struct Demo {
    src_particles: wgpu::Buffer,
    dst_particles: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    simulation_buffer: wgpu::Buffer,
}

impl framework::Demo for Demo {
    fn new(display: &Display) -> Result<Self> {
        // This is where we need the particle struct
        todo!();
    }

    fn resize(&mut self, _display: &Display) {

    }

    fn device_event(&mut self, event: &DeviceEvent) -> bool {
        match event {
            _ => false,
        }
    }

    fn render(&mut self, display: &Display, frame: &wgpu::SwapChainFrame, encoder: &mut wgpu::CommandEncoder, dt: std::time::Duration) {
        
    }
}

fn main() -> Result<()> {
    framework::run::<Demo>("Particles")
}