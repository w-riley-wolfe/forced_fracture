#![feature(core)] 
#![feature(box_syntax)]
#![allow(unstable)]
#![feature(collections)]
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;
use std::cell::RefCell;
use location::Vec2d;
//use core::slice::Iter;

//add my mods here
mod location;
mod world;
mod enitys;
mod render;
mod things;

thread_local!(static root: RefCell<world::World<'static>> = RefCell::new(world::World::new()));

fn main(){
    {
        let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Forced Fracture"))
            .build_glium().unwrap();
        root.with(|w| w.set_context(display));
    }
    let rend_engine: render::Render;
        
        root.with(|w| rend_engine = render::Render::new(w.contex()));

    let mut world: Vec<Box<enitys::Enity>> = Vec::new();
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(-20.1,-1.0)));
     world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(78.0,9.0)));
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(-45.0,45.0)));
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(23.0,-450.0)));
    world.push(box things::mobs::DevDan::
               new("Dan".to_string(),Vec2d::new(0.0,0.0)));
    world.push(box things::mobs::John::
               new("117".to_string(),Vec2d::new(-50.0,-70.0)));
    world.push(box things::mobs::John::
               new("104".to_string(),Vec2d::new(20.0,50.0)));
    let mut camera = location::Vec2d::new(0.0,0.0);

    loop{ //play loop
        //polling and handling the events received by the window
        let events;
        root.with(|w| events = w.contex().poll_events());
        for event in events{
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }

        //draw things
        let mut draws = Vec::new();
        for x in world.iter(){
            let temp = x.draw_handle();
            if let Some(thing) = temp {
                draws.push(thing);
            }
        }

        let mut target = display.draw();
        rend_engine.draw_frame(&display, draws.iter(), &camera);

    }//end main loop
}
