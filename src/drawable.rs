use crate::node::{NodeType, Texture};

#[derive(Clone,Debug)]
pub struct MeshCollection {
    pub drawable_meshes: Vec<DrawableMesh>
}

impl MeshCollection {
    pub fn new() -> Self {
        MeshCollection {
            drawable_meshes: Vec::new()
        }
    }

    pub fn merge(mut self, mut other: Self) -> Self {
        self.drawable_meshes.append(&mut other.drawable_meshes);
        self
    }

    pub fn add_drawable_mesh(&mut self,
        drawable_mesh: Vec<f32>,
        indices: Vec<u32>,
        texture_data: Option<Texture>,
        vertices_filling: Option<Vec<VertexFilling>>) {
        if let Some(filling) = &vertices_filling {
            assert!(filling.len() == (drawable_mesh.len() / 3),
                "Expected vertices filling to be equal to number of vertices provided.");
        }
        self.drawable_meshes.push(DrawableMesh {
            vertices: drawable_mesh,
            indices: indices,
            draw_with_indices: true,
            vertices_filling: vertices_filling,
            texture_data: texture_data,
        });
    }

    pub fn get_full_interleaved_vertices(&self) -> Vec<f32> {
        self.drawable_meshes.iter().flat_map(|mesh| {
            mesh.get_full_interleaved_vertices()
        }).collect()
    }

    pub fn get_all_indices(&self) -> Vec<u32> {
        let mut index_c = Vec::new();
        let mut acc = 0;
        for mesh in &self.drawable_meshes {
            // println!("First set of indices are: {:?}",mesh.indices.len());
            let mut c = mesh.indices.clone().iter().map(|e| {
                e + acc
            }).collect();
            // println!("Returning set of indices are: {:?}",c);
            index_c.append(&mut c);
            acc += mesh.get_num_vertices();
        }
        index_c
    }

    pub fn get_num_vertices(&self) -> u32 {
        self.drawable_meshes.iter().map(|mesh| {
            // println!("Number of mesh set is {}",mesh.get_num_vertices());
            mesh.get_num_vertices()
        }).sum()
    }

    pub fn get_draw_map(&self) -> Vec<(u32,Option<String>)> {
        self.drawable_meshes.iter().map(|mesh| {
            (mesh.indices.len() as u32,
            mesh.texture_data.as_ref().map_or(None, |tex| {
                Some(tex.get_name())
            }))
        }).collect()
    }
}

#[derive(Clone,Debug)]
pub enum VertexFilling {
    Color(f32,f32,f32),
    TwoDTextureCoordinates(f32,f32),
    CubeTextureCoordinates(f32,f32,f32)
}

#[derive(Clone,Debug)]
pub struct DrawableMesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub draw_with_indices: bool,
    pub vertices_filling: Option<Vec<VertexFilling>>,
    //Assumes only a single texture for a given mesh
    pub texture_data: Option<Texture> 
}

impl DrawableMesh {
    pub fn new() -> Self {
        DrawableMesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            draw_with_indices: true,  
            vertices_filling: None,
            texture_data: None,
        }
    }

    pub fn get_num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    pub fn get_num_vertices(&self) -> u32 {
        (self.vertices.len() / 3) as u32
    }

    pub fn get_full_interleaved_vertices(&self) -> Vec<f32> {
        let num_vertices = self.vertices.len() / 3;
        let mut interleaved_vertices = Vec::new();
        if let Some(vertices_filling) = &self.vertices_filling {
            // println!("Getting interleaved vertices with texture filling");
            for i in 0..num_vertices {
                interleaved_vertices.push(self.vertices.get(i*3).expect("Expected vertex").clone());
                interleaved_vertices.push(self.vertices.get(i*3+1).expect("Expected vertex").clone());
                interleaved_vertices.push(self.vertices.get(i*3+2).expect("Expected vertex").clone());
                let filling = vertices_filling.get(i).expect("Expected vertex filling");
                match filling {
                    VertexFilling::Color(r,g,b) => {
                        interleaved_vertices.push(r.clone());
                        interleaved_vertices.push(g.clone());
                        interleaved_vertices.push(b.clone());
                        interleaved_vertices.push(0.0);
                        interleaved_vertices.push(0.0);
                        interleaved_vertices.push(0.0);
                    },
                    VertexFilling::CubeTextureCoordinates(u,v,w) => {
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(u.clone());
                        interleaved_vertices.push(v.clone());
                        interleaved_vertices.push(w.clone());
                    },
                    VertexFilling::TwoDTextureCoordinates(u,v) => {
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(1.0);
                        interleaved_vertices.push(u.clone());
                        interleaved_vertices.push(v.clone());
                        interleaved_vertices.push(0.0);
                    }
                }
            }
        } else {
            // println!("Getting interleaved vertices without texture filling");
            for i in 0..num_vertices {
                interleaved_vertices.push(self.vertices.get(i*3).expect("").clone());
                interleaved_vertices.push(self.vertices.get(i*3+1).expect("").clone());
                interleaved_vertices.push(self.vertices.get(i*3+2).expect("").clone());
                interleaved_vertices.push(209.0 / 255.0);
                interleaved_vertices.push(180.0 / 255.0);
                interleaved_vertices.push(50.0 / 255.0);
                interleaved_vertices.push(0.0);
                interleaved_vertices.push(0.0);
                interleaved_vertices.push(0.0);
            }
        }
        interleaved_vertices
    }
}