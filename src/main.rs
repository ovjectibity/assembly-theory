use eframe::egui;
use eframe::glow::HasContext;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::str::FromStr;
use std::rc::Rc;
use std::cell::RefCell;
use eframe::glow::{self};
use std::sync::{Arc,Mutex};
use rfd::FileDialog;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use crate::model::Node;
use crate::model::NodeType;
use crate::model::WorldBody;
use crate::model::Defaults;
use crate::model::AssetsManager;
use crate::model::Mesh;
use crate::model::Body;
use crate::model::Geom;
use crate::model::Joint;
use crate::model::Texture;
use crate::model::Material;
use nalgebra::{DMatrix, Matrix3, Matrix4};
use image::ImageReader;
use std::HashMap;

pub mod model;

const TOOLBAR_POINTER: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_restrict_select_on.svg";
const COLLAPSE: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_area_join_down.svg";
const EXPAND: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_area_join_up.svg";
const LEFT_COLLAPSE: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_area_join_left.svg";
const EXPAND_RIGHT: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_area_join.svg";

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        //with_inner_size([500.0, 500.0]),
        renderer: eframe::Renderer::Glow,
        depth_buffer: 32,
        stencil_buffer: 8,
        ..Default::default()
    };
    eframe::run_native(
        "AssemblyTheory-0.0.1",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<AssemblyTheory>::new(AssemblyTheory::new(cc)))
        }),
    )
}

struct Logger {
    logs: Vec<String>
}

impl Logger {
    fn new() -> Self {
        Logger {
            logs: Vec::new()
        }
    }

    fn add_log(&mut self,log: &str) {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into(); // Convert to UTC
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        self.logs.push(formatted + " " + log);
    }
}

struct ViewProp {
    collapse_debugger: bool,
    to_fill: bool
}

struct AssemblyTheory {
    viewport_painter: Arc<Mutex<PaintsMan>>,
    model: Option<Rc<RefCell<Node>>>,
    defaults: Option<Rc<RefCell<Node>>>,
    assets: Option<Rc<RefCell<Node>>>,
    view_prop: Arc<Mutex<ViewProp>>,
    logger: Logger
}

impl AssemblyTheory {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // let gl = 
        //     cc.gl.as_ref().expect("You need to run eframe with the glow backend");
        let mut inst = Self {
            view_prop: Arc::new(Mutex::new(ViewProp {collapse_debugger: false, to_fill: true})),
            viewport_painter: Arc::new(Mutex::new(PaintsMan::new())),
            // angle: egui::Vec2::new(0.0,0.0),
            model: None,
            assets: None,
            defaults: None,
            logger: Logger{logs: Vec::new()}
        };
        Self::initialize_gl_context(&cc);
        inst.logger.add_log("No model loaded.");
        inst
    }

    fn initialize_gl_context(cc: &eframe::CreationContext<'_>) {
        unsafe {
            let gl_c = cc.gl.as_ref().expect("You need to run eframe with the glow backend");
            gl_c.enable(glow::DEPTH_TEST);
        }
    }
}

struct PaintsMan {
    program: Option<glow::Program>,
    vertex_array: Option<glow::VertexArray>,
    viewport_transform_matrix: Matrix3<f32>
}

impl PaintsMan {
    fn new() -> Self {
        Self {
            program: None,
            vertex_array: None,
            viewport_transform_matrix: Matrix3::<f32>::new(1.0,0.0,0.0,
                                                            0.0,1.0,0.0,
                                                        0.0,0.0,1.0)
        }
    }

    fn applyViewportTransform(&mut self, rotateX: f32,rotateY: f32, zoom_scale: f32) {
        // println!("Applying the viewport rotation {} {}",rotateX, rotateY);
        let rotation_params: (f32,f32,f32) = if rotateY.abs() > rotateX.abs() {
            (rotateY,0.0,0.0)
        } else {
            (0.0,rotateX,0.0)
        };
        let rotation_s = Matrix3::new(
                zoom_scale, 0.0, 0.0,
                0.0, zoom_scale, 0.0,
                0.0, 0.0, zoom_scale,
        );
        let rotation_m = Matrix3::new(
                rotation_params.1.cos() * rotation_params.2.cos(), 
                rotation_params.1.cos() * rotation_params.2.sin(), 
                -rotation_params.1.sin(), 

                rotation_params.0.sin() * rotation_params.1.sin() * rotation_params.2.cos() - 
                rotation_params.0.cos() * rotation_params.2.sin(), 
                rotation_params.0.sin() * rotation_params.1.sin() * rotation_params.2.sin() + 
                rotation_params.0.cos() * rotation_params.2.cos(), 
                rotation_params.0.sin() * rotation_params.1.cos(), 

                rotation_params.0.cos() * rotation_params.1.sin() * rotation_params.2.cos() + 
                rotation_params.0.sin() * rotation_params.2.sin(),
                rotation_params.0.cos() * rotation_params.1.sin() * rotation_params.2.sin() - 
                rotation_params.0.sin() * rotation_params.2.cos(),
                rotation_params.0.cos() * rotation_params.1.cos()
        );
        // println!("Applying the viewport rotation {} to {}",rotation_m,self.viewport_transform_matrix);
        self.viewport_transform_matrix = rotation_m * rotation_s * self.viewport_transform_matrix;
        // println!("To obtain {}",self.viewport_transform_matrix);
    }

    fn refresh_draw(&mut self, gl: &glow::Context, mut vertices: Vec<f32>, 
            indices: Vec<u32>, angle: egui::Vec2, zoom_scale: f32) -> &mut Self {
        self.destroy(gl);
        use glow::HasContext as _;

        self.applyViewportTransform(angle.x, angle.y, zoom_scale);
        //applying the transform on the CPU, instead of via the vertex shader
        // vertices = Vec::from((self.viewport_transform_matrix * 
        //     DMatrix::from_vec(3,vertices.len() / 3,vertices)).as_slice());

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"
                    layout(location = 0) in vec3 verts;
                    uniform mat4 projection;
                    const vec4 colors[3] = vec4[3](
                        vec4(1.0, 0.0, 0.0, 1.0),
                        vec4(0.0, 1.0, 0.0, 1.0),
                        vec4(0.0, 0.0, 1.0, 1.0)
                    );
                    out vec4 v_color;
                    uniform mat3 viewport_transform;
                    void main() {
                        v_color = colors[int(mod(gl_VertexID,3))];
                        gl_Position = projection * vec4(viewport_transform * verts, 1.0);
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec4 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = v_color;
                    }
                "#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();
            
            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array: glow::NativeVertexArray = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

            // println!("Vertexes length is: {}",vertices.len());
            // println!("Elements length is: {}",indices.len());
            let mut vertexes: Vec<u8> = bincode::encode_to_vec(vertices,
                bincode::config::standard().with_fixed_int_encoding()).expect("Failed to serialise via bincode.");
            // assert!(vertexes.len() == (8+96)); //Length encoding followed by vertices
            let mut elements: Vec<u8> = bincode::encode_to_vec(indices,
                bincode::config::standard().with_fixed_int_encoding()).expect("Failed to serialise via bincode.");
            // assert!(elements.len() == (8+144)); //Length encoding followed by indices
            vertexes.drain(..8);
            elements.drain(..8);

            let vertex_buffer = gl.create_buffer().expect("Cannot create vertex buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertexes.as_slice(),glow::STATIC_DRAW);
            let elements_buffer = gl.create_buffer().expect("Cannot create elements buffer");
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER,Some(elements_buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, elements.as_slice(), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(0, 3, 
                        glow::FLOAT, false, 3*4, 0);
            gl.enable_vertex_attrib_array(0);
            gl.bind_vertex_array(None);

            self.program = Some(program);
            self.vertex_array = Some(vertex_array);
        }
        self
    }

    fn destroy(&mut self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            if let Some(program) = self.program {
                gl.delete_program(program);
                self.program = None;
            }
            if let Some(vertex_array) = self.vertex_array {
                gl.delete_vertex_array(vertex_array);
                self.vertex_array = None;
            }
        }
    }

    fn paint(&self,view_props: Arc<Mutex<ViewProp>>, gl: &glow::Context, angle: egui::Vec2, width: f32, height: f32, len_indices: i32) {
        use glow::HasContext as _;
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            let mut viewport_r: [i32;4] = [0,0,0,0];
            gl.get_parameter_i32_slice(glow::VIEWPORT, &mut viewport_r);
            let aspect_r: f32 = (viewport_r[2] as f32) / (viewport_r[3] as f32);

            gl.use_program(self.program);
            gl.uniform_matrix_3_f32_slice(
                gl.get_uniform_location(self.program.expect("No program available in PaintsMan instance"), "viewport_transform").as_ref(),
                false,
                self.viewport_transform_matrix.as_slice() //TO CONFIRM: I hope this won't get consumed on using this here
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program.expect("No program available in PaintsMan instance"), "projection").as_ref(),
                false,
                Matrix4::<f32>::new(
                    1.0 / aspect_r, 0.0, 0.0, 0.0, 
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0).as_slice()
            );
            gl.bind_vertex_array(Some(self.vertex_array.expect("No vertex array available in PaintsMan instance")));
            // Draw elements from the EBO instead
            // println!("Drawing {} indices",len_indices);
            // gl.polygon_mode(glow::FRONT_AND_BACK,glow::LINE);
            gl.depth_mask(true);
            if !view_props.lock().expect("Had an issue locking the viewport properties").to_fill {
                gl.polygon_mode(glow::FRONT_AND_BACK,glow::LINE);
            } else {
                gl.polygon_mode(glow::FRONT_AND_BACK,glow::FILL);
            }
            gl.draw_elements(glow::TRIANGLES, len_indices, glow::UNSIGNED_INT, 0);
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
        }
    }
}

impl AssemblyTheory {
    fn load_model(file: std::path::PathBuf) -> (Option<Rc<RefCell<Node>>>,Option<Rc<RefCell<Node>>>,Option<Rc<RefCell<Node>>>) {
        let contents = fs::read_to_string(file).unwrap();
        let mut reader = Reader::from_str(contents.as_str());
        let mut buf = Vec::new();
        let mut wbody: Option<Rc<RefCell<Node>>> = None; 
        let mut defaults: Option<Rc<RefCell<Node>>> = None;
        let mut assets: Option<Rc<RefCell<Node>>> = None;
        let mut p_stack: Vec<Rc<RefCell<Node>>> = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::End(e)) => {
                    let current_node = p_stack.pop();
                    if let Some(current_node) = current_node {
                        let current_node_type = current_node.borrow().get_node_name();
                        match e.name().as_ref() {
                            b"worldbody" => {
                                if current_node_type != "worldbody" {
                                    panic!("Expected worldbody on the p_stack. Got {}",current_node_type);
                                }
                            },
                            b"body" => {
                                if current_node_type != "body" {
                                    panic!("Expected body on the p_stack. Got {}",current_node_type);
                                } 
                            },
                            b"default" => {
                                if current_node_type != "default" {
                                    panic!("Expected default on the p_stack. Got {}",current_node_type);
                                } 
                            },
                            b"asset" => {
                                if current_node_type != "asset" {
                                    panic!("Expected asset on the p_stack. Got {}",current_node_type);
                                } 
                            },
                            _ => ()
                        }
                    }
                },
                
                Ok(Event::Start(e)) => {
                    let mut attributes: Vec<(String,String)> = Vec::new();
                    for attr in e.attributes() {
                        match attr {
                            Err(e) => panic!("Error extracting attributes."),
                            Ok(quick_xml::events::attributes::Attribute{key:k,value:v}) => {
                                let key = std::str::from_utf8(k.into_inner());
                                let value = std::str::from_utf8(&v);
                                match (key,value) {
                                    (Ok(k),Ok(v)) => {
                                        attributes.push((String::from_str(k).expect("error deriving string"),
                                                String::from_str(v).expect("error deriving string")));
                                    }
                                    _ => panic!("Error deriving attributes")
                                }
                            }
                        }
                    }
                    let child = match e.name().as_ref() {
                        b"worldbody" => {
                            println!("Worldbody detected.");
                            //process attributes
                            let x = Rc::new(RefCell::new(Node::WorldBody(WorldBody::default())));
                            x.borrow_mut().add_attrs(attributes);
                            wbody = Some(x.clone());
                            Some(x)
                        },
                        b"default" => {
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    println!("Nested default detected.");
                                    let mut parent = e.borrow_mut();
                                    let mut node = Node::Defaults(Defaults::new());
                                    node.add_attrs(attributes);
                                    let child = parent.add_child(node);  
                                    Some(child)
                                },
                                None => {
                                    println!("Root level default detected");
                                    let x = Rc::new(RefCell::new(Node::Defaults(Defaults::new())));
                                    x.borrow_mut().add_attrs(attributes);
                                    defaults = Some(x.clone());
                                    Some(x)
                                }
                            }
                        },
                        b"asset" => {
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    panic!("Wasn't expecting asset to be nested within a parent"); 
                                    None
                                },
                                None => {
                                    println!("Root level asset detected");
                                    let x = Rc::new(RefCell::new(Node::Assets(AssetsManager::new())));
                                    x.borrow_mut().add_attrs(attributes);
                                    assets = Some(x.clone());
                                    Some(x)
                                }
                            }
                        },
                        b"body" => {
                            println!("body detected.");
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    let mut parent = e.borrow_mut();
                                    let mut node = Node::Body(Body::default());
                                    node.add_attrs(attributes);
                                    let child = parent.add_child(node);  
                                    Some(child)
                                },
                                None => {
                                    panic!("No parent detected in the p_stack when parsing body start tag.");
                                    None
                                }
                            }
                        }
                        _ => None
                    };
                    child.map_or((), |e| {
                        p_stack.push(e);
                    })
                },

                Ok(Event::Empty(e)) => {
                    let mut attributes: Vec<(String,String)> = Vec::new();
                    for attr in e.attributes() {
                        match attr {
                            Err(e) => panic!("Error extracting attributes."),
                            Ok(quick_xml::events::attributes::Attribute{key:k,value:v}) => {
                                let key = std::str::from_utf8(k.into_inner());
                                let value = std::str::from_utf8(&v);
                                match (key,value) {
                                    (Ok(k),Ok(v)) => {
                                        attributes.push((String::from_str(k).expect("error deriving string"),String::from_str(v).expect("error deriving string")));
                                    }
                                    _ => panic!("Error deriving attributes")
                                }
                            }
                        }
                    }
                    match e.name().as_ref() {
                        b"geom" => {
                            println!("Geom detected.");
                            let parent = p_stack.last();
                            if let Some(parent) = parent {
                                let mut node = Node::Geom(Geom::default());
                                node.add_attrs(attributes);
                                parent.borrow_mut().add_child(node);
                            } else {
                                panic!("No parent detected in the p_stack when parsing joint tag.");
                            }
                        },
                        b"joint" => {
                            println!("joint detected.");
                            let parent = p_stack.last();
                            if let Some(parent) = parent {
                                let mut node = Node::Joint(Joint::default());
                                node.add_attrs(attributes);
                                parent.borrow_mut().add_child(node);
                            } else {
                                panic!("No parent detected in the p_stack when parsing joint tag.");
                            }
                        },
                        b"mesh" => {
                            println!("Mesh detected.");
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    let mut parent = e.borrow_mut();
                                    let mut node = Node::Mesh(Mesh::new("".to_string()));
                                    node.add_attrs(attributes);
                                    parent.add_child(node);  
                                },
                                None => {
                                    panic!("No parent detected in the p_stack when parsing mesh tag.");
                                }
                            }
                        },
                        b"material" => {
                            println!("Material detected.");
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    let mut parent = e.borrow_mut();
                                    let mut node = Node::Material(Material::new());
                                    node.add_attrs(attributes);
                                    parent.add_child(node);  
                                },
                                None => {
                                    panic!("No parent detected in the p_stack when parsing material tag.");
                                }
                            }
                        },
                        b"texture" => {
                            println!("Texture detected.");
                            let parent = p_stack.last();
                            match parent {
                                Some(e) => {
                                    let mut parent = e.borrow_mut();
                                    let mut node = Node::Texture(Texture::new());
                                    node.add_attrs(attributes);
                                    parent.add_child(node);  
                                },
                                None => {
                                    panic!("No parent detected in the p_stack when parsing texture tag.");
                                }
                            }
                        },
                        _ => ()
                    };
                },
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        if let Some(e) = &defaults {
            if let Some(w) = &wbody {
                match &mut *e.borrow_mut() {
                    Node::Defaults(d) => {
                        d.apply_defaults(w.clone(),None);
                    },
                    _ => {}
                }
            }
        }
        if let Some(e) = &assets {
            if let Some(w) = &wbody {
                 w.borrow_mut().apply_assets(e.clone());
                println!("{:?}",*w.borrow());
            }
            let mut asset_manager = e.borrow_mut();
            //WARNING: Good chance this will fail due to multiple borrows
            // asset_manager.apply_assets(e.clone()); 
            match &*asset_manager {
                Node::Assets(ref m) => {
                    let files = m.to_load_files();
                    let files_map: HashMap<String,Vec<u8>> = Vec::new();
                    for file in files {
                        let img = ImageReader::open(file.as_str())?.decode()?.to_rgb8();
                        //TODO: Do we really need to do a costly clone here? 
                        let raw_img = img.as_raw().clone();
                        files_map.insert(file,raw_img);
                    }
                    m.load_files(files_map);
                },
                _ => ()
            }
        }
        (wbody,assets,defaults)
    }

    //Replace angle with UI state to generate the geometries
    fn extract_geometries(&mut self) -> Vec<(Vec<f32>,Vec<u32>)> {
        match &self.model {
            None => Vec::new(),
            Some(node) => {
                match *node.borrow_mut() {
                    Node::WorldBody(ref node) => {
                        let geometries = node.getAllGeometries();
                        geometries
                    },
                    _ => Vec::new()
                }
            }
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui, zoom_scale: f32) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::new(
                ui.available_width(),ui.available_height()), 
                egui::Sense::drag());
        // Clone locals so we can move them into the paint callback:
        let angle = egui::Vec2 {
            x: response.drag_motion().x * 0.1,
            y: response.drag_motion().y * 0.1
        };
        let width = ui.available_width();
        let height = ui.available_height();
        let viewport_painter = self.viewport_painter.clone();
        let geometries = self.extract_geometries();
        let mut indice_acc = 0;
        let len_indices: usize = geometries.iter().map(|e| {
            e.1.len()
        }).sum();
        let vertices: Vec<f32> = geometries.iter().flat_map(|e| {
            e.0.clone()
        }).collect();
        let indices: Vec<u32> = geometries.iter().flat_map(|e| {
            let temp_acc = indice_acc as u32;
            indice_acc += e.1.len();
            e.1.iter().map(move |f | f+temp_acc)
        }).collect();
        let view_props = self.view_prop.clone();

        if !geometries.is_empty() {
            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                    move |_info, painter| {
                    viewport_painter.lock().expect("Issue locking the drawing struct.").
                        refresh_draw(painter.gl(),
                        vertices.clone(),
                        indices.clone(),
                        angle,
                        zoom_scale).  
                        paint(view_props.clone(), painter.gl(), angle, width, height, len_indices as i32);
                })),
            };
            ui.painter().add(callback);
        }
    }
}

impl eframe::App for AssemblyTheory {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu-bar").
            frame(egui::Frame::default().
            // fill(egui::Color32::DARK_GRAY).
            inner_margin(egui::Margin::same(5))).
            show(ctx, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import").clicked() {
                        if let Some(file) = FileDialog::new().
                            add_filter("models", &["xml"]).pick_file() {
                            (self.model,self.assets,self.defaults) = AssemblyTheory::load_model(file);
                        }
                        ui.close_kind(egui::UiKind::Menu);
                    }
                });
            });
        if !self.view_prop.lock().expect("Had an issue locking the viewport properties").
            collapse_debugger {
            egui::TopBottomPanel::bottom("debugging-panel").min_height(200.0).
                frame(egui::Frame::default().
                inner_margin(egui::Margin::same(10))).
                resizable(true).show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.add(egui::Button::
                        image(egui::Image::from_uri(COLLAPSE).
                        fit_to_exact_size(egui::Vec2{x:15.0,y:15.0}))).clicked() {
                            self.view_prop.lock().expect("Had an issue locking the viewport properties").
                                collapse_debugger = true;
                    }
                    for log in &self.logger.logs {
                        ui.label(log);
                    }
                })
            });
        }
        else {
            egui::TopBottomPanel::bottom("debugging-panel").max_height(50.0).
                frame(egui::Frame::default().
                inner_margin(egui::Margin::same(5))).
                show(ctx, |ui| {// Add a lot of widgets here.
                    if ui.add(egui::Button::
                        image(egui::Image::from_uri(EXPAND).
                        fit_to_exact_size(egui::Vec2{x:15.0,y:15.0}))).clicked() {
                            self.view_prop.lock().expect("Had an issue locking the viewport properties").
                                collapse_debugger = false;
                    }
            });
        }
        egui::SidePanel::right("hierarchy-view").min_width(200.0).
            resizable(true).
            show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Model hierarchy");
                });
            });
        egui::SidePanel::left("toolbar").max_width(40.0).
            frame(egui::Frame::default().
            // fill(egui::Color32::DARK_GRAY).
            // outer_margin(egui::Margin::same(5)).
            inner_margin(egui::Margin::same(5))).
            show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    ui.add(egui::Button::
                        image(egui::Image::from_uri(TOOLBAR_POINTER).
                        fit_to_exact_size(egui::Vec2{x:15.0,y:15.0})));
                });
            });
        egui::CentralPanel::default().frame(egui::Frame::default().
            inner_margin(egui::Margin::same(10))).
            show(ctx, |ui| {
                let mut zoom_scale: f32 = 0.0;
                ctx.input(|r| {
                    zoom_scale = r.zoom_delta();
                });
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui,zoom_scale);
                });
            });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.viewport_painter.lock().
                expect("Issue locking the drawing struct.").destroy(gl);
        }
    }
}