use std::error::Error;

use glium::{
    Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex, uniform,
    winit::{
        application::ApplicationHandler,
        event::{ElementState, StartCause, WindowEvent},
        event_loop::EventLoop,
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
};

use crate::{
    consts::{CANVAS_INDICES, CANVAS_POSITIONS, MAT4_ID},
    info_display,
    shaders::{CANVAS_VERT, JULIA_FRAG, MANDELBROT_FRAG},
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 4],
}

implement_vertex!(Vertex, position location(0));

#[derive(Debug)]
pub struct App<'a> {
    window: Window,
    display: Display<WindowSurface>,
    draw_params: DrawParameters<'a>,

    zoom: f32,
    center: [f32; 2],
    start: [f32; 2],
    palette_offset: f32,
    draw_mandel: bool, //if true draw Mandelbrot, else draw Julia

    //mandelbrot
    mand_prog: Program,

    //julia
    julia_prog: Program,

    canvas_v_buff: VertexBuffer<Vertex>,
    canvas_indices: IndexBuffer<u8>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new) => {
                self.display.resize(new.into());
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => self
                .draw()
                .unwrap_or_else(|err| eprintln!("Draw err : {err}")),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    match (key_code, event.state) {
                        // Moving the start point
                        (KeyCode::ArrowLeft, _) => {
                            self.start[0] -= 0.01;
                            self.window.request_redraw();
                        }
                        (KeyCode::ArrowRight, _) => {
                            self.start[0] += 0.01;
                            self.window.request_redraw();
                        }
                        (KeyCode::ArrowDown, _) => {
                            self.start[1] -= 0.01;
                            self.window.request_redraw();
                        }
                        (KeyCode::ArrowUp, _) => {
                            self.start[1] += 0.01;
                            self.window.request_redraw();
                        }

                        // Moving the center
                        (KeyCode::KeyA, _) => {
                            self.center[0] += 0.1 * self.zoom;
                            self.window.request_redraw();
                        }
                        (KeyCode::KeyD, _) => {
                            self.center[0] -= 0.1 * self.zoom;
                            self.window.request_redraw();
                        }
                        (KeyCode::KeyS, _) => {
                            self.center[1] += 0.1 * self.zoom;
                            self.window.request_redraw();
                        }
                        (KeyCode::KeyW, _) => {
                            self.center[1] -= 0.1 * self.zoom;
                            self.window.request_redraw();
                        }

                        // Changing the zoom
                        (KeyCode::ShiftLeft, _) | (KeyCode::ShiftRight, _) => {
                            self.zoom /= 1.1;
                            self.window.request_redraw();
                        }
                        (KeyCode::ControlLeft, _) | (KeyCode::ControlRight, _) => {
                            self.zoom *= 1.1;
                            self.window.request_redraw();
                        }

                        // Moving the palette offset
                        (KeyCode::NumpadSubtract, _) | (KeyCode::Minus, _) => {
                            self.palette_offset -= 0.01;
                            self.window.request_redraw();
                        }
                        (KeyCode::NumpadAdd, _) => {
                            self.palette_offset += 0.01;
                            self.window.request_redraw();
                        }

                        // Changing set to draw
                        (KeyCode::KeyJ, _) => {
                            if self.draw_mandel {
                                self.draw_mandel = false;
                                self.window.request_redraw();
                            }
                        }
                        (KeyCode::KeyM, _) => {
                            if !self.draw_mandel {
                                self.draw_mandel = true;
                                self.window.request_redraw();
                            }
                        }

                        // Reset
                        (KeyCode::Numpad0, ElementState::Released) => {
                            self.reset_params();
                            self.window.request_redraw();
                        }

                        // Help
                        (KeyCode::KeyH, ElementState::Released) => self.display_infos(),

                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &glium::winit::event_loop::ActiveEventLoop,
        cause: glium::winit::event::StartCause,
    ) {
        if cause == StartCause::Init {
            self.display_infos();
        }
    }
}

impl App<'_> {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn Error>> {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("Co's complex fractal")
            .build(event_loop);
        let draw_params = DrawParameters::default();

        let mand_prog = Program::from_source(&display, CANVAS_VERT, MANDELBROT_FRAG, None)?;
        let julia_prog = Program::from_source(&display, CANVAS_VERT, JULIA_FRAG, None)?;

        let canvas_v_buff = VertexBuffer::new(&display, &CANVAS_POSITIONS)?;
        let canvas_indices = IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TriangleFan,
            &CANVAS_INDICES,
        )?;

        Ok(Self {
            window,
            display,
            draw_params,

            zoom: 3.,
            center: [0.5, 0.25],
            start: [0.; 2],
            palette_offset: 0.5,
            draw_mandel: true,

            mand_prog,
            julia_prog,

            canvas_v_buff,
            canvas_indices,
        })
    }

    fn draw(&self) -> Result<(), Box<dyn Error>> {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.7, 0.7, 0.7, 1.), 0.);

        let resolution = self.window.inner_size();
        let resolution = (resolution.width as f32, resolution.height as f32);
        let uni = uniform! {
            resolution : resolution,
            transforms : MAT4_ID,
            start : self.start,
            center : self.center,
            palette_offset : self.palette_offset,
            zoom : self.zoom
        };
        if self.draw_mandel {
            frame.draw(
                &self.canvas_v_buff,
                &self.canvas_indices,
                &self.mand_prog,
                &uni,
                &self.draw_params,
            )?;
        } else {
            frame.draw(
                &self.canvas_v_buff,
                &self.canvas_indices,
                &self.julia_prog,
                &uni,
                &self.draw_params,
            )?;
        }

        frame.finish()?;

        Ok(())
    }

    fn display_infos(&self) {
        let drawed = if self.draw_mandel {
            "Mandelbrot"
        } else {
            "Julia"
        };
        println!(
            info_display!(),
            drawed,
            self.center[0],
            self.center[1],
            self.start[0],
            self.start[1],
            self.zoom,
            self.palette_offset
        );
    }

    fn reset_params(&mut self) {
        self.zoom = 3.;
        self.center = [0.5, 0.25];
        self.start = [0.; 2];
        self.palette_offset = 0.;
    }
}
