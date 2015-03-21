extern crate glutin;
extern crate glium;
use glium::{Surface,DisplayBuild};
use std::default::Default;

use std::cell::RefCell;

use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use super::{Game,BoardLayout,Event,Position, MoveType,MoveValid,Player, Item,glium_support};

extern crate cam;
use self::cam::{Camera,CameraPerspective};

extern crate image;
use std::old_io::BufReader;

use std::collections::HashMap;
use glium::vertex::VertexBufferAny;

extern crate "nalgebra" as na;
use self::na::{Vec3,Mat4, Rot3, UnitQuat, ToHomogeneous};

#[derive(Debug,Copy)]
pub enum Render {
    Quit,
    Pause(bool),
    Reset, // note: might be better to rebuild instead
    Animate(MoveValid), //perhaps include the Err results too? for visual feedback
    RotateScene((i32,i32)), //comes from input's drag
}

impl Render {
    pub fn new(w:u32,h:u32, board:BoardLayout) -> (Sender<Vec<Render>>,Receiver<glutin::Event>) {
        let (inpt,inpr) = channel();
        let (gfxt,gfxr) = channel();

        //let guard = thread::scoped
        thread::spawn(move || {
            // building the display, ie. the main object
            let display = glutin::WindowBuilder::new()
                .with_dimensions(w,h)
                .with_title(format!("Chess"))
                .with_depth_buffer(24)
                .build_glium()
                .unwrap();

            let img_lt = image::load(BufReader::new(include_bytes!("data/img_lt.png")),
                                    image::PNG).unwrap();
            let img_drk = image::load(BufReader::new(include_bytes!("data/img_drk.png")),
                                     image::PNG).unwrap();
            let tex_lt = glium::texture::CompressedTexture2d::new(&display, img_lt);
            let tex_drk = glium::texture::CompressedTexture2d::new(&display, img_drk);

            //setup projection mat
            let mut proj = Render::cam_proj(w,h);

            //build cam
            let mut cam = Render::cam_new();

            // building the vertex and index buffers
            let mut items: HashMap<Item,VertexBufferAny> = HashMap::new();
            let get_vbo = |bs| glium_support::load_wavefront(&display, bs);
            
            items.insert(Item::Queen,get_vbo(include_bytes!("data/queen.obj")));
            items.insert(Item::King(false),get_vbo(include_bytes!("data/king.obj")));
            items.insert(Item::Rook(false),get_vbo(include_bytes!("data/rook.obj")));
            items.insert(Item::King(true),get_vbo(include_bytes!("data/king.obj"))); //need both, peq is picky/accurate
            items.insert(Item::Rook(true),get_vbo(include_bytes!("data/rook.obj")));
            items.insert(Item::Bishop,get_vbo(include_bytes!("data/bishop.obj")));
            items.insert(Item::Knight,get_vbo(include_bytes!("data/knight.obj")));
            items.insert(Item::Pawn,get_vbo(include_bytes!("data/pawn.obj")));

            let item_inst = {
                #[derive(Copy)]
                struct Attr {
                    inst_position: [f32; 3],
                    inst_color: f32,
                }
                implement_vertex!(Attr, inst_position, inst_color);
                let mut data: Vec<Attr> = Vec::new();
                data.push(Attr {
                    inst_position: [0 as f32, 0f32, 0 as f32],
                    inst_color: 1.0f32,
                });
                glium::vertex::PerInstanceAttributesBuffer::new_if_supported(&display, data).unwrap()
            };

            let tile_verts = glium_support::load_wavefront(&display, include_bytes!("data/tile.obj"));
            let board_inst = {
                #[derive(Copy)]
                struct Attr {
                    inst_position: [f32; 3],
                    inst_color: f32,
                }
                implement_vertex!(Attr, inst_position, inst_color);

                let mut data: Vec<Attr> = Vec::new();
                for (x,_) in board.iter().enumerate() { 
                    for (z,_) in board.iter().enumerate() {
                        let mut color = 1.0f32;
                        if x % 2 == 1 && z % 2 == 1 ||
                            x % 2 == 0 && z % 2 == 0 { color = 0.0f32; }

                        data.push(Attr {
                            inst_position: [(x as f32 * 2.0f32), 0f32, (z as f32 * 2.0f32)],
                            inst_color: color,
                        });
                    }
                }

                glium::vertex::PerInstanceAttributesBuffer::new_if_supported(&display, data).unwrap()
            };



            let prog_board = glium::Program::from_source(&display,
                                                         VERT_SH_BOARD,
                                                         FRAG_SH,
                                                         None).unwrap();

            let prog_item = glium::Program::from_source(&display,
                                                        VERT_SH_ITEM,
                                                        FRAG_SH,
                                                        None).unwrap();


            // identity matrix for model
            // todo: build one for each piece, or use instancing
            let model = [[1.0, 0.0, 0.0, 0.0],
                         [0.0, 1.0, 0.0, 0.0],
                         [0.0, 0.0, 1.0, 0.0],
                         [0.0, 0.0, 0.0, 1.0f32]];


            let mut paused = false;
            
            // the main loop
            glium_support::start_loop(|| {
              //  cam.look_at([0f32,0f32,0f32]);
                let view = cam.orthogonal();
                let viewmat4 = *(Mat4::from_array_ref(&view));

                // draw parameters
                let params = glium::DrawParameters {
                    depth_test: glium::DepthTest::IfLess,
                    depth_write: true,
                    .. Default::default()
                };

                // drawing a frame
                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

                // draw board
                let uniform = uniform! { model: model,
                                         proj: proj,
                                         view: view,
                                         tex_lt: &tex_lt,
                                         tex_drk: &tex_drk };

                target.draw((&tile_verts, &board_inst),
                            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            &prog_board, &uniform, &params).unwrap();



                // draw pieces
                for (x,n) in board.iter().enumerate() { 
                    for (z,p) in n.iter().enumerate() { 
                        let mut color = 1.0f32;
                        let mat =  [[1.0, 0.0, 0.0, 0.0],
                                    [0.0, 1.0, 0.0, 0.0],
                                    [0.0, 0.0, 1.0, 0.0],
                                    [(x as f32 * 2.0f32), 0.0, (z as f32 * 2.0f32), 1.0f32]];

                        let mut nrot = na::BaseFloat::frac_pi_2();

                        if let Some(_p) = *p {
                            let r = match _p {
                                Player::White(i) => {
                                    color = 1.0f32;
                                    items.get(&i)
                                },
                                Player::Black(i) => {
                                    color = 0.0f32;
                                    nrot = na::BaseFloat::frac_pi_2() * -1.0f32;
                                    items.get(&i)
                                }
                            };

                            let transmat4: &Mat4<f32> = Mat4::from_array_ref(&mat);
                            let rotmat4 = Rot3::new_with_euler_angles(0.0f32,nrot,0.0).to_homogeneous(); //Rot3::new(nrot).to_homogeneous();

                            let uniform = uniform! 
                            { model: *(*transmat4*rotmat4).as_array(),
                              proj: proj,
                              view: view,
                              tex_lt: &tex_lt,
                              tex_drk: &tex_drk,
                              col: color
                            };

                            target.draw(r.unwrap(),
                                        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                                        &prog_item, &uniform, &params).unwrap();
                        }
                    }
                }

                
                target.finish();

                // polling and handling the events received by the window
                for event in display.poll_events() {
                    match event {
                        glutin::Event::Closed => return glium_support::Action::Stop,
                        glutin::Event::Resized(w,h) => { 
                            proj = Render::cam_proj(w,h);
                        },
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

    fn cam_new() -> Camera {
        let v = [25f32,8f32,20f32];
        let mut cam = Camera::new(v);
        cam.look_at([12f32,0f32,12f32]);
        cam
    }

    fn cam_proj(w:u32,h:u32) -> [[f32;4];4] {
        CameraPerspective {
            fov: 60.0f32,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (w as f32) / (h as f32)
        }.projection()
    }
}



const VERT_SH_BOARD:&'static str =  "#version 110
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    attribute vec3 position;
    attribute vec3 inst_position;
    attribute vec3 normal;
    attribute vec2 texture;
    attribute float inst_color;

    varying vec3 v_position;
    varying vec3 v_normal;
    varying vec2 v_tex;
    varying float v_col;

    void main() {
    v_position = position + inst_position;
    v_normal = normal;
    v_tex = texture;
    v_col = inst_color;
    gl_Position = proj * view * model * vec4(v_position, 1.0);
            }";

const FRAG_SH:&'static str = "#version 110
    varying vec3 v_position;
    varying vec3 v_normal;
    varying vec2 v_tex;
    varying float v_col;

    uniform sampler2D tex_lt;
    uniform sampler2D tex_drk;

    const vec3 LIGHT = vec3(-5.0, 200.0, 5);
    void main() {

    float lum = max(dot(normalize(v_normal), normalize(LIGHT-v_position)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    if (v_col < 1.0) { gl_FragColor = vec4(color, 1.0) * texture2D(tex_drk, v_tex); }
    else { gl_FragColor = vec4(color, 1.0) * texture2D(tex_lt, v_tex); }

            }";


const VERT_SH_ITEM:&'static str =  "#version 110
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;
    uniform float col;

    attribute vec3 position;
    attribute vec3 normal;
    attribute vec2 texture;

    varying vec3 v_position;
    varying vec3 v_normal;
    varying vec2 v_tex;
    varying float v_col;

    void main() {
    v_position = position;
    v_normal = normal;
    v_tex = texture;
    v_col = col;
    gl_Position = proj * view * model * vec4(v_position, 1.0);
            }";


//ignoreme:
//proj * view * trans * rot * vertex_position
//"trans * rot" ahead of rendering into "modelview"
//shader:proj * view * model * vec4(v_position, 1.0)
//uniforms: "model: trans * rot"
