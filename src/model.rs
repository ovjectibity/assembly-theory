use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::str::FromStr;
use egui::epaint::text;
use nalgebra::{ComplexField, DMatrix, Matrix3, Matrix3x1, SMatrix};
use qhull::{Qh};
use std::collections::HashMap;

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

#[derive(Debug)]
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
pub struct Mesh {
    name: String,
    vertices: Vec<f32>,
    scale: (f32,f32,f32),
    attrs_map: HashMap<String,String>
}

impl Mesh {
    pub fn new(name: String) -> Self {
        Mesh {
            name: name,
            vertices: Vec::new(),
            scale: (1.0,1.0,1.0),
            attrs_map: HashMap::new()
        }
    }

    pub fn getAllGeometries(&self) -> (Vec<f32>,Vec<u32>) {
        /* Pushing test vertices for now, 
        to get the orientation & translation applied vertices */
        // (self.getVertices(),self.getIndices())
        let vertices_t = self.getVertices();
        let mut x: Vec<f64> = vertices_t.iter().map(|&val| val as f64).collect();
        // geometries.push((self.getVertices(),self.getIndices()));
        let mut y = 
            Self::generate_triangulated_convex_hull(x.as_mut_slice()).expect("Qhull error");
        let y_prime: Vec<f64> = y.0.iter().flat_map(|array| *array).collect();
        // y.1 = vec![3,4,5,6,7,8];
        let z: (Vec<f32>,Vec<u32>) = (y_prime.iter().map(|&val| val as f32).collect(),y.1);
        // println!("The sets are {} {}",z.0.len(),z.1.len());
        z
    }

    pub fn getVertices(&self) -> Vec<f32> {
        self.apply_transforms(self.vertices.clone())
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
        // 1. Use the builder pattern to initialize and run Qhull.
        // The `build_from_iter` method takes an iterator of points and executes the algorithm.
        // The triangulation is implicitly handled for 3D convex hulls.
        let qh = Qh::builder()
            .build(3,points)?;

        // 2. Create the vertex array from the hull's vertices.
        // The `qh.vertices()` method returns an iterator over the unique vertices of the hull.
        // let hull_vertices: Vec<[f64; 3]> = qh.vertices()
        //     .map(|vertex| {
        //         let coords = vertex.point().expect("Expected the triangle vertices");
        //         // Assuming 3D points from the function signature
        //         [coords[0], coords[1], coords[2]]
        //     })
        //     .collect();
        
        let mut indices = Vec::new();
        let mut hull_vertices = Vec::new();
        let mut i = 0;
        for facet in qh.all_facets() {
            if let Some(vertices) = facet.vertices() {
                // The `vertex.index(&qh)` method is the new way to get this mapping.
                for vertex in vertices.iter() {
                    let coords = vertex.point().expect("Expected the triangle vertices");
                    hull_vertices.push([coords[0], coords[1], coords[2]]);
                    indices.push(i);
                    i+=1;
                }
            }
        }

        // 3. Create the index array by iterating through the triangular faces (simplices).
        // let mut indices = Vec::new();
        // for simplex in qh.simplices() {
        //     // A simplex is a face of the hull (a triangle in 3D).
        //     // We get the vertices belonging to this face.
        //     if let Some(simplex_vertices) = simplex.vertices() {
        //         // The `vertex.index(&qh)` method is the new way to get this mapping.
        //         for vertex in simplex_vertices.iter() {
        //             indices.push(vertex.index(&qh).expect("Was expecting index") as u32);
        //         }
        //     }
        // }
        // println!("Hull vertices & indices: {:#?} {:#?}", hull_vertices,indices);

        Ok((hull_vertices, indices))
    }

    pub fn getIndices(&self) -> Vec<u32> {
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
                "vertex" => {
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
        self.name.clone()
    }

    fn get_set_attributes_list(&self) -> HashMap<String,String> {
        self.attrs_map.clone()
    }
}

#[derive(Debug)]
pub struct Texture {
    name: String,
    attrs_map: HashMap<String,String>,
    file: Option<(bool,String)>,
    texture_data: Vec<u8>,
    gridlayout: String,
    gridsize: (u32,u32),
    t_type: TextureType
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            name: "".to_string(),
            attrs_map: HashMap::new(),
            file: None,
            texture_data: Vec::new(),
            gridlayout: "".to_string(),
            gridsize: (1,1),
            t_type: TextureType::Cube
        }
    }

    pub fn to_load_file(&self) -> Option<(bool,String)> {
        self.file.clone()
    }

    pub fn load_file(&mut self,file: Vec<u8>) {
        if let Some(file_details) = &self.file {
            if file_details.0 {
                self.texture_data = file;
                self.file = Some((true,file_details.1.clone()));
            }
        }
    }

    fn get_cube_texture_face_coordinates(&self,face: String) -> Option<(f32,f32)> {
        if let Some(face_index) = self.gridlayout.find(&face) {
            if face_index as u32 > (self.gridsize.0 * self.gridsize.1) - 1 {
                println!("Warning: Found cube texture face index gt number of grid cells");
                None
            } else {
                let row: u32 = (face_index as u32) / self.gridsize.1;
                let column: u32 = (face_index as u32) % self.gridsize.1;
                Some(((row as f32) * (1.0 / self.gridsize.0 as f32), (column as f32) * (1.0 / self.gridsize.1 as f32)))
            }
        } else {
            None
        }
    }

    fn get_cube_texture_coordinates(mut vertex: (f32,f32,f32)) -> (String,f32,f32) {
        /*WARNING: The orientation of the faces with the coordinate frame & 
          the orientation of texture frame may be wrong here */
        /*Normalise the coordinate first; a sphere encloses the cube; with radius sqrt(2)
          Sides of the cube being 2 units long*/
        let mag = (vertex.0.powi(2) + vertex.1.powi(2) + vertex.2.powi(2)).sqrt();
        vertex.0 = (2.0).sqrt() * vertex.0 / mag;
        vertex.1 = (2.0).sqrt() * vertex.1 / mag;
        vertex.2 = (2.0).sqrt() * vertex.2 / mag;
        //Check for all faces - Left face first; 
        //y coordinate should be in range of 1
        //z coordinate should be in range of 1
        //x coordinate should be < 0
        if vertex.1.abs() < 1.0 && vertex.2.abs() < 1.0 && vertex.0 < 0.0 {
            ("L".to_string(),(vertex.2+1.0)/2.0,(vertex.1+1.0)/2.0)
        } 
        // Right face then; 
        //y coordinate should be in range of 1
        //z coordinate should be in range of 1
        //x coordinate should be > 0
        else if vertex.1.abs() < 1.0 && vertex.2.abs() < 1.0 && vertex.0 > 0.0 {
            ("R".to_string(),(vertex.2+1.0)/2.0,(vertex.1+1.0)/2.0)
        }
        // Down face then; 
        //x coordinate should be in range of 1
        //z coordinate should be in range of 1
        //y coordinate should be < 0
        else if vertex.0.abs() < 1.0 && vertex.2.abs() < 1.0 && vertex.1 < 0.0 {
            ("D".to_string(),(vertex.0+1.0)/2.0,(vertex.2+1.0)/2.0)
        }
        // Up face then; 
        //x coordinate should be in range of 1
        //z coordinate should be in range of 1
        //y coordinate should be > 0
        else if vertex.0.abs() < 1.0 && vertex.2.abs() < 1.0 && vertex.1 > 0.0 {
            ("U".to_string(),(vertex.0+1.0)/2.0,(vertex.2+1.0)/2.0)
        }
        // Back face then; 
        //x coordinate should be in range of 1
        //y coordinate should be in range of 1
        //z coordinate should be < 0
        else if vertex.0.abs() < 1.0 && vertex.1.abs() < 1.0 && vertex.2 < 0.0 {
            ("B".to_string(),(vertex.0+1.0)/2.0,(vertex.1+1.0)/2.0)
        } 
        // Front face then; 
        //x coordinate should be in range of 1
        //y coordinate should be in range of 1
        //z coordinate should be > 0
        else if vertex.0.abs() < 1.0 && vertex.1.abs() < 1.0 && vertex.2 > 0.0 {
            ("F".to_string(),(vertex.0+1.0)/2.0,(vertex.1+1.0)/2.0)
        } 
        // Something else: 
        else {
            ("V".to_string(),0.0,0.0)
        } 
    }

    pub fn get_cube_texture_map_coordinates(&self,vertex: (f32,f32,f32)) -> Option<(f32,f32)> {
        let cube_texture_coordinates = Self::get_cube_texture_coordinates(vertex);
        let texture_face_coordinates = self.get_cube_texture_face_coordinates(cube_texture_coordinates.0);
        if let Some(tfc) = texture_face_coordinates {
            Some((tfc.0 + (cube_texture_coordinates.1 * (1.0 / self.gridsize.0 as f32)),
                tfc.1 + (cube_texture_coordinates.2 * (1.0 / self.gridsize.1 as f32))))
        } else {
            None
        }
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
                    self.file = Some((false,value.clone()));
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
            parent: None
        }
    }

    pub fn to_load_file(&self) -> Option<(bool,String)> {
        if let Some(texture) = &self.texture {
            match &*texture.borrow() {
                Node::Texture(txt) => {
                    txt.to_load_file()
                },
                _ => None
            }
        } else {
            None
        }
    }

    pub fn load_file(&self,file: Vec<u8>) {
        if let Some(texture) = &self.texture {
            match &mut *texture.borrow_mut() {
                Node::Texture(ref mut txt) => {
                    txt.load_file(file);
                },
                _ => ()
            }
        }
    }

    pub fn add_texture(&mut self, texture: Rc<RefCell<Node>>) {
        self.texture = Some(texture);
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
                Node::Material(t) => {
                    if let Some(file) = t.to_load_file() {
                        if file.0 {
                            files.push(file.1);
                        }
                    }
                },
                _ => ()
            }
        }
        files
    }

    pub fn load_files(&self, files: HashMap<String,Vec<u8>>) {
        for child in &self.children {
            match &mut *child.borrow_mut() {
                Node::Material(t) => {
                    if let Some(file) = t.to_load_file() {
                        if file.0 && files.contains_key(&file.1) {
                            t.load_file(files.get(&file.1).
                                expect("Expected to get the file in the filesmap.").clone())
                        }
                    }
                },
                _ => ()
            }
        }
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
        self.children.insert(0,Rc::new(RefCell::new(child)));
        self.children.get(0).expect("Issue returning child").clone()
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

    fn add_child_ref(&mut self, child: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        self.children.insert(0,child.clone());
        child
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
    pub fn getAllGeometries(&self) -> Vec<(Vec<f32>,Vec<u32>)> {
        let mut acc: Vec<(Vec<f32>,Vec<u32>)> = Vec::new();
        for child in &self.children {
            match *child.borrow_mut() {
                Node::Body(ref bodyn) => {
                    acc.append(&mut bodyn.getAllGeometries());
                },
                Node::Geom(ref geomn) => {
                    acc.append(&mut geomn.getAllGeometries());
                },
                _ => ()
            }
        }
        return acc;
    }

    // pub fn handleBodyTransformations(&self, vertices: Vec<f32>) -> Vec<f32> {

    // }
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
    scale: (f32,f32,f32)
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
            attrs_map: HashMap::new()
        }
    }

    fn get_rotation_params(&self) -> (f32,f32,f32) {
        if let Orientation::Euler(e) = &self.orientation {
            (e.x,e.y,e.z)
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
        let rotation_s = Matrix3::new(
                self.scale.0, 0.0, 0.0,
                0.0, self.scale.1, 0.0,
                0.0, 0.0, self.scale.2,
        );
        let vertices_m = DMatrix::from_vec(3,num_vertices,vertices);
        let mut vertices_mr = rotation_m * rotation_s * vertices_m;
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

    pub fn getAllGeometries(&self) -> Vec<(Vec<f32>,Vec<u32>)> {
        let mut acc: Vec<(Vec<f32>,Vec<u32>)> = Vec::new();
        for child in &self.children {
            match *child.borrow_mut() {
                Node::Body(ref bodyn) => {
                    let geoms = bodyn.getAllGeometries();
                    let mut geoms_t = Vec::new();
                    for geom in geoms {
                        geoms_t.push((self.apply_transforms(geom.0),geom.1));
                    }
                    acc.append(&mut geoms_t);
                },
                Node::Geom(ref geomn) => {
                    let geoms = geomn.getAllGeometries();
                    let mut geoms_t = Vec::new();
                    for geom in geoms {
                        geoms_t.push((self.apply_transforms(geom.0),geom.1));
                    }
                    acc.append(&mut geoms_t);
                },
                _ => ()
            }
        }
        return acc;
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
            (e.x,e.y,e.z)
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

    pub fn getAllGeometries(&self) -> Vec<(Vec<f32>,Vec<u32>)> {
        let mut geometries = Vec::new();
        match &self.geom_t {
                GeomType::Mesh => {
                    match &*(self.mesh.as_ref().expect("Was expecting mesh to be available").borrow()) {
                        Node::Mesh(y) => {
                            let mesh_geoms = y.getAllGeometries();
                            geometries.push((self.apply_transforms(mesh_geoms.0),mesh_geoms.1));
                            geometries
                        },
                        _ => {
                            geometries.push((self.getVertices(),self.getIndices()));
                            geometries
                        }
                    }
                },
                _ => {
                    geometries.push((self.getVertices(),self.getIndices()));
                    geometries
                }
            }
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

    fn getVertices(&self) -> Vec<f32> {
        self.apply_transforms(
    match &self.geom_t {
                GeomType::Sphere => {
                    self.getSphereVertices()
                },
                GeomType::Box => {
                    self.getBoxVertices()
                },
                _ => Vec::new()
            })
    }

    fn getIndices(&self) -> Vec<u32> {
        match &self.geom_t {
            GeomType::Sphere => {
                Self::getSphereIndices()
            },
            GeomType::Box => {
                Self::getBoxIndices()
            },
            _ => Vec::new()
        }
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
        match self.geom_t {
           GeomType::Mesh => {
                println!("Applying mesh asset to geom node.");
                match &*asset_manager.borrow() {
                    Node::Assets(e) => {
                        let mesh = e.get_mesh(self.mesh_name.clone());
                        self.mesh = Some(mesh.expect("Was expecting a mesh in assets.").clone());
                    },
                    _ => {}
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