use eframe::egui;
use eframe::glow::{self};
use nalgebra::{Matrix4x1, Matrix3, Matrix4};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use crate::ViewProp;
use crate::node::{NodeType, TextureType};
use crate::node::Texture;
use crate::drawable::{MeshCollection};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct PaintsMan {
    program: Option<glow::Program>,
    vertex_array: Option<glow::VertexArray>,
    viewport_transform_matrix: Matrix3<f32>,
    viewport_translation_matrix: Matrix4x1<f32>,
    texture_ids_map: Option<HashMap<String,glow::NativeTexture>>,
    draw_map: Option<Vec<(u32,Option<String>)>>
}

impl PaintsMan {
    pub fn new() -> Self {
        Self {
            program: None,
            vertex_array: None,
            viewport_transform_matrix: Matrix3::<f32>::new(1.0,0.0,0.0,
                                                            0.0,1.0,0.0,
                                                        0.0,0.0,1.0),
            texture_ids_map: None,
            draw_map: None,
            viewport_translation_matrix: Matrix4x1::new(0.0,0.0,0.0,0.0)
        }
    }

    fn applyViewportTransform(&mut self, rotate_x: f32,rotate_y: f32, zoom_scale: f32) {
        // println!("Applying the viewport rotation {} {}",rotateX, rotateY);
        let rotation_params: (f32,f32,f32) = if rotate_y.abs() > rotate_x.abs() {
            (rotate_y,0.0,0.0)
        } else {
            (0.0,rotate_x,0.0)
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

    pub fn update_viewport(&mut self, translation: egui::Vec2, angle: egui::Vec2, zoom_scale: f32) -> &mut Self {
        self.applyViewportTransform(angle.x, angle.y, zoom_scale);       
        self.viewport_translation_matrix += 
            Matrix4x1::new(translation.x*0.01,translation.y*0.01,0.0,0.0);
        //applying the transform on the CPU, instead of via the vertex shader
        // vertices = Vec::from((self.viewport_transform_matrix * 
        //     DMatrix::from_vec(3,vertices.len() / 3,vertices)).as_slice());
        self
    }

    fn calculate_hash(data: &Vec<u8>) -> u64 {
        // Create a new DefaultHasher
        let mut hasher = DefaultHasher::new();

        // Write the bytes of the Vec<u8> into the hasher
        data.hash(&mut hasher);

        // Finalize the hash and return it as a u64
        hasher.finish()
    }

    pub fn refresh_draw(&mut self, 
            gl: &glow::Context, 
            meshes: MeshCollection,
            textures_data: Option<Vec<Texture>>) -> &Self {
        self.destroy(gl);
        use glow::HasContext as _;

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
                    layout(location = 1) in vec3 color;
                    layout(location = 2) in vec3 tex_coord_r;
                    uniform mat4 projection;
                    out vec4 v_color;
                    out vec3 tex_coord;
                    uniform mat3 viewport_transform;
                    uniform vec4 viewport_translate;
                    void main() {
                        v_color = vec4(color,0.75);
                        tex_coord = tex_coord_r;
                        gl_Position = viewport_translate + 
                            (projection * vec4(viewport_transform * verts, 1.0));
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec4 v_color;
                    in vec3 tex_coord;
                    out vec4 out_color;
                    // uniform sampler2D o_tex;
                    uniform samplerCube o_tex;
                    uniform uint use_texture;
                    // uniform uint is_cube_tex;

                    void main() {
                        // out_color = vec4(209.0 / 255.0,180.0 / 255.0,50.0 / 255.0,1.0);
                        // out_color = v_color;
                        if(use_texture == uint(1)) {
                            vec4 texture_m = texture(o_tex,tex_coord);
                            out_color = texture_m * v_color;
                            // out_color = vec4(209.0 / 255.0,180.0 / 255.0,50.0 / 255.0,1.0);
                            // if(texture_m.r > 0 || texture_m.g > 0 || texture_m.b > 0) {
                            //     out_color = vec4(209.0 / 255.0,180.0 / 255.0,50.0 / 255.0,1.0);
                            // } else {
                            //     out_color = vec4(0.5,0.5,0.5,0.5);
                            // }   
                        } 
                        else {
                            out_color =  v_color;
                        }
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

            let interleaved_vertices = meshes.get_full_interleaved_vertices();
            let all_indices = meshes.get_all_indices();
            let f32_size = std::mem::size_of::<f32>() as i32;
            // println!("Vertexes length is: {}",meshes.get_num_vertices());
            // println!("Interleaved vertexes length is: {}",interleaved_vertices.len());
            // println!("Elements length is: {}",all_indices.len());
            // println!("Interleaved vertexes is: {:?}",temp);
            // println!("All indices are: {:?}",temp_2);
            let vertices_flattened: Vec<f32> = interleaved_vertices;
            let indices_flattened: Vec<u32> = all_indices;
            let mut vertexes: Vec<u8> = bincode::encode_to_vec(vertices_flattened,
                bincode::config::standard().with_fixed_int_encoding()).expect("Failed to serialise via bincode.");
            let mut elements: Vec<u8> = bincode::encode_to_vec(indices_flattened,
                bincode::config::standard().with_fixed_int_encoding()).expect("Failed to serialise via bincode.");
            vertexes.drain(..8);
            elements.drain(..8);

            let vertex_buffer = gl.create_buffer().expect("Cannot create vertex buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertexes.as_slice(),glow::STATIC_DRAW);
            let elements_buffer = gl.create_buffer().expect("Cannot create elements buffer");
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER,Some(elements_buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, elements.as_slice(), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(0, 3, 
                        glow::FLOAT, false, 9*f32_size, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, 
                        glow::FLOAT, false, 9*f32_size, 3*f32_size);
            gl.enable_vertex_attrib_array(1); 
            gl.vertex_attrib_pointer_f32(2, 3, 
                        glow::FLOAT, false, 9*f32_size, 6*f32_size);
            gl.enable_vertex_attrib_array(2);
            gl.bind_vertex_array(None);

            // println!("Got this textures map {:?}",textures_ids_map);
            self.program = Some(program);
            self.vertex_array = Some(vertex_array);
            if let Some(tex) = textures_data {
                self.texture_ids_map = Some(Self::load_textures(gl, tex));
            }
            self.draw_map = Some(meshes.get_draw_map());
        }
        self
    }

    fn load_textures(
            gl: &glow::Context, 
            textures_data: Vec<Texture>) -> HashMap<String,glow::NativeTexture> {
        use glow::HasContext as _;
        unsafe {
            //Set texture data
            let mut textures_ids_map: HashMap<String,glow::NativeTexture> = HashMap::new();
            for texture in textures_data {
                // println!("Hash of texture file {} is {:?}",texture.get_name(),Self::calculate_hash(&texture.file_data));
                match texture.t_type {
                    TextureType::Cube => {
                        let tex: glow::NativeTexture = gl.create_texture().unwrap();
                        textures_ids_map.insert(texture.get_name(),tex);
                        gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(tex));

                        //For some reason the latter works rather than the former: 
                        // let faces = ["R","L","U","D","B","F"];
                        let faces = ["L","R","D","U","F","B"];
                        for i in 0..6 {
                            let face_data = 
                                texture.get_cube_face_raw_image_data(faces[i].to_string());
                            gl.tex_image_2d(
                                glow::TEXTURE_CUBE_MAP_POSITIVE_X + (i as u32), 
                                0, 
                                glow::RGB as i32, 
                                face_data.0 as i32, 
                                face_data.1 as i32, 
                                0, 
                                glow::RGB,
                                glow::UNSIGNED_BYTE, 
                                glow::PixelUnpackData::Slice(Some(face_data.2.as_slice())));
                        }
                        gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
                        gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
                        gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_R, glow::CLAMP_TO_EDGE as i32);
                        gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
                        gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

                        // gl.tex_image_2d(glow::TEXTURE_2D, 
                        //         0, glow::RGB as i32, 
                        //         texture.texture_dimensions.0 as i32, texture.texture_dimensions.1 as i32, 
                        //         0, glow::RGB, 
                        //         glow::UNSIGNED_BYTE, 
                        //         glow::PixelUnpackData::Slice(Some(&texture.texture_raw.as_slice())));
                        // println!("Binding the texture {} of dimensions {} {}",
                        //     texture.file,
                        //     texture.texture_dimensions.0,
                        //     texture.texture_dimensions.1);
                        // gl.generate_mipmap(glow::TEXTURE_2D);
                    }, 
                    _ => ()
                }
                gl.bind_texture(glow::TEXTURE_CUBE_MAP, None);
            }
            textures_ids_map
        }
    }

    pub fn paint(&self,view_props: Arc<Mutex<ViewProp>>, gl: &glow::Context) {
        use glow::HasContext as _;
        let u32_size = std::mem::size_of::<u32>() as i32;
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
            gl.uniform_4_f32_slice(
                gl.get_uniform_location(self.program.expect("No program available in PaintsMan instance"), "viewport_translate").as_ref(),
                self.viewport_translation_matrix.as_slice()
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

            let draw_map = 
                self.draw_map.as_ref().expect("Expected draw map to be available.");
            let texture_map = 
                self.texture_ids_map.as_ref().expect("Expected texture ids map to be available.");

            let mut index_offset = 0;
            for draw_comm in draw_map {
                gl.uniform_1_u32(
                    gl.get_uniform_location(self.program.
                        expect("No program available in PaintsMan instance"), 
                        "use_texture").as_ref(),
                        draw_comm.1.as_ref().map_or(0, |e| {
                            1
                        }));
                gl.bind_texture(glow::TEXTURE_CUBE_MAP, 
                    draw_comm.1.as_ref().map_or(None, |e| {
                    // println!("Binding texture {} {:?}",e,texture_map.get(e).expect("Expected texture binding to be available.").clone());
                    Some(texture_map.get(e).expect("Expected texture binding to be available.").clone())
                }));
                // println!("Drawing {} indices at offset: {}",draw_comm.0.clone(),index_offset);
                gl.draw_elements(glow::TRIANGLES, draw_comm.0.clone() as i32, 
                    glow::UNSIGNED_INT, index_offset * u32_size);
                index_offset += draw_comm.0.clone() as i32;
            }
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
        }
    }

    pub fn destroy(&mut self, gl: &glow::Context) {
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
}