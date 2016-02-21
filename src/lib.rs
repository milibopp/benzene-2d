extern crate elmesque;
extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate piston;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;
extern crate benzene;

use benzene::{Driver, Communication};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use glium::Surface;
use glium_graphics::{Glium2d, GliumGraphics, GliumWindow, GlyphCache};
use glutin_window::GlutinWindow;
use carboxyl_window::{RunnableWindow, StreamingWindow, SourceWindow, Context,
                      Event};
use elmesque::{Element, Renderer};
use shader_version::OpenGL;
use piston::window::WindowSettings;
use graphics::context;


pub struct Driver2d {
    glutin_window: Rc<RefCell<GlutinWindow>>,
    source_window: SourceWindow<Rc<RefCell<GlutinWindow>>>
}

impl Driver2d {
    pub fn new(settings: WindowSettings) -> Driver2d {
        let glutin_window = Rc::new(RefCell::new(GlutinWindow::new(settings).ok().unwrap()));
        let source_window = SourceWindow::new(glutin_window.clone());
        Driver2d {
            glutin_window: glutin_window,
            source_window: source_window
        }
    }
}

impl Driver<Communication<Element, ()>> for Driver2d {
    type Output = Communication<Context, Event>;

    fn output(&self) -> Communication<Context, Event> {
        Communication {
            context: self.source_window.context(),
            events: self.source_window.events()
        }
    }

    fn run(&mut self, input: Communication<Element, ()>) {
        const GLVERSION: OpenGL = OpenGL::V2_1;
        let glium_window = GliumWindow::new(&self.glutin_window).ok().unwrap();
        let mut backend_sys = Glium2d::new(GLVERSION, &glium_window);
        let mut glyph_cache = GlyphCache::new(&Path::new("./assets/NotoSans/NotoSans-Regular.ttf"),
                                              glium_window.clone())
                                  .unwrap();

        let canvas = lift!(|context, view| (context.window.size, view),
            &self.source_window.context(),
            &input.context
        );

        self.source_window.run_with(120.0, || {
            let ((w, h), element) = canvas.sample();
            let mut target = glium_window.draw();
            {
                let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
                let mut renderer = Renderer::new(context::Context::new_abs(w as f64, h as f64), &mut backend)
                                       .character_cache(&mut glyph_cache);
                element.draw(&mut renderer);
            }
            target.finish().unwrap();
        });
    }
}
