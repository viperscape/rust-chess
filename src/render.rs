extern crate glutin;
extern crate glium;
use glium::{Surface,DisplayBuild};
use std::default::Default;

use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use super::{Game,BoardLayout,Event,Position, MoveType,MoveValid,Player,glium_support};



#[derive(Debug,Copy)]
pub enum Render {
    Quit,
    Pause(bool),
    Reset, // note: might be better to rebuild instead
    Animate(MoveValid), //perhaps include the Err results too? for visual feedback
    RotateScene((i32,i32)), //comes from input's drag
}

impl Render {
    pub fn new() -> (Sender<Vec<Render>>,Receiver<glutin::Event>) {
        let (inpt,inpr) = channel();
        let (gfxt,gfxr) = channel();

        // building the display, ie. the main object
        let display = glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();
        //let d2 = display.clone();

        //let guard = thread::scoped
            thread::spawn(move || {

            // building the vertex and index buffers
            let vertex_buffer = glium_support::load_wavefront(&display, include_bytes!("data/knight.obj")); 

            // building the instances buffer
            let per_instance = {
                #[derive(Copy)]
                struct Attr {
                    world_position: [f32; 3],
                }

                implement_vertex!(Attr, world_position);

                let mut data = Vec::new();
                for x in (0u8 .. 1) {
                    //for y in (0u32 .. 82) {
                        data.push(Attr {
                           // world_position: [(x*100) as f32, 0 as f32, (x*100) as f32],
                            world_position: [0 as f32, -0.8 as f32, 0 as f32],
                        });
                    //}
                }

                glium::vertex::PerInstanceAttributesBuffer::new_if_supported(&display, data).unwrap()
            };


            // the program
            let program = glium::Program::from_source(&display,
                                                      // vertex shader
                                                      VERT_SH,
                                                      // fragment shader
                                                      FRAG_SH,
                                                      // geometry shader
                                                      None).unwrap();


            // the main loop
            glium_support::start_loop(|| {
                let mut paused = false;

                // building the uniforms
                let uniforms = uniform! {
                    matrix: [[1.0, 0.0, 0.0, 0.0],
                             [0.0, 1.0, 0.0, 0.0],
                             [0.0, 0.0, 1.0, 0.0],
                             [0.0, 0.0, 0.0, 1.0f32]]
                };

                // draw parameters
                let params = glium::DrawParameters {
                    //depth_function: glium::DepthFunction::IfLess,
                    .. Default::default()
                };

                // drawing a frame

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);
                target.draw((&vertex_buffer, &per_instance),
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
                if !paused {
                    let rc = gfxr.try_recv();
                    if rc.is_ok() {
                        for n in rc.unwrap() {
                            match n {
                                Render::Quit => return glium_support::Action::Stop,
                                Render::Pause(p) => {
                                    paused = p;
                                },
                                Render::Reset => (),
                                _ => Render::render_cmd(n),
                            }
                        }
                    }
                }

                glium_support::Action::Continue
            });

            inpt.send(glutin::Event::Closed); //shutdown input
        });
        (gfxt,inpr)//,guard)
    }

    fn render_cmd(rc: Render) {
        match rc {
            Render::Animate(mv) => {
                if let Some(cap) = mv.cap { };
                if let Some(check) = mv.check {};
                match mv.mt {
                    MoveType::Regular | MoveType::Double(_) => {
                        println!("{:?}",mv.item.play_path(mv.mv.0,mv.mv.1));
                    },
                    MoveType::Castle => {
                        println!("{:?}",mv.item.castle_path(mv.mv.0,mv.mv.1));
                    },
                    MoveType::Upgrade => (),
                }
            },
            _ => (),
        }
    }
}

const VERT_SH:&'static str =  "#version 110
    uniform mat4 matrix;
    attribute vec3 position;
    attribute vec3 world_position;

    attribute vec3 normal;
    varying vec3 v_position;
    varying vec3 v_normal;
    void main() {
    v_position = position + world_position;
    v_normal = normal;
    gl_Position = vec4(v_position, 1.0) * matrix;
            }";

const FRAG_SH:&'static str = "#version 110
    varying vec3 v_normal;
    const vec3 LIGHT = vec3(-1.0, 3.0, 0.1);
    void main() {
    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    gl_FragColor = vec4(color, 1.0);
            }";
