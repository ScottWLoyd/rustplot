extern crate glium;
extern crate winit;

fn main() {
   use glium::{glutin, Surface};

   let mut events_loop = glium::glutin::EventsLoop::new();
   let window = glium::glutin::WindowBuilder::new().with_title("Graph Window").with_dimensions(1024, 768);
   let context = glium::glutin::ContextBuilder::new();   
   let display = glium::Display::new(window, context, &events_loop).unwrap();

   let mut closed = false;
   while !closed {
      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);
      target.finish().unwrap();

      events_loop.poll_events(|event| {
         match event {
            glutin::Event::WindowEvent { event, .. } => match event {
               glutin::WindowEvent::Closed => closed = true,
               _ => ()
            },
            _ => (),
         }
      });

      //display.swap_buffers();
      //std::thread::sleep(std::time::Duration::from_millis(17));
   }

}
