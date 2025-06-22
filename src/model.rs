use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::str::FromStr;
use image::ImageReader;
use std::rc::Rc;
use std::cell::RefCell;
use crate::node::Node;
use crate::node::NodeType;
use crate::node::WorldBody;
use crate::node::Defaults;
use crate::node::AssetsManager;
use crate::node::Mesh;
use crate::node::Body;
use crate::node::Geom;
use crate::node::Joint;
use crate::node::Texture;
use crate::node::Material;
use std::collections::HashMap;

#[derive(Clone)]
pub enum FileType {
    Image(u32,u32)
}

pub struct Model {
    pub world_body: Option<Rc<RefCell<Node>>>,
    pub defaults: Option<Rc<RefCell<Node>>>,
    pub asset_manager: Option<Rc<RefCell<Node>>>,
    pub asset_files: HashMap<String,(FileType,Vec<u8>)>
}

impl Model {
    pub fn get_textures_data(&self) -> Vec<Texture> {
        let mut textures = Vec::new();
        let assets = &self.asset_files;
        if let Some(asset_manager) = &self.asset_manager {
            match &*asset_manager.borrow() {
                Node::Assets(ass) => {
                    let available_textures = ass.get_textures_data();
                    for mut tex in available_textures {
                        if assets.contains_key(&tex.file) {
                            tex.file_data = assets.get(&tex.file).
                                expect("Expected asset to be available").1.clone();
                            textures.push(tex);
                        }
                    }
                },
                _ => ()
            }
        }
        textures
    }

    pub fn load_model(file: std::path::PathBuf) -> Self {
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

        let mut image_files_map: HashMap<String,(FileType,Vec<u8>)> = HashMap::new();
        let mut obj_files_map: HashMap<String,String> = HashMap::new();
        if let Some(e) = &mut assets {
            if let Some(w) = &wbody {
                 w.borrow_mut().apply_assets(e.clone());
                println!("{:?}",*w.borrow());
            }
            let mut asset_manager = e.borrow_mut();
            match &mut *asset_manager {
                Node::Assets(ref mut m) => {
                    let files = m.to_load_files();
                    println!("Loading files now: {:?}",files);
                    let mut dimensions_map: HashMap<String,(u32,u32)> = HashMap::new();
                    for file in &files {
                        // println!("Opening file: {}",file);
                        if file.ends_with(".png") {
                            let img = 
                                ImageReader::open(("assets/assets/".to_string() + file.as_str()).as_str()).
                                expect("Expected to obtain asset file").decode().
                                expect("Expected to be able to decode asset file").to_rgb8();
                            //TODO: Do we really need to do a costly clone here? 
                            let raw_img = img.as_raw().clone();
                            // println!("Image loaded of dimensions {} {}",img.width(),img.height());
                            image_files_map.insert(file.clone(),
                                (FileType::Image(img.width(),img.height()),raw_img));
                            dimensions_map.insert(file.clone(),(img.width(),img.height()));
                        } else if file.ends_with(".obj") {
                            let file_raw = 
                                fs::read_to_string("assets/assets/".
                                    to_string() + file).expect("Expected to have the obj file.");
                            obj_files_map.insert(file.clone(), file_raw);
                        }
                    }      
                    m.load_dimensions(dimensions_map);
                    m.load_obj_meshes(obj_files_map);
                },
                _ => ()
            }
        }
        Self {
            world_body: wbody,
            defaults: defaults,
            asset_manager: assets,
            asset_files: image_files_map
        }
    }
}
