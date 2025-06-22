use eframe::egui;
use eframe::glow::HasContext;
use eframe::glow::{self};
use std::sync::{Arc,Mutex};
use rfd::FileDialog;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use crate::node::Node;
use crate::model::Model;
use crate::painter::PaintsMan;
use crate::drawable::{MeshCollection};
use std::collections::HashMap;

pub mod drawable;
pub mod node;
pub mod model;
pub mod painter;

const TOOLBAR_POINTER: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_restrict_select_on.svg";
const COLLAPSE: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_area_join_down.svg";
const ZOOM: &str = "file:///Users/avi/Documents/manual/assembly-theory/assets/blender_icon_view_zoom.svg";
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

pub enum Tool {
    None,
    Panning,
    Rotate
}
pub struct ViewProp {
    collapse_debugger: bool,
    to_fill: bool,
    model_updated: bool,
    model_loaded: bool
}

struct AssemblyTheory {
    viewport_painter: Arc<Mutex<PaintsMan>>,
    model: Option<Model>,
    view_prop: Arc<Mutex<ViewProp>>,
    logger: Logger,
    file_assets: HashMap<String,(u32,u32,Vec<u8>)>,
    current_tool: Tool
}

impl AssemblyTheory {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // let gl = 
        //     cc.gl.as_ref().expect("You need to run eframe with the glow backend");
        let mut inst = Self {
            view_prop: Arc::new(Mutex::new(
                ViewProp {
                    collapse_debugger: false, 
                    to_fill: true,
                    model_updated: false,
                    model_loaded: false
            })),
            viewport_painter: Arc::new(Mutex::new(PaintsMan::new())),
            model: None,
            logger: Logger{logs: Vec::new()},
            file_assets: HashMap::new(),
            current_tool: Tool::None
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

impl AssemblyTheory {
    //Replace angle with UI state to generate the geometries
    fn extract_geometries(&mut self) -> MeshCollection {
        match &self.model {
            None => MeshCollection::new(),
            Some(model) => {
                let node = &model.world_body;
                if let Some(node) = node {
                    match *node.borrow_mut() {
                        Node::WorldBody(ref node) => {
                            let geometries = node.getAllGeometries();
                            geometries
                        },
                        _ => MeshCollection::new()
                    }
                } else {
                    MeshCollection::new()
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
        let mut angle = egui::Vec2 {x:0.0,y:0.0};
        let mut translation = egui::Vec2 {x:0.0,y:0.0};
        match self.current_tool {
            Tool::Panning => {
                translation = egui::Vec2 {
                    x: response.drag_motion().x * 0.1,
                    y: response.drag_motion().y * 0.1
                };
            },
            Tool::Rotate => {
                angle = egui::Vec2 {
                    x: response.drag_motion().x * 0.1,
                    y: response.drag_motion().y * 0.1
                };
            },
            _ => ()
        }
        
        let viewport_painter = self.viewport_painter.clone();

        if self.view_prop.lock().expect("Failed to lock view prop").model_updated {
            let view_props = self.view_prop.clone();
            let geometries = self.extract_geometries();
            let mut textures_data = Vec::new();
            if let Some(model) = &self.model {
                textures_data = model.get_textures_data(); 
            }

            if !geometries.drawable_meshes.is_empty() {
                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                        move |_info, painter| {
                        let mut painter_l = 
                            viewport_painter.lock().expect("Issue locking the drawing struct.");
                        painter_l.update_viewport(translation,angle,zoom_scale).
                            refresh_draw(painter.gl(),
                                geometries.clone(),
                                textures_data.clone());
                        {
                            //TOFIX: This is horrendous. Can't I reset this flag in the same thread??
                            //Or perhaps do it at the refresh_draw end which is the source of updates
                            view_props.lock().expect("Issue locking viewprops object").model_updated = false;
                        }
                        painter_l.paint(view_props.clone(), painter.gl());
                    })),
                };
                ui.painter().add(callback);
            }
        } else if self.view_prop.lock().expect("Failed to lock view prop").model_loaded {
            let view_props = self.view_prop.clone();
            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                    move |_info, painter| {
                    viewport_painter.lock().expect("Issue locking the drawing struct.").
                            update_viewport(translation,angle,zoom_scale).
                            paint(view_props.clone(), painter.gl());
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
                            self.model = Some(Model::load_model(file));
                            self.view_prop.lock().expect("Expected view prop lock to be available").
                                model_updated = true;
                            self.view_prop.lock().expect("Expected view prop lock to be available").
                                model_loaded = true;
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
                ui.horizontal_wrapped(|ui| {
                    if ui.add(egui::Button::
                        image(egui::Image::from_uri(TOOLBAR_POINTER).
                        fit_to_exact_size(egui::Vec2{x:15.0,y:15.0}))).clicked() {
                            self.current_tool = Tool::Rotate;
                        }
                    if ui.add(egui::Button::
                        image(egui::Image::from_uri(ZOOM).
                        fit_to_exact_size(egui::Vec2{x:15.0,y:15.0}))).clicked() {
                            self.current_tool = Tool::Panning;
                        }
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