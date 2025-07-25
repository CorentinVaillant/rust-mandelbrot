use std::error::Error;

use glium::winit::event_loop::EventLoopBuilder;

use crate::app::App;


mod app;
mod shaders;
mod consts;

fn main()-> Result<(), Box<dyn Error>> {
    let ev_loop = EventLoopBuilder::default().build()?;
    let mut app = App::new(&ev_loop)?;

    ev_loop.run_app(&mut app)?;

    Ok(())
}
