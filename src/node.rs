use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::str::FromStr;
use nalgebra::{DMatrix, Matrix3};
use qhull::{Qh};
use std::collections::HashMap;
use crate::drawable::{MeshCollection,VertexFilling};
use tobj;
use ahash;

pub trait NodeType {
    fn add_attr(&mut self, key: String, value: String) -> bool;
    fn add_attrs(&mut self,attrs: Vec<(String,String)>) -> ();
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>>;
    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>>;
    fn get_node_name(&self) -> String;
    fn get_children(&self) -> Vec<Rc<RefCell<Node>>>;
    fn get_name(&self) -> String;
    fn get_set_attributes_list(&self) -> HashMap<String,String>;
    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> ();
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>>;

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
        println!("Assets not allowed on this node type.")
    }

    fn get_class(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug)]
pub enum Node {
    Geom(Geom),
    WorldBody(WorldBody),
    Body(Body),
    Joint(Joint),
    Defaults(Defaults),
    Assets(AssetsManager),
    Mesh(Mesh),
    Texture(Texture),
    Material(Material)
}

#[derive(Debug)]
pub enum Orientation {
    Euler(Euler),
    AxisAngle
}

#[derive(Debug)]
pub enum GeomType {
    Plane, 
    Hfield, 
    Sphere, 
    Capsule, 
    Ellipsoid, 
    Cylinder, 
    Box, 
    Mesh, 
    Sdf
}

#[derive(Debug)]
pub enum JointType {
    Free,
    Ball,
    Slide, 
    Hinge
}

#[derive(Debug, Clone)]
pub enum TextureType {
    TwoD,
    Cube,
    Skybox
}

#[derive(Debug)]
pub struct Euler {
    x: f32, 
    y: f32,
    z: f32
}

impl NodeType for Node {
    fn add_attr(&mut self, name: String, value: String) -> bool {
        match self {
            Node::Geom(e) => {
                e.add_attr(name,value)
            },
            Node::WorldBody(e) => {
                e.add_attr(name,value)
            },
            Node::Body(e) => {
                e.add_attr(name,value)
            },
            Node::Joint(e) => {
                e.add_attr(name,value)
            },
            Node::Defaults(e) => {
                e.add_attr(name,value)
            },
            Node::Mesh(e) => {
                e.add_attr(name,value)
            },
            Node::Assets(e) => {
                e.add_attr(name,value)
            },
            Node::Texture(e) => {
                e.add_attr(name,value)
            },
            Node::Material(e) => {
                e.add_attr(name,value)
            }
        }
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        match self {
            Node::Geom(e) => {
                e.add_attrs(attrs)
            },
            Node::WorldBody(e) => {
                e.add_attrs(attrs)
            },
            Node::Body(e) => {
                e.add_attrs(attrs)
            },
            Node::Joint(e) => {
                e.add_attrs(attrs)
            },
            Node::Defaults(e) => {
                e.add_attrs(attrs)
            },
            Node::Mesh(e) => {
                e.add_attrs(attrs)
            },
            Node::Assets(e) => {
                e.add_attrs(attrs)
            },
            Node::Texture(e) => {
                e.add_attrs(attrs)
            },
            Node::Material(e) => {
                e.add_attrs(attrs)
            }
        }
    }

    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        match self {
            Node::Geom(e) => {
                e.add_child(child)
            },
            Node::WorldBody(e) => {
                e.add_child(child)
            },
            Node::Body(e) => {
                e.add_child(child)
            },
            Node::Joint(e) => {
                e.add_child(child)
            },
            Node::Defaults(e) => {
                e.add_child(child)
            },
            Node::Assets(e) => {
                e.add_child(child)
            },
            Node::Mesh(e) => {
                e.add_child(child)
            },
            Node::Texture(e) => {
                e.add_child(child)
            },
            Node::Material(e) => {
                e.add_child(child)
            }
        }
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match self {
            Node::Geom(e) => {
                e.add_child_ref(child)
            },
            Node::WorldBody(e) => {
                e.add_child_ref(child)
            },
            Node::Body(e) => {
                e.add_child_ref(child)
            },
            Node::Joint(e) => {
                e.add_child_ref(child)
            },
            Node::Defaults(e) => {
                e.add_child_ref(child)
            },
            Node::Assets(e) => {
                e.add_child_ref(child)
            },
            Node::Mesh(e) => {
                e.add_child_ref(child)
            },
            Node::Texture(e) => {
                e.add_child_ref(child)
            },
            Node::Material(e) => {
                e.add_child_ref(child)
            }
        }
    }

    fn get_node_name(&self) -> String {
        match self {
            Node::Geom(e) => {
                e.get_node_name()
            },
            Node::WorldBody(e) => {
                e.get_node_name()
            },
            Node::Body(e) => {
                e.get_node_name()
            },
            Node::Joint(e) => {
                e.get_node_name()
            },
            Node::Defaults(e) => {
                e.get_node_name()
            },
            Node::Assets(e) => {
                e.get_node_name()
            },
            Node::Mesh(e) => {
                e.get_node_name()
            },
            Node::Texture(e) => {
                e.get_node_name()
            },
            Node::Material(e) => {
                e.get_node_name()
            }
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        match self {
            Node::Geom(e) => {
                e.set_parent(parent)
            },
            Node::WorldBody(e) => {
                e.set_parent(parent)
            },
            Node::Body(e) => {
                e.set_parent(parent)
            },
            Node::Joint(e) => {
                e.set_parent(parent)
            },
            Node::Defaults(e) => {
                e.set_parent(parent)
            },
            Node::Assets(e) => {
                e.set_parent(parent)
            },
            Node::Mesh(e) => {
                e.set_parent(parent)
            },
            Node::Texture(e) => {
                e.set_parent(parent)
            },
            Node::Material(e) => {
                e.set_parent(parent)
            }
        }
    }

    fn get_class(&self) -> String {
        match self {
            Node::Geom(e) => {
                e.get_class()
            },
            Node::WorldBody(e) => {
                e.get_class()
            },
            Node::Body(e) => {
                e.get_class()
            },
            Node::Joint(e) => {
                e.get_class()
            },
            Node::Defaults(e) => {
                e.get_class()
            },
            Node::Assets(e) => {
                e.get_class()
            },
            Node::Mesh(e) => {
                e.get_class()
            },
            Node::Texture(e) => {
                e.get_class()
            },
            Node::Material(e) => {
                e.get_class()
            }
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        match self {
            Node::Geom(e) => {
                e.get_children()
            },
            Node::WorldBody(e) => {
                e.get_children()
            },
            Node::Body(e) => {
                e.get_children()
            },
            Node::Joint(e) => {
                e.get_children()
            },
            Node::Defaults(e) => {
                e.get_children()
            },
            Node::Assets(e) => {
                e.get_children()
            },
            Node::Mesh(e) => {
                e.get_children()
            },
            Node::Texture(e) => {
                e.get_children()
            },
            Node::Material(e) => {
                e.get_children()
            }
        }
    }

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
        match self {
            Node::Geom(e) => {
                e.apply_assets(asset_manager)
            },
            Node::WorldBody(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Body(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Joint(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Defaults(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Assets(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Mesh(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Texture(e) => {
                e.apply_assets(asset_manager)
            },
            Node::Material(e) => {
                e.apply_assets(asset_manager)
            }
        }
    }

    fn get_name(&self) -> String {
        match self {
            Node::Geom(e) => {
                e.get_name()
            },
            Node::WorldBody(e) => {
                e.get_name()
            },
            Node::Body(e) => {
                e.get_name()
            },
            Node::Joint(e) => {
                e.get_name()
            },
            Node::Defaults(e) => {
                e.get_name()
            },
            Node::Assets(e) => {
                e.get_name()
            },
            Node::Mesh(e) => {
                e.get_name()
            },
            Node::Texture(e) => {
                e.get_name()
            },
            Node::Material(e) => {
                e.get_name()
            }
        }
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        match self {
            Node::Geom(e) => {
                e.get_set_attributes_list()
            },
            Node::WorldBody(e) => {
                e.get_set_attributes_list()
            },
            Node::Body(e) => {
                e.get_set_attributes_list()
            },
            Node::Joint(e) => {
                e.get_set_attributes_list()
            },
            Node::Defaults(e) => {
                e.get_set_attributes_list()
            },
            Node::Assets(e) => {
                e.get_set_attributes_list()
            },
            Node::Mesh(e) => {
                e.get_set_attributes_list()
            },
            Node::Texture(e) => {
                e.get_set_attributes_list()
            },
            Node::Material(e) => {
                e.get_set_attributes_list()
            }
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Geom(e) => {
                e.get_parent()
            },
            Node::WorldBody(e) => {
                e.get_parent()
            },
            Node::Body(e) => {
                e.get_parent()
            },
            Node::Joint(e) => {
                e.get_parent()
            },
            Node::Defaults(e) => {
                e.get_parent()
            },
            Node::Assets(e) => {
                e.get_parent()
            },
            Node::Mesh(e) => {
                e.get_parent()
            },
            Node::Texture(e) => {
                e.get_parent()
            },
            Node::Material(e) => {
                e.get_parent()
            }
        }
    }
}

#[derive(Debug)]
pub struct Defaults {
    children: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
    class: String,
    element_attrs: HashMap<String,HashMap<String,String>>,
    attrs_map: HashMap<String,String>
}

impl Defaults {
    pub fn new() -> Self {
        Defaults {
            children: Vec::new(),
            class: "main".to_string(),
            parent: Weak::new(),
            element_attrs: HashMap::new(),
            attrs_map: HashMap::new()
        }
    }

    pub fn set_element_attrs(&mut self, parent_element_attrs: &HashMap<String,HashMap<String,String>>) {
        /*This function assumes that all children have been added to this defaults node 
        This means that all the overriden attributes are already set */
        for (element,attrs) in parent_element_attrs {
            if self.element_attrs.contains_key(element) {
                let element_attrs = self.element_attrs.get_mut(element).
                    expect("Failed to get a from the  elements attribute map");
                for (key, value) in attrs {
                    if !element_attrs.contains_key(key) {
                        element_attrs.insert(key.clone(), value.clone());
                    }
                }
            } else {
                self.element_attrs.insert(element.clone(), attrs.clone());
            }
        }
    }

    pub fn apply_defaults(&mut self, body: Rc<RefCell<Node>>, 
            parent_element_attrs: Option<&HashMap<String,HashMap<String,String>>>) {
        //First push all the parent element attributes 
        if let Some(e) = parent_element_attrs {
            self.set_element_attrs(&e);
        }
        {
            //Set all relevant attributes from self if the body node matches
            // println!("Applying defaults. Checking self class {} & node {}'s class {}",
            //     self.class,b.get_node_name(),b.get_class());
            if body.borrow().get_class() == self.class {
                let mut b = body.borrow_mut();
                let attrs_o = self.element_attrs.get(&b.get_node_name());
                if let Some(attrs) = attrs_o {
                    for (key,value) in attrs.iter() {
                        b.add_attr(key.clone(), value.clone());
                    }
                }
            } else {
                //Check all other children defaults to see if any default application needs to be done
                for child in &self.children {
                    match &mut (*child.borrow_mut()) {
                        Node::Defaults(e) => {
                            e.apply_defaults(body.clone(),
                                Some(&self.element_attrs));
                        }
                        _ => {}
                    }
                }
            }
        }
        //Apply defaults to all children of body
        {
            let b = body.borrow_mut();
            let children = b.get_children();
            for child in children {
                self.apply_defaults(child, None)
            }
        }
    } 
}

impl NodeType for Defaults {
    fn add_attr(&mut self, key: String, value: String) -> bool {
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "class" => {
                    self.class = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        }  
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }

    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        if child.get_node_name() == "default" {
            self.children.insert(0,Rc::new(RefCell::new(child)));
            self.children.get(0).expect("Issue returning child").clone()
        } else {
            /*We don't take non-default children, they're only relevant anyway if 
            attributes are set for them, which we strip them of here */
            let attrs = child.get_set_attributes_list();
            self.element_attrs.insert(child.get_node_name(), attrs);
            Rc::new(RefCell::new(child))
        }
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let child_ref = child.borrow();
        if child_ref.get_node_name() == "default" {
            self.children.insert(0,child.clone());
        } else {
            /*We don't take non-default children, they're only relevant anyway if 
            attributes are set for them, which we strip them of here */
            let attrs = child_ref.get_set_attributes_list();
            self.element_attrs.insert(child_ref.get_node_name(), attrs);
        }
        child.clone()
    }

    fn get_node_name(&self) -> String {
        String::from_str("default").expect("Error deriving string")
    }

    fn get_name(&self) -> String {
        "".to_string()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.upgrade()
    }

    fn get_class(&self) -> String {
        if self.class == "" {
            "main".to_string()
        } else {
            String::from_str(&self.class).expect("Failed to create string")
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut children_c = Vec::new();
        for child in &self.children {
            children_c.push(child.clone());
        }
        children_c
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug)]
pub enum MeshType {
    ObjFile,
    Inline
}

#[derive(Debug)]
pub struct Mesh {
    name: String,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    scale: (f32,f32,f32),
    file: String,
    attrs_map: HashMap<String,String>,
    pub mesh_contents_type: MeshType
}

impl Mesh {
    pub fn new(name: String) -> Self {
        Mesh {
            name: name,
            vertices: Vec::new(),
            scale: (1.0,1.0,1.0),
            attrs_map: HashMap::new(),
            mesh_contents_type: MeshType::Inline,
            indices: Vec::new(),
            file: "".to_string(),
        }
    }

    pub fn getAllGeometries(&self) -> (Vec<f32>,Vec<u32>) {
        match self.mesh_contents_type {
            MeshType::Inline => {
                /* Pushing test vertices for now, 
                to get the orientation & translation applied vertices */
                let vertices_t = self.getVertices();
                let mut x: Vec<f64> = vertices_t.iter().map(|&val| val as f64).collect();
                let y = 
                    Self::generate_triangulated_convex_hull(x.as_mut_slice()).expect("Qhull error");
                let y_prime: Vec<f64> = y.0.iter().flat_map(|array| *array).collect();;
                let z: (Vec<f32>,Vec<u32>) = (y_prime.iter().map(|&val| val as f32).collect(),y.1);
                // println!("The sets are {} {}",z.0.len(),z.1.len());
                z
                // (self.getVertices(),self.getIndices())
            },
            MeshType::ObjFile => {
                (self.getVertices(),self.getIndices())
            }
        }
    }

    pub fn getVertices(&self) -> Vec<f32> {
        self.apply_transforms(self.vertices.clone())
    }

    pub fn getIndices(&self) -> Vec<u32> {
        self.indices.clone()
    }

    pub fn apply_transforms(&self,vertices: Vec<f32>) -> Vec<f32> {
        // println!("Applying scale to mesh {}: {} {} {} ", self.name,self.scale.0,self.scale.1,self.scale.2);
        let num_vertices = vertices.len() / 3;
        let rotation_s = Matrix3::new(
                self.scale.0, 0.0, 0.0,
                0.0, self.scale.1, 0.0,
                0.0, 0.0, self.scale.2,
        );
        let vertices_m = DMatrix::from_vec(3,num_vertices,vertices);
        let vertices_mr = rotation_s * vertices_m;
        // let vertices_mr = vertices_m;
        let vertices_r : Vec<f32> = Vec::from(vertices_mr.as_slice());
        vertices_r
    }
    
    /* Generates a triangulated convex hull from a 3D point cloud using the modern qhull API.
    /
    / This function uses the `Qh::builder()` pattern to compute the convex hull.
    /
    / # Arguments
    /
    / * `points`: A slice of 3D points, where each point is an array `[f64; 3]`.
    /
    / # Returns
    /
    / A `Result` containing a tuple with two elements:
    / 1. A `Vec<[f64; 3]>` representing the ordered array of unique vertices on the hull.
    / 2. A `Vec<usize>` representing the index array for drawing with triangle primitives.
    /
    / or a `qhull::Error` if the computation fails. */
    pub fn generate_triangulated_convex_hull(points: &mut [f64])
        -> Result<(Vec<[f64; 3]>, Vec<u32>), qhull::QhError>
    {
        let qh = Qh::builder().triangulate(true).
                            build(3,points)?;
        
        let mut indices = Vec::new();
        let mut hull_vertices = Vec::new();
        let mut i = 0;
        for facet in qh.facets() {
            if let Some(vertices) = facet.vertices() {
                let num_vertices = vertices.size(&qh);
                // println!("Vertices length is {}",num_vertices);
                assert!(num_vertices == 3,"Expected 3 vertices in the facet given triangulation");
                // The `vertex.index(&qh)` method is the new way to get this mapping.
                for vertex in vertices.iter() {
                    let coords = vertex.point().expect("Expected the triangle vertices");
                    hull_vertices.push([coords[0], coords[1], coords[2]]);
                }
                indices.push(i);        
                indices.push(i+1);
                indices.push(i+2);
                i+=3;
            }
        }
        Ok((hull_vertices, indices))
    }

    pub fn get_indices(&self) -> Vec<u32> {
        let mut indices = Vec::new();
        // indices.push(0);indices.push(1);indices.push(2);
        // indices.push(3);indices.push(4);indices.push(5);
        // indices.push(6);indices.push(7);indices.push(8);
        // indices.push(1);indices.push(2);indices.push(3);
        // indices.push(2);indices.push(3);indices.push(4);
        // indices.push(3);indices.push(4);indices.push(5);
        for i in 0..(self.vertices.len() / 3) {
            indices.push(i as u32);
            // indices.push(((i+1)%self.vertices.len()) as u32);
            // indices.push(((i+2)%self.vertices.len()) as u32);
        }
        indices
    }

    pub fn setup_with_obj_stream(&mut self,raw_obj: String) {
        self.mesh_contents_type = MeshType::ObjFile;
        let parsed_objs = Self::parse_obj_stream(raw_obj);
        self.vertices = parsed_objs.0;
        self.indices = parsed_objs.1;
    }

    fn parse_obj_stream(raw_obj: String) -> (Vec<f32>,Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut buff = std::io::Cursor::new(raw_obj);
        let models = 
            tobj::load_obj_buf(&mut buff, &tobj::LoadOptions {
                    triangulate: true,
                    ignore_lines: true,
                    ignore_points: true,
                    ..Default::default()
                },
                |_p| {
                     Ok((Vec::new(), ahash::AHashMap::<String,usize>::new()))
                }
            ).
            expect("Expected to be able to parse the obj file");
        for model in models.0 {
            let mesh = model.mesh;
            let mut positions = mesh.positions;
            let mut inds = mesh.indices;
            vertices.append(&mut positions);
            indices.append(&mut inds);
        }
        (vertices,indices)
    }

    // #[deprecated]
    // pub fn parse_obj_stream(raw_obj: String) {
    //     let mut vertices = Vec::new();
    //     let lines = raw_obj.split("\n").into_iter();
    //     for line in lines {
    //         if line.starts_with("v") {
    //             let contents: Vec<&str> = line.split(" ").collect();
    //             let x = contents.get(1).
    //                     expect("Was expecting the first index of the vertex").
    //                     parse::<f32>().unwrap();
    //             let y = contents.get(2).
    //                     expect("Was expecting the first index of the vertex").
    //                     parse::<f32>().unwrap();
    //             let z = contents.get(3).
    //                     expect("Was expecting the first index of the vertex").
    //                     parse::<f32>().unwrap();
    //             vertices.push(x);
    //             vertices.push(y);
    //             vertices.push(z);
    //         } else if line.starts_with("f") {
    //         }
    //     }
    // }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }
}

impl NodeType for Mesh {
    fn add_attr(&mut self, key: String, value: String) -> bool { 
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "file" => {
                    self.mesh_contents_type = MeshType::ObjFile;
                    self.file = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "vertex" => {
                    self.mesh_contents_type = MeshType::Inline;
                    let values = value.split_whitespace();
                    let values: Vec<&str> = values.collect();
                    for value in values {
                        self.vertices.push(value.parse::<f32>().unwrap())
                    }
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "scale" => {
                    let values = value.split_whitespace();
                    let values: Vec<&str> = values.collect();
                    self.scale.0 = values.get(0).expect("Expected position 0 for scale").parse::<f32>().unwrap();
                    self.scale.1 = values.get(1).expect("Expected position 1 for scale").parse::<f32>().unwrap();
                    self.scale.2 = values.get(2).expect("Expected position 2 for scale").parse::<f32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        }
        
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () { 
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        println!("Warning: Meshes can't have children. Skipping this.");
        Rc::new(RefCell::new(child))
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        // self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
        // self.parent.upgrade()
    }

    fn get_node_name(&self) -> String {
        String::from_str("mesh").expect("Error deriving string")
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        println!("Warning: Meshes can't have children. Skipping this.");
        child
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut children_c = Vec::new();
        children_c
    }

    fn get_name(&self) -> String {
        if self.attrs_map.contains_key("name") {
            self.name.clone()
        } else if self.attrs_map.contains_key("file") {
            //Warning: there can be a path here & in textures, remove that 
            self.attrs_map.get("file").expect("Expected file name in texture").
                split(".").nth(0).expect("Expected file name in texture").to_string()
        } else {
            "".to_string()
        }
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    attrs_map: HashMap<String,String>,
    pub file: String,
    gridlayout: String,
    pub gridsize: (u32,u32),
    pub t_type: TextureType,
    pub texture_image_dimensions: (u32,u32),
    pub rgb1: (f32,f32,f32),
    pub file_data: Vec<u8>,
    name: String
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            attrs_map: HashMap::new(),
            file: "".to_string(),
            texture_image_dimensions: (0,0),
            gridlayout: "".to_string(),
            gridsize: (1,1),
            t_type: TextureType::Cube,
            rgb1: (0.5,0.4,0.3),
            file_data: Vec::new(),
            name: "".to_string()
        }
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    pub fn to_load_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_cube_face_raw_image_data(&self,face: String) -> (u32,u32,Vec<u8>) {
        let full_width = self.texture_image_dimensions.0;
        let full_height = self.texture_image_dimensions.1;
        let width = full_width / self.gridsize.1;
        let height = full_height / self.gridsize.0;
        let mut raw_data = Vec::new();
        if let Some(face_index) = self.gridlayout.find(&face) {
            if (face_index as u32) < (self.gridsize.0 * self.gridsize.1 - 1) {
                let row = (face_index as u32) / self.gridsize.1;
                let column = (face_index as u32) % self.gridsize.1;
                let index_y = row * height;
                let index_x = column * width;
                let index_end_y = index_y + height;
                let index_end_x = index_x + width;
                for j in index_y..index_end_y {
                    for i in index_x..index_end_x {
                        let key = (3 * ((j as u32)*full_width + (i as u32))) as usize;
                        raw_data.push(self.file_data.get(key).
                            expect("Expected pixel data in raw texture").clone());
                        raw_data.push(self.file_data.get(key+1).
                            expect("Expected pixel data in raw texture").clone());
                        raw_data.push(self.file_data.get(key+2).
                            expect("Expected pixel data in raw texture").clone());
                    }
                }
                // println!("Returning raw texture face data: {:?} for face {}",(width,height,&raw_data),face);
            } else {
                for i in 0..(width*height) {
                    raw_data.push((self.rgb1.0 * 256.0) as u8);
                    raw_data.push((self.rgb1.1 * 256.0) as u8);
                    raw_data.push((self.rgb1.2 * 256.0) as u8);
                }
            }
        } else {
            for i in 0..(width*height) {
                    raw_data.push((self.rgb1.0 * 256.0) as u8);
                    raw_data.push((self.rgb1.1 * 256.0) as u8);
                    raw_data.push((self.rgb1.2 * 256.0) as u8);
            }
        }
        (width,height,raw_data)
    }
}

impl NodeType for Texture {
    fn add_attr(&mut self, key: String, value: String) -> bool {
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "file" => {
                    self.file = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "gridsize" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 2,"Expected 2 gridsize values only");
                    self.gridsize.0 = values.get(0).expect("Expected position 0 in gridsize").parse::<u32>().unwrap();
                    self.gridsize.1 = values.get(1).expect("Expected position 1 in gridsize").parse::<u32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "gridlayout" => {
                    self.gridlayout = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "type" => {
                    match value.as_str() {
                        "2d" => {self.t_type = TextureType::TwoD},
                        "skybox" => {self.t_type = TextureType::Skybox},
                        "Cube" => {self.t_type = TextureType::Cube},
                        _ => {}
                    }
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        } 
    }

    fn add_attrs(&mut self,attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        println!("Warning: Textures can't have children. Skipping add_child.");
        Rc::new(RefCell::new(child))
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        println!("Warning: Textures can't have children. Skipping add_child_ref.");
        child
    }

    fn get_node_name(&self) -> String {
        String::from_str("texture").expect("Error deriving string")
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        //Do nothing
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let children_c = Vec::new();
        children_c
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }

    fn get_name(&self) -> String {
        if self.attrs_map.contains_key("name") {
            self.name.clone()
        } else if self.attrs_map.contains_key("file") {
            self.attrs_map.get("file").expect("Expected file name in texture").
                split(".").nth(0).expect("Expected file name in texture").to_string()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Material {
    name: String,
    texture_name: String,
    rgba: (f32,f32,f32,f32),
    texture: Option<Rc<RefCell<Node>>>,
    attrs_map: HashMap<String,String>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            name: "".to_string(),
            texture_name: "".to_string(),
            texture: None,
            attrs_map: HashMap::new(),
            parent: None,
            rgba: (1.0,1.0,1.0,1.0)
        }
    }

    pub fn to_load_file(&self) -> Option<String> {
        if let Some(texture) = &self.texture {
            match &*texture.borrow() {
                Node::Texture(txt) => {
                    Some(txt.to_load_file())
                },
                _ => None
            }
        } else {
            None
        }
    }

    pub fn add_texture(&mut self, texture: Rc<RefCell<Node>>) {
        if let None = self.texture {
            self.texture = Some(texture);
        }
    }

    pub fn get_textures_data(&self) -> Option<Texture> {
        self.texture.as_ref().map_or(None, |e| {
            match &*e.borrow() {
                Node::Texture(t) => {
                    Some(t.clone())
                },
                _ => None
            }
        })
    }

    pub fn get_centered_vertices(vertices: &Vec<f32>) -> Vec<f32> {
        let mut temp_acc = (0.0,0.0,0.0);
        let mut re_centered_vertices = Vec::new();
        for i in 0..vertices.len()/3 {
            let x = vertices.get(i*3).expect("Did not get vertex index").clone();
            let y = vertices.get(i*3+1).expect("Did not get vertex index").clone();
            let z = vertices.get(i*3+2).expect("Did not get vertex index").clone();
            temp_acc.0 += x;
            temp_acc.1 += y;
            temp_acc.2 += z;
        }
        temp_acc = (temp_acc.0 * 3.0 / vertices.len() as f32,
                    temp_acc.1 * 3.0 / vertices.len() as f32,
                    temp_acc.2 * 3.0 / vertices.len() as f32);
        // println!("Centroid of vertice mesh being used for filling: {:?}",temp_acc);
        for i in 0..vertices.len()/3 {
            let x = vertices.get(i*3).expect("Did not get vertex index").clone();
            let y = vertices.get(i*3+1).expect("Did not get vertex index").clone();
            let z = vertices.get(i*3+2).expect("Did not get vertex index").clone();
            re_centered_vertices.push(x-temp_acc.0);
            re_centered_vertices.push(y-temp_acc.1);
            re_centered_vertices.push(z-temp_acc.2);
        }
        re_centered_vertices
    }

    pub fn get_vertices_filling(&self,vertices: &Vec<f32>) -> Option<Vec<VertexFilling>> {
        let mut filling: Vec<VertexFilling> = Vec::new();
        let vertices = Self::get_centered_vertices(vertices);
        if let Some(e) = self.texture.as_ref() {
            match &*e.borrow() {
                Node::Texture(t) => {
                    match t.t_type {
                        TextureType::Cube => {
                            for i in 0..vertices.len()/3 {
                                let x = vertices.get(i*3).expect("Did not get vertex index").clone();
                                let y = vertices.get(i*3+1).expect("Did not get vertex index").clone();
                                let z = vertices.get(i*3+2).expect("Did not get vertex index").clone();
                                filling.push(VertexFilling::CubeTextureCoordinates(x,y,z));
                            }
                            Some(filling)
                        },
                        _ => None
                    }
                },
                _ => None
            }
        } else if self.attrs_map.contains_key("rgba") {
            for i in 0..(vertices.len() / 3) {
                filling.push(VertexFilling::Color(self.rgba.0, self.rgba.1, self.rgba.2))
            }
            Some(filling)
        } else {
            None
        }
    }
}

impl NodeType for Material {
    fn add_attr(&mut self, key: String, value: String) -> bool {
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "rgba" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 4,"Expected 4 position values only");
                    self.rgba.0 = values.get(0).expect("Expected position 0").parse::<f32>().unwrap();
                    self.rgba.1 = values.get(1).expect("Expected position 1").parse::<f32>().unwrap();
                    self.rgba.2 = values.get(2).expect("Expected position 2").parse::<f32>().unwrap();
                    self.rgba.3 = values.get(3).expect("Expected position 3").parse::<f32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "texture" => {
                    //check if the mesh asset is available, if yes, attach it
                    //otherwise skip & just attach the name 
                    self.texture_name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        } 
    }

    fn add_attrs(&mut self,attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        println!("Warning: Material can't have children. Skipping add_child.");
        Rc::new(RefCell::new(child))
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        println!("Warning: Material can't have children. Skipping add_child_ref.");
        child
    }

    fn get_node_name(&self) -> String {
        String::from_str("material").expect("Error deriving string")
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = Some(parent);
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let children_c = Vec::new();
        children_c
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub struct AssetsManager {
    children: Vec<Rc<RefCell<Node>>>
}

impl AssetsManager {
    pub fn new() -> Self {
        AssetsManager {
            children: Vec::new()
        }
    }

    pub fn get_mesh(&self, name: String) -> Option<Rc<RefCell<Node>>> {
        let mut mesh_iq = None;
        for child in &self.children {
            let child_name = child.borrow().get_name();
            let child_node_name = child.borrow().get_node_name();
            // println!("Searched meshes, got: {} was searching for {}", child_name, name);
            if child_name == name && child_node_name == "mesh" {
               mesh_iq = Some(child.clone());
            }
        }
        mesh_iq
    }

    fn process_materials(&self) {
        let mut texture_map: HashMap<String,Rc<RefCell<Node>>> = HashMap::new();
        let mut material_map: HashMap<String,Rc<RefCell<Node>>> = HashMap::new();
        for child in &self.children {
            let child_name = child.borrow().get_name();
            let child_node_name = child.borrow().get_node_name();
            if child_node_name == "material" {
               material_map.insert(child_name, child.clone());
            } else if child_node_name == "texture" {
               texture_map.insert(child_name, child.clone());
            } 
        }
        for material in material_map.iter() {
            if texture_map.contains_key(material.0) {
                match &mut *material.1.borrow_mut() {
                    Node::Material(ref mut m) => {
                        m.add_texture(texture_map.get(material.0).
                            expect("Expected texture in the map when processing materials").
                            clone());
                    },
                    _ => ()
                }
            }
        }
    }

    pub fn get_material(&self, name: String) -> Option<Rc<RefCell<Node>>> {
        //process materials to link textures with them first
        self.process_materials();
        let mut material_iq = None;
        for child in &self.children {
            let child_name = child.borrow().get_name();
            let child_node_name = child.borrow().get_node_name();
            if child_name == name && child_node_name == "material" {
               material_iq = Some(child.clone());
            }
        }
        material_iq
    }

    pub fn to_load_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        for child in &self.children {
            match &*child.borrow() {
                Node::Texture(t) => {
                    files.push(t.file.clone());
                },
                Node::Mesh(m) => {
                    match &m.mesh_contents_type {
                        MeshType::ObjFile => {
                            files.push(m.file.clone());
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }
        files
    }

    pub fn load_obj_meshes(&self,files: HashMap<String,String>) {
        for child in &self.children {
            match &mut *child.borrow_mut() {
                Node::Mesh(m) => {
                    match m.mesh_contents_type {
                        MeshType::ObjFile => {
                            if let Some(file_raw) = files.get(&m.get_file()) {
                                println!("Loading obj file in asset manager to mesh {:?} {:?}",
                                    m.file,m.file);
                                //WARNING: Performance hit, this might be an expensive clone:
                                m.setup_with_obj_stream(file_raw.clone());

                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }
    }

    pub fn load_dimensions(&mut self, files: HashMap<String,(u32,u32)>) {
        for child in &self.children {
            match &mut *child.borrow_mut() {
                Node::Texture(t) => {
                    let dimensions = files.get(&t.file);
                    println!("Loading file in asset manager {:?} {:?}",t.file,files.contains_key(&t.file));
                    if let Some(dimensions) = dimensions {
                        t.texture_image_dimensions = (dimensions.0,dimensions.1);
                    }
                },
                _ => ()
            }
        }
    }

    pub fn get_textures_data(&self) -> Vec<Texture> {
        let mut textures = Vec::new();
        for child in &self.children {
            match &*child.borrow() {
                Node::Texture(m) => {
                    textures.push(m.clone());
                },
                _ => ()
            }
        }
        textures
    }
}

impl NodeType for AssetsManager {
    fn add_attr(&mut self, key: String, value: String) -> bool {
        false
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        self.add_child_ref(Rc::new(RefCell::new(child)))
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        self.children.insert(0,child.clone());
        child
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        // self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
        // self.parent.upgrade()
    }

    fn get_node_name(&self) -> String {
        String::from_str("asset").expect("Error deriving string")
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut children_c = Vec::new();
        for child in &self.children {
            children_c.push(child.clone());
        }
        children_c
    }
    
    fn get_name(&self) -> String {
        "".to_string()
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        HashMap::new()
    }
}

#[derive(Debug)]
pub struct WorldBody {
    children: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
    name: String,
    childclass: String,
    class: String,
    attrs_map: HashMap<String,String>
}

impl WorldBody {
    pub fn default() -> Self {
        WorldBody {
            children: Vec::new(),
            name: String::new(),
            childclass: String::new(),
            class: String::new(),
            parent: Weak::new(),
            attrs_map: HashMap::new()
        }
    }

    /* Returns a dump of vertex & indices arrays holding the 
        all the geometries in the base frame of reference */ 
    pub fn getAllGeometries(&self) -> MeshCollection {
        let mut meshes = MeshCollection::new();
        for child in &self.children {
            match *child.borrow_mut() {
                Node::Body(ref bodyn) => {
                    meshes = meshes.merge(bodyn.getAllGeometries());
                },
                Node::Geom(ref geomn) => {
                    meshes = meshes.merge(geomn.getAllGeometries());
                },
                _ => ()
            }
        }
        meshes
    }
}

impl NodeType for WorldBody {
    fn add_attr(&mut self, key: String, value: String) -> bool {
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "class" => {
                    self.class = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "childclass" => {
                    self.childclass = value.clone();
                    for child in &self.children {
                        child.borrow_mut().add_attr(
                            key.clone(), 
                            value.clone());
                    }
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        }  
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, mut child: Node) -> Rc<RefCell<Node>> {
        //set childclass attribute of children here 
        if self.attrs_map.contains_key("childclass") {
            child.add_attr("class".to_string(),self.childclass.clone());
            child.add_attr("childclass".to_string(),self.childclass.clone());
        }
        self.children.insert(0,Rc::new(RefCell::new(child)));
        self.children.get(0).expect("Issue returning child").clone()
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        //set childclass attribute of children here 
        if self.attrs_map.contains_key("childclass") {
            child.borrow_mut().add_attr("class".to_string(),self.childclass.clone());
            child.borrow_mut().add_attr("childclass".to_string(),self.childclass.clone());
        }
        self.children.insert(0,child.clone());
        child
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.upgrade()
    }

    fn get_node_name(&self) -> String {
        String::from_str("worldbody").expect("Error deriving string")
    }

    fn get_class(&self) -> String {
        if self.class == "" {
            "main".to_string()
        } else {
            String::from_str(&self.class).expect("Failed to create string")
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut children_c = Vec::new();
        for child in &self.children {
            children_c.push(child.clone());
        }
        children_c
    }

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
        //just apply to children
        for child in &self.children {
            child.borrow_mut().apply_assets(asset_manager.clone());
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug)]
pub struct Body { 
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
    class: String,
    childclass: String,
    pos: (f32,f32,f32),
    orientation: Orientation,
    attrs_map: HashMap<String,String>,
    scale: (f32,f32,f32),
    added_rotations: Vec<Orientation>
}

impl Body { 
    pub fn default() -> Self {
        Body {
            name: String::new(),
            children: Vec::new(),
            class: String::new(),
            childclass: String::new(),
            pos: (0.0,0.0,0.0),
            scale: (1.0,1.0,1.0),
            parent: Weak::new(),
            orientation: Orientation::Euler(Euler{x:0.0,y:0.0,z:0.0}),
            attrs_map: HashMap::new(),
            added_rotations: Vec::new()
        }
    }

    fn get_rotation_params(orientation: &Orientation) -> (f32,f32,f32) {
        if let Orientation::Euler(e) = &orientation {
            (e.x * (std::f64::consts::PI / 180.0) as f32,
            e.y * (std::f64::consts::PI / 180.0) as f32,
            e.z * (std::f64::consts::PI / 180.0) as f32)
        } else {
            //TODO: Handle for other forms of rotation
            (0.0,0.0,0.0)
        }
    }

    pub fn apply_added_rotations(&mut self,rotation_params: (f32,f32,f32)) {
        self.added_rotations.push(Orientation::Euler(
            Euler{
                x: rotation_params.0,
                y: rotation_params.1,
                z: rotation_params.2
            }
        ));
    }

    fn apply_transforms(&self,vertices: Vec<f32>) -> Vec<f32> {
        let num_vertices = vertices.len() / 3;
        let mut added_rotation_m = Matrix3::new(
                    1.0,0.0,0.0,
                    0.0, 1.0, 0.0,
                    0.0, 0.0, 1.0
            );
        for added_r in &self.added_rotations {
            let rotation_params = Self::get_rotation_params(added_r);
            let applied_m = Matrix3::new(
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
            added_rotation_m = applied_m * added_rotation_m;
        }
        let rotation_params = Self::get_rotation_params(&self.orientation);
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
        let scale_m = Matrix3::new(
                self.scale.0, 0.0, 0.0,
                0.0, self.scale.1, 0.0,
                0.0, 0.0, self.scale.2,
        );
        let vertices_m = DMatrix::from_vec(3,num_vertices,vertices);
        let mut vertices_mr = added_rotation_m * rotation_m * scale_m * vertices_m;
        let mut translation = Vec::new();
        for _i in 0..num_vertices {
            translation.push(self.pos.0);
            translation.push(self.pos.1);
            translation.push(self.pos.2);
        }
        let translation_m = 
                DMatrix::from_vec(3,num_vertices,translation);
        vertices_mr += translation_m;
        let vertices_r : Vec<f32> = Vec::from(vertices_mr.as_slice());
        vertices_r
    }

    pub fn getAllGeometries(&self) -> MeshCollection {
        let mut meshes = MeshCollection::new();
        for child in &self.children {
            match *child.borrow_mut() {
                Node::Body(ref bodyn) => {
                    let child_meshes = bodyn.getAllGeometries();
                    for mesh in child_meshes.drawable_meshes.iter() {
                        meshes.add_drawable_mesh(
                            self.apply_transforms(mesh.vertices.clone()), 
                            mesh.indices.clone(), 
                            mesh.texture_data.clone(),
                        mesh.vertices_filling.clone());
                    }
                },
                Node::Geom(ref geomn) => {
                    let child_meshes = geomn.getAllGeometries();
                    for mesh in child_meshes.drawable_meshes.iter() {
                        meshes.add_drawable_mesh(
                            self.apply_transforms(mesh.vertices.clone()), 
                            mesh.indices.clone(), 
                            mesh.texture_data.clone(),
                        mesh.vertices_filling.clone());
                    }
                },
                _ => ()
            }
        }
        meshes
    }
}

impl NodeType for Body {
    fn add_attr(&mut self, key: String, value: String) -> bool { 
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "class" => {
                    self.class = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "childclass" => {
                    self.childclass = value.clone();
                    for child in &self.children {
                        child.borrow_mut().add_attr(
                            key.clone(), 
                            value.clone());
                    }
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "pos" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 3,"Expected 3 position values only");
                    self.pos.0 = values.get(0).expect("Expected position 0").parse::<f32>().unwrap();
                    self.pos.1 = values.get(1).expect("Expected position 1").parse::<f32>().unwrap();
                    self.pos.2 = values.get(2).expect("Expected position 2").parse::<f32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "euler" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 3,"Expected 3 position values only");
                    let mut orientation_ = Euler {x: 0.0, y: 0.0, z:0.0};
                    orientation_.x = values.get(0).expect("Expected position 0").parse::<f32>().unwrap();
                    orientation_.y = values.get(1).expect("Expected position 1").parse::<f32>().unwrap();
                    orientation_.z = values.get(2).expect("Expected position 2").parse::<f32>().unwrap();
                    self.orientation = Orientation::Euler(orientation_);
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "scale" => {
                    let values = value.split_whitespace();
                    let values: Vec<&str> = values.collect();
                    self.scale.0 = values.get(0).expect("Expected position 0 for scale").parse::<f32>().unwrap();
                    self.scale.1 = values.get(1).expect("Expected position 1 for scale").parse::<f32>().unwrap();
                    self.scale.2 = values.get(2).expect("Expected position 2 for scale").parse::<f32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        }         
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, mut child: Node) -> Rc<RefCell<Node>> {
        //set childclass attribute of children here 
        println!("Adding child node to body: {}",child.get_node_name());
        if self.attrs_map.contains_key("childclass") {
            child.add_attr("class".to_string(),self.childclass.clone());
            child.add_attr("childclass".to_string(),self.childclass.clone());
        }
        self.children.insert(0,Rc::new(RefCell::new(child)));
        self.children.get(0).expect("Issue returning child").clone()
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        //set childclass attribute of children here 
        println!("Adding child node tp body: {}",child.borrow_mut().get_node_name());
        if self.attrs_map.contains_key("childclass") {
            child.borrow_mut().add_attr("class".to_string(),self.childclass.clone());
            child.borrow_mut().add_attr("childclass".to_string(),self.childclass.clone());
        }
        self.children.insert(0,child.clone());
        child
    }

    fn get_node_name(&self) -> String {
        String::from_str("body").expect("Error deriving string")
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
    }

    fn get_class(&self) -> String {
        if self.class == "" {
            "main".to_string()
        } else {
            String::from_str(&self.class).expect("Failed to create string")
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut children_c = Vec::new();
        for child in &self.children {
            children_c.push(child.clone());
        }
        children_c
    }

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
        //just apply to children
        for child in &self.children {
            child.borrow_mut().apply_assets(asset_manager.clone());
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug)]
pub struct Joint { 
    joint_type: JointType,
    name: String,
    class: String,
    parent: Weak<RefCell<Node>>,
    attrs_map: HashMap<String,String>
}

impl Joint { 
    pub fn default() -> Self {
        Joint {
            joint_type: JointType::Free,
            name: String::new(),
            class: String::new(),
            parent: Weak::new(),
            attrs_map: HashMap::new()
        }
    }
}

impl NodeType for Joint {
    fn add_attr(&mut self, key: String, value: String) -> bool { 
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "class" => {
                    self.class = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        }  
        
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        println!("Warning: Joint can't have children. Skipping add_child.");
        Rc::new(RefCell::new(child))
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        println!("Warning: Joint can't have children. Skipping add_child_ref.");
        child
    }

    fn get_node_name(&self) -> String {
        String::from_str("joint").expect("Error deriving string")
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = parent;
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
    }

    fn get_class(&self) -> String {
        if self.class == "" {
            "main".to_string()
        } else {
            String::from_str(&self.class).expect("Failed to create string")
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        Vec::new()
    }

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug)]
pub struct Geom {
    geom_t: GeomType,
    name: String,
    class: String,
    childclass: String,
    // children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
    orientation: Orientation,
    mesh: Option<Rc<RefCell<Node>>>,
    material: Option<Rc<RefCell<Node>>>,
    mesh_name: String,
    material_name: String,
    mass: f32,
    density: f32,
    size: [f32;3],
    pos: (f32,f32,f32),
    attrs_map: HashMap<String,String>
}

impl Geom { 
    pub fn default() -> Self {
        Geom {
            geom_t: GeomType::Box,
            name: String::new(),
            class: String::new(),
            childclass: String::new(),
            parent: None,
            // children: Vec::new(),
            size: [0.0,0.0,0.0],
            pos: (0.0,0.0,0.0),
            mesh: None,
            material: None,
            mesh_name: "".to_string(),
            material_name: "".to_string(),
            mass: 0.0,
            density: 0.0,
            orientation: Orientation::Euler(Euler{x:0.0,y:0.0,z:0.0}),
            attrs_map: HashMap::new()
        }
    }

    fn get_rotation_params(&self) -> (f32,f32,f32) {
        if let Orientation::Euler(e) = &self.orientation {
            (e.x * (std::f64::consts::PI / 180.0) as f32,
            e.y * (std::f64::consts::PI / 180.0) as f32,
            e.z * (std::f64::consts::PI / 180.0) as f32)
        } else {
            //TODO: Handle for other forms of rotation
            (0.0,0.0,0.0)
        }
    }

    fn apply_transforms(&self,vertices: Vec<f32>) -> Vec<f32> {
        let num_vertices = vertices.len() / 3;
        let rotation_params = self.get_rotation_params();
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
        let vertices_m = DMatrix::from_vec(3,num_vertices,vertices);
        let mut vertices_mr = rotation_m * vertices_m;
        let mut translation = Vec::new();
        for _i in 0..num_vertices {
            translation.push(self.pos.0);
            translation.push(self.pos.1);
            translation.push(self.pos.2);
        }
        // println!("Translation matrix: {:?}; rotation matrix: {:?}",(self.pos.0,self.pos.1,self.pos.2),rotation_m);
        let translation_m = 
                DMatrix::from_vec(3,num_vertices,translation);
        vertices_mr += translation_m;
        let vertices_r : Vec<f32> = Vec::from(vertices_mr.as_slice());
        vertices_r
    }

    pub fn getAllGeometries(&self) -> MeshCollection {
        let mut meshes = MeshCollection::new();
        match &self.geom_t {
            GeomType::Mesh => {
                match &*(self.mesh.as_ref().expect("Was expecting mesh to be available").borrow()) {
                    Node::Mesh(y) => {
                        let mesh_geoms = y.getAllGeometries();
                        let mesh_texture_data = self.get_textures_data();
                        let mesh_vertices_filling = 
                            self.get_vertices_filling(&mesh_geoms.0);
                        // println!("Returning geom geometries: {} {} {:?} {}",
                        //     mesh_geoms.0.len(),
                        //     mesh_geoms.1.len(),
                        // mesh_texture_data.as_ref().expect("Expected texture data"),
                        // mesh_vertices_filling.as_ref().expect("Expected vertices filling").len());
                        meshes.add_drawable_mesh(
                            self.apply_transforms(mesh_geoms.0),
                            mesh_geoms.1,
                            mesh_texture_data,
                            mesh_vertices_filling);
                    },
                    _ => {
                        println!("Expected mesh node to be available");
                    }
                }
            },
            GeomType::Sphere => {
                let vertices = self.getSphereVertices();
                let mesh_texture_data = self.get_textures_data();
                //TO FIX: Get filling before applying transforms!!
                let mesh_vertices_filling = 
                    self.get_vertices_filling(&vertices);
                meshes.add_drawable_mesh(
                            self.apply_transforms(vertices),
                            Self::getSphereIndices(),
                            mesh_texture_data,
                        mesh_vertices_filling);
            },
            GeomType::Box => {
                let vertices = self.getBoxVertices();
                let mesh_texture_data = self.get_textures_data();
                //TO FIX: Get filling before applying transforms!!
                let mesh_vertices_filling = 
                    self.get_vertices_filling(&vertices);
                meshes.add_drawable_mesh(
                            self.apply_transforms(vertices),
                            Self::getBoxIndices(),
                            mesh_texture_data,
                        mesh_vertices_filling);
            },
            _ => {
                println!("Unimplemented geometries for node");
            }
        }
        meshes
}

    fn getBoxVertices(&self) -> Vec<f32> {
        let mut box_vertices_r = Vec::new();
        box_vertices_r.push(self.pos.0 - self.size[0]); box_vertices_r.push(self.pos.1 - self.size[1]); box_vertices_r.push(self.pos.2 - self.size[2]); //0
        box_vertices_r.push(self.pos.0 - self.size[0]); box_vertices_r.push(self.pos.1 - self.size[1]); box_vertices_r.push(self.pos.2 + self.size[2]); //1
        box_vertices_r.push(self.pos.0 - self.size[0]); box_vertices_r.push(self.pos.1 + self.size[1]); box_vertices_r.push(self.pos.2 - self.size[2]); //2
        box_vertices_r.push(self.pos.0 - self.size[0]); box_vertices_r.push(self.pos.1 + self.size[1]); box_vertices_r.push(self.pos.2 + self.size[2]); //3
        box_vertices_r.push(self.pos.0 + self.size[0]); box_vertices_r.push(self.pos.1 - self.size[1]); box_vertices_r.push(self.pos.2 - self.size[2]); //4
        box_vertices_r.push(self.pos.0 + self.size[0]); box_vertices_r.push(self.pos.1 - self.size[1]); box_vertices_r.push(self.pos.2 + self.size[2]); //5
        box_vertices_r.push(self.pos.0 + self.size[0]); box_vertices_r.push(self.pos.1 + self.size[1]); box_vertices_r.push(self.pos.2 - self.size[2]); //6
        box_vertices_r.push(self.pos.0 + self.size[0]); box_vertices_r.push(self.pos.1 + self.size[1]); box_vertices_r.push(self.pos.2 + self.size[2]); //7
        box_vertices_r
    }

    fn getTestVertices() -> Vec<f32> {
        let mut vertices: Vec<f32> = Vec::new();
        vertices.push(-0.5); vertices.push(-0.5); vertices.push(0.5); // 0
        vertices.push(-0.5); vertices.push(0.5); vertices.push(0.5); // 1
        vertices.push(0.5); vertices.push(-0.5); vertices.push(0.5); // 2
        vertices.push(0.5); vertices.push(0.5); vertices.push(0.5); // 3
        vertices.push(0.8); vertices.push(-0.5); vertices.push(-0.5); // 4
        vertices.push(0.8); vertices.push(0.5); vertices.push(-0.5); // 5
        vertices
    }

    fn getTestIndices() -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        indices.push(0);indices.push(1);indices.push(2);indices.push(1);indices.push(2);indices.push(3); 
        indices.push(2);indices.push(3);indices.push(4);indices.push(3);indices.push(4);indices.push(5); 
        indices
    }

    fn getBoxIndices() -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        indices.push(0);indices.push(1);indices.push(2);indices.push(1);indices.push(2);indices.push(3); // X-
        indices.push(4);indices.push(5);indices.push(6);indices.push(5);indices.push(6);indices.push(7); // X+
        indices.push(0);indices.push(1);indices.push(4);indices.push(1);indices.push(4);indices.push(5); // Y-
        indices.push(2);indices.push(3);indices.push(6);indices.push(3);indices.push(6);indices.push(7); // Y+
        indices.push(0);indices.push(2);indices.push(4);indices.push(2);indices.push(4);indices.push(6); // Z-
        indices.push(1);indices.push(3);indices.push(5);indices.push(3);indices.push(5);indices.push(7); // Z+
        indices
    }

    fn getXZRotationVertices(box_vertices_r: &Vec<f32>,angle: f32) -> Vec<f32> {
        assert!(box_vertices_r.len() == 24);
        let mut box_vertices_r_c = Vec::new();
        for i in 0..8 {
            let r = (box_vertices_r.get(i*3).expect("error ext vertices").powf(2.0) + box_vertices_r.get(i*3 + 2).expect("error ext vertices").powf(2.0)).sqrt();
            let theta = box_vertices_r.get(i*3 + 2).expect("error ext vertices").atan2(box_vertices_r.get(i*3).expect("error ext vertices").clone());
            box_vertices_r_c.push(r * (theta + angle).cos());
            box_vertices_r_c.push(box_vertices_r.get(i*3 + 1).expect("error ext vertices").clone());
            box_vertices_r_c.push(r * (theta + angle).sin());
        }
        box_vertices_r_c
    }

    fn getYZRotationVertices(box_vertices_r: &Vec<f32>,angle: f32) -> Vec<f32> {
        assert!(box_vertices_r.len() == 24);
        let mut box_vertices_r_c = Vec::new();
        for i in 0..8 {
            let r = (box_vertices_r.get(i*3 + 1).expect("error ext vertices").powf(2.0) + box_vertices_r.get(i*3 + 2).expect("error ext vertices").powf(2.0)).sqrt();
            let theta = box_vertices_r.get(i*3 + 2).expect("error ext vertices").atan2(box_vertices_r.get(i*3 + 1).expect("error ext vertices").clone());
            box_vertices_r_c.push(box_vertices_r.get(i*3).expect("error ext vertices").clone());
            box_vertices_r_c.push(r * (theta + angle).cos());
            box_vertices_r_c.push(r * (theta + angle).sin());
        }
        box_vertices_r_c
    }

    fn getXYRotationVertices(box_vertices_r: &Vec<f32>,angle: f32) -> Vec<f32> {
        assert!(box_vertices_r.len() == 24);
        let mut box_vertices_r_c = Vec::new();
        for i in 0..8 {
            let r = (box_vertices_r.get(i*3).expect("error ext vertices").powf(2.0) + box_vertices_r.get(i*3 + 1).expect("error ext vertices").powf(2.0)).sqrt();
            let theta = box_vertices_r.get(i*3 + 1).expect("error ext vertices").atan2(box_vertices_r.get(i*3).expect("error ext vertices").clone());
            box_vertices_r_c.push(r * (theta + angle).cos());
            box_vertices_r_c.push(r * (theta + angle).sin());
            box_vertices_r_c.push(box_vertices_r.get(i*3 + 2).expect("error ext vertices").clone());
        }
        box_vertices_r_c
    }

    fn getInitializedBoxVertices(size_x: f32, size_y: f32, size_z: f32) -> Vec<f32> {
        let mut box_vertices_r = Vec::new();
        let r = (size_x.powf(2.0) + size_y.powf(2.0) + size_z.powf(2.0)).sqrt() / 2.0;
        let angle_with_x_axis: f32 = std::f32::consts::FRAC_PI_4;
        let angle_with_y_axis: f32 = (1.0 / (2.0 as f32).sqrt()).atan();
        box_vertices_r.push(r * angle_with_y_axis.cos() * (3.0*angle_with_x_axis).cos()); box_vertices_r.push(r * (-angle_with_y_axis).sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (3.0*angle_with_x_axis).sin()); // 0
        box_vertices_r.push(r * angle_with_y_axis.cos() * (5.0*angle_with_x_axis).cos()); box_vertices_r.push(r * (-angle_with_y_axis).sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (5.0*angle_with_x_axis).sin()); // 1
        box_vertices_r.push(r * angle_with_y_axis.cos() * (3.0*angle_with_x_axis).cos()); box_vertices_r.push(r * angle_with_y_axis.sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (3.0*angle_with_x_axis).sin()); // 2
        box_vertices_r.push(r * angle_with_y_axis.cos() * (5.0*angle_with_x_axis).cos()); box_vertices_r.push(r * angle_with_y_axis.sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (5.0*angle_with_x_axis).sin()); // 3
        box_vertices_r.push(r * angle_with_y_axis.cos() * (angle_with_x_axis).cos()); box_vertices_r.push(r * (-angle_with_y_axis).sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (angle_with_x_axis).sin()); // 4
        box_vertices_r.push(r * angle_with_y_axis.cos() * (-angle_with_x_axis).cos()); box_vertices_r.push(r * (-angle_with_y_axis).sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (-angle_with_x_axis).sin()); // 5
        box_vertices_r.push(r * angle_with_y_axis.cos() * (angle_with_x_axis).cos()); box_vertices_r.push(r * angle_with_y_axis.sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (angle_with_x_axis).sin()); // 6
        box_vertices_r.push(r * angle_with_y_axis.cos() * (-angle_with_x_axis).cos()); box_vertices_r.push(r * angle_with_y_axis.sin()); box_vertices_r.push(r * angle_with_y_axis.cos() * (-angle_with_x_axis).sin()); // 7
        box_vertices_r
    }

    fn generateSphereMesh(subdivisions: u32, center: [f32; 3], radius: f32) -> Vec<f32> {
        let lat_segments = subdivisions;
        let lon_segments = subdivisions;
        
        // Vertex storage (flattened x,y,z coordinates)
        let mut vertices = Vec::new();
        // Generate vertices (flattened format)
        for i in 0..(lat_segments+1) {
            let theta = (std::f32::consts::PI * i as f32) / lat_segments as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for j in 0..lon_segments {
                let phi = 2.0 * std::f32::consts::PI * j as f32 / lon_segments as f32;
                //0,1..7
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                // Calculate vertex position with center offset
                let x = center[0] + radius * sin_theta * cos_phi;
                let y = center[1] + radius * cos_theta;
                let z = center[2] + radius * sin_theta * sin_phi;

                // Add flattened coordinates
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);

                // println!("Index & xyz {} {} {} {} {}",i,j,x,y,z);
            }
        }
        vertices
    }
    
    fn generateSphereIndices(subdivisions: u32) -> Vec<u32> {   
        let lat_segments = subdivisions;
        let lon_segments = subdivisions;
        // Index storage (triangle indices as f32)
        let mut indices = Vec::new();
        // Generate triangle indices (2 triangles per quad)
        for i in 0..lat_segments {
            for j in 0..lon_segments {
                let first = (i * lon_segments + j) as u32;
                let first_plus_1 = (i * lon_segments + ((j + 1) % lon_segments)) as u32;
                let second = (first + lon_segments) as u32;
                let second_plus_1 = (first_plus_1 + lon_segments) as u32;

                // First triangle (lower-left)
                indices.push(first);
                indices.push(first_plus_1);
                indices.push(second);

                // Second triangle (upper-right)
                indices.push(first_plus_1);
                indices.push(second);
                indices.push(second_plus_1);

                // println!("Sending indices for the first triangle {} {} {}",first,first+1,second);
                // println!("Sending indices for the second triangle {} {} {}",first+1,second,second+1);
            }
        }
        indices
    }

    fn getSphereVertices(&self) -> Vec<f32> {
        Self::generateSphereMesh(32,[self.pos.0,self.pos.1,self.pos.2],self.size[0])
    }
    
    fn getSphereIndices() -> Vec<u32> {
        Self::generateSphereIndices(32)
    }

    fn get_textures_data(&self) -> Option<Texture> {
        // println!("Getting textures data for geom class: {}",self.class);
        self.material.as_ref().map_or(None,|e| {
            // println!("Material available for geom class: {}",self.class);
            match &*e.borrow() {
                Node::Material(m) => {
                    m.get_textures_data()
                },
                _ => None
            }
        })
    }

    fn get_vertices_filling(&self,vertices: &Vec<f32>) -> Option<Vec<VertexFilling>> {
        // println!("Getting vertices filling for geom class: {} {:?}",self.class,vertices);
        self.material.as_ref().map_or(None,|e| {
            // println!("Material available for geom class: {}",self.class);
            match &*e.borrow() {
                Node::Material(m) => {
                    m.get_vertices_filling(vertices)
                },
                _ => None
            }
        })
    }
}

impl NodeType for Geom {
    fn add_attr(&mut self, key: String, value: String) -> bool { 
        if self.attrs_map.contains_key(&key) {
            false
        } else {
            match key.as_str() {
                "class" => {
                    println!("Setting class for geom node {}",value.clone());
                    self.class = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "pos" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 3,"Expected 3 position values only");
                    self.pos.0 = values.get(0).expect("Expected position 0").parse::<f32>().unwrap();
                    self.pos.1 = values.get(1).expect("Expected position 1").parse::<f32>().unwrap();
                    self.pos.2 = values.get(2).expect("Expected position 2").parse::<f32>().unwrap();
                    self.attrs_map.insert(key.clone(), value.clone());
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "name" => {
                    self.name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "material" => {
                    self.material_name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "euler" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    assert!(values.len() == 3,"Expected 3 position values only");
                    let mut orientation_ = Euler {x: 0.0, y: 0.0, z:0.0};
                    orientation_.x = values.get(0).expect("Expected position 0").parse::<f32>().unwrap();
                    orientation_.y = values.get(1).expect("Expected position 1").parse::<f32>().unwrap();
                    orientation_.z = values.get(2).expect("Expected position 2").parse::<f32>().unwrap();
                    self.orientation = Orientation::Euler(orientation_);
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "type" => {
                    self.geom_t = match value.as_str() {
                        "plane" => GeomType::Plane,
                        "hfield" => GeomType::Hfield,
                        "sphere" => GeomType::Sphere,
                        "capsule" => GeomType::Capsule,
                        "ellipsoid" => GeomType::Ellipsoid,
                        "box" => GeomType::Box,
                        "mesh" => GeomType::Mesh,
                        "sdf" => GeomType::Sdf,
                        _ => GeomType::Box,
                    };
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "mesh" => {
                    //check if the mesh asset is available, if yes, attach it
                    //otherwise skip & just attach the name 
                    self.mesh_name = value.clone();
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                "size" => {
                    let values = value.split(" ");
                    let values: Vec<&str> = values.collect();
                    for i in 0..values.len() {
                        self.size[i] = values.get(i).expect("Expected this position in size").parse::<f32>().unwrap();
                    }
                    self.attrs_map.insert(key.clone(), value.clone());
                    true
                },
                _ => false
            }
        } 
    }

    fn add_attrs(&mut self, attrs: Vec<(String,String)>) -> () {
        for (key, value) in attrs {
            self.add_attr(key, value);
        }
    }
    
    fn add_child(&mut self, child: Node) -> Rc<RefCell<Node>> {
        println!("Warning: Geoms can't have children. Skipping add_child.");
        Rc::new(RefCell::new(child))
    }

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        println!("Warning: Geoms can't have children. Skipping add_child_ref.");
        child
    }

    fn get_node_name(&self) -> String {
        String::from_str("geom").expect("Error deriving string")
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Node>>) -> () {
        self.parent = Some(parent);
    }
    
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        None
    }

    fn get_class(&self) -> String {
        if self.class == "" {
            "main".to_string()
        } else {
            String::from_str(&self.class).expect("Failed to create string")
        }
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        let children_c = Vec::new();
        children_c
    }

    fn apply_assets(&mut self,asset_manager: Rc<RefCell<Node>>) {
        match &*asset_manager.borrow() {
            Node::Assets(e) => {
                match self.geom_t {
                    GeomType::Mesh => {
                            println!("Applying mesh asset to geom node.");
                            let mesh = e.get_mesh(self.mesh_name.clone());
                            self.mesh = Some(mesh.
                                expect("Was expecting a mesh in assets when processing geoms.").
                                clone());
                    },
                    _ => {}
                }
                if self.attrs_map.contains_key("material") {
                    let material = 
                        e.get_material(self.material_name.clone()).
                        expect("Was expecting the material in assets when processing geoms").
                        clone();
                    self.material = Some(material);
                }
            },
            _ => {}
        }
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}