extern crate glutin;
extern crate glium;
use glium::{Surface,DisplayBuild};
use std::default::Default;


use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use super::{Event,Position,glium_support};



#[derive(Debug)]
pub enum Render {
    Quit,
    Pause(bool),
    Reset, // note: might be better to rebuild instead
    Move(Position,Position), //from,to
    
}

impl Render {

    pub fn stop(&self) {
        // post glutin::Closed to events
    }

    pub fn new() -> (Sender<Render>,Receiver<glutin::Event>) {
        let (inpt,inpr) = channel();
        let (gfxt,gfxr) = channel();

        // building the display, ie. the main object
        let display = glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();
        //let d2 = display.clone();

        thread::spawn(move || {

            // building the vertex and index buffers
            let vertex_buffer = glium_support::load_wavefront(&display, include_bytes!("data/teapot.obj"));

            // the program
            let program = glium::Program::from_source(&display,
                                                      // vertex shader
                                                      vert_sh,
                                                      // fragment shader
                                                      frag_sh,
                                                      // geometry shader
                                                      None).unwrap();


            // the main loop
            glium_support::start_loop(|| {
                let mut paused = false;

                // building the uniforms
                let uniforms = uniform! {
                    matrix: [
                        [0.005, 0.0, 0.0, 0.0],
                        [0.0, 0.005, 0.0, 0.0],
                        [0.0, 0.0, 0.005, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                            ]
                };

                // draw parameters
                let params = glium::DrawParameters {
                    //depth_function: glium::DepthFunction::IfLess,
                    .. Default::default()
                };

                // drawing a frame

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);
                target.draw(&vertex_buffer,
                            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            &program, &uniforms, &params).unwrap();
                target.finish();


                // polling and handling the events received by the window
                for event in display.poll_events() {
                    match event {
                        glutin::Event::Closed => return glium_support::Action::Stop,
                        glutin::Event::Resized(w,h) => (),
                        glutin::Event::Focused(focused) => {
                            if focused { paused = false; }
                            else { paused = true; }
                        },
                        
                        _ => { inpt.send(event); }, //send all other events to input thread
                    }
                }

                //poll for render commands from main thread
                let rc = gfxr.try_recv();
                if rc.is_ok() {
                    match rc.unwrap() {
                        Render::Quit => return glium_support::Action::Stop, //note: we must drop the display manually now!
                        _ => (),
                    }
                }

                glium_support::Action::Continue
            });

            inpt.send(glutin::Event::Closed); //shutdown input
            drop(display); //this prevents a weird bug since I'm threading and closing the display from outside the context; see Quit above
        });
        (gfxt,inpr)
    }
}

const vert_sh:&'static str =  "#version 110
    uniform mat4 matrix;
    attribute vec3 position;
    attribute vec3 normal;
    varying vec3 v_position;
    varying vec3 v_normal;
    void main() {
    v_position = position;
    v_normal = normal;
    gl_Position = vec4(v_position, 1.0) * matrix;
            }";

const frag_sh:&'static str = "#version 110
    varying vec3 v_normal;
    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
    void main() {
    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    gl_FragColor = vec4(color, 1.0);
            }";
