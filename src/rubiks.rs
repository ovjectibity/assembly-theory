use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::plugin::Plugin;

use crate::node::{Node, NodeType};

#[derive(Clone)]
pub enum FaceColor {
    Red,
    Blue,
    Green, 
    White,
    Yellow,
    Orange
}

#[derive(Clone,Debug)]
pub enum CubeMove {
    LPlus,
    LMinus,
    RPlus,
    RMinus,
    UPlus,
    UMinus,
    DPlus,
    DMinus,
    FPlus,
    FMinus,
    BPlus,
    BMinus
}

#[derive(Clone)]
pub enum Cubelet {
    Center(FaceColor),
    Corner(FaceColor,FaceColor,FaceColor),
    Edge(FaceColor,FaceColor)
}

#[derive(Clone)]
pub enum SlottedCubelet {
    Center(u32),
    Corner(CornerSlot,u32),
    Edge(EdgeSlot,u32)
}

impl SlottedCubelet {
    fn get_index(&self) -> u32 {
        match self {
            SlottedCubelet::Center(i) => {
                i.clone()
            },
            SlottedCubelet::Corner(c,i) => {
                i.clone()
            },
            SlottedCubelet::Edge(c,i) => {
                i.clone()
            }
        }
    }

    fn set_index(&mut self,index: u32) {
        match self {
            SlottedCubelet::Center(i) => {
                *i = index;
            },
            SlottedCubelet::Corner(c,i) => {
                *i = index;
            },
            SlottedCubelet::Edge(c,i) => {
                *i = index;
            }
        }
    }
}

#[derive(Clone)]
pub enum EdgeSlot {
    LeftUp(u32,u32),
    LeftDown(u32,u32),
    LeftBack(u32,u32),
    LeftFront(u32,u32),
    RightUp(u32,u32),
    RightDown(u32,u32),
    RightBack(u32,u32),
    RightFront(u32,u32),
    UpBack(u32,u32),
    UpFront(u32,u32),
    DownBack(u32,u32),
    DownFront(u32,u32)
}

impl EdgeSlot {
    fn get_color_index(&self,primary: &str) -> (u32,u32) {
        match self {
            EdgeSlot::LeftUp(x,y) | 
            EdgeSlot::LeftDown(x,y) | 
            EdgeSlot::RightUp(x,y) | 
            EdgeSlot::RightDown(x,y) => {
                match primary {
                    "x" => (x.clone(),y.clone()),
                    "y" => (y.clone(),x.clone()),
                    // "z" => (z.clone(),x.clone()),
                    _ => (0,0)
                }
            },
            EdgeSlot::LeftBack(x,z) | 
            EdgeSlot::LeftFront(x,z) | 
            EdgeSlot::RightBack(x,z) | 
            EdgeSlot::RightFront(x,z) => {
                match primary {
                    "x" => (x.clone(),z.clone()),
                    // "y" => (y.clone(),x.clone()),
                    "z" => (z.clone(),x.clone()),
                    _ => (0,0)
                }
            },
            EdgeSlot::UpBack(y,z) | 
            EdgeSlot::UpFront(y,z) | 
            EdgeSlot::DownBack(y,z) | 
            EdgeSlot::DownFront(y,z) => {
                match primary {
                    // "x" => (x.clone(),z.clone()),
                    "y" => (y.clone(),z.clone()),
                    "z" => (z.clone(),y.clone()),
                    _ => (0,0)
                }
            }
        }
    }

    fn set_color_index(&mut self,primary: &str,indices: (u32,u32)) {
        match self {
            EdgeSlot::LeftUp(x,y) | 
            EdgeSlot::LeftDown(x,y) | 
            EdgeSlot::RightUp(x,y) | 
            EdgeSlot::RightDown(x,y) => {
                match primary {
                    "x" => {
                        *x = indices.0;
                        *y = indices.1;
                    },
                    "y" => {
                        *x = indices.1;
                        *y = indices.0;
                    }
                    // "z" => {}
                    _ => ()
                }
            },
            EdgeSlot::LeftBack(x,z) | 
            EdgeSlot::LeftFront(x,z) | 
            EdgeSlot::RightBack(x,z) | 
            EdgeSlot::RightFront(x,z) => {
                match primary {
                    "x" => {
                        *x = indices.0;
                        *z = indices.1;
                    },
                    // "y" => {}
                    "z" => {
                        *x = indices.1;
                        *z = indices.0;
                    }
                    _ => ()
                }
            },
            EdgeSlot::UpBack(y,z) | 
            EdgeSlot::UpFront(y,z) | 
            EdgeSlot::DownBack(y,z) | 
            EdgeSlot::DownFront(y,z) => {
                match primary {
                    // "x" => {},
                    "y" => {
                        *y = indices.0;
                        *z = indices.1;
                    },
                    "z" => {
                        *y = indices.1;
                        *z = indices.0;
                    }
                    _ => ()
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum CornerSlot {
    UpLeftFront(u32,u32,u32),
    UpRightFront(u32,u32,u32),
    UpLeftBack(u32,u32,u32),
    UpRightBack(u32,u32,u32),
    DownLeftFront(u32,u32,u32),
    DownRightFront(u32,u32,u32),
    DownLeftBack(u32,u32,u32),
    DownRightBack(u32,u32,u32)
}

impl CornerSlot {
    fn get_color_index(&self,primary: &str) -> (u32,u32,u32) {
        match self {
            CornerSlot::UpLeftFront(x,y,z) | 
            CornerSlot::UpRightFront(x,y,z) |
            CornerSlot::UpLeftBack(x,y,z) | 
            CornerSlot::UpRightBack(x,y,z) | 
            CornerSlot::DownLeftFront(x,y,z) | 
            CornerSlot::DownRightFront(x,y,z) | 
            CornerSlot::DownLeftBack(x,y,z) | 
            CornerSlot::DownRightBack(x,y,z)
             => {
                match primary {
                    "x" => (x.clone(),y.clone(),z.clone()),
                    "y" => (y.clone(),x.clone(),z.clone()),
                    "z" => (z.clone(),x.clone(),y.clone()),
                    _ => (0,0,0)
                }
            }
        }
    }

    pub fn set_color_index(&mut self,side: &str,indices: (u32,u32,u32)) {
        match self {
            CornerSlot::UpLeftFront(x,y,z) | 
            CornerSlot::UpRightFront(x,y,z) |
            CornerSlot::UpLeftBack(x,y,z) | 
            CornerSlot::UpRightBack(x,y,z) | 
            CornerSlot::DownLeftFront(x,y,z) | 
            CornerSlot::DownRightFront(x,y,z) | 
            CornerSlot::DownLeftBack(x,y,z) | 
            CornerSlot::DownRightBack(x,y,z) => {
                match side {
                    "x" => {
                        *x = indices.0;
                        *y = indices.1;
                        *z = indices.2;
                    },
                    "y" => {
                        *x = indices.1;
                        *y = indices.0;
                        *z = indices.2;
                    },
                    "z" => {
                        *x = indices.1;
                        *y = indices.2;
                        *z = indices.0;
                    },
                    _ => {}
                }
            }
        }
    }
}

pub struct RubiksCube {
    body: Option<Rc<RefCell<Node>>>, //Geometric representation
    cubelets: [Cubelet;26], //Cubelets description
    //Logical representation, 6 centers, 12 edge slots, 8 corner slots
    cube_slot_map: RefCell<HashMap<String,SlottedCubelet>> //([SlottedCubelet;6],[SlottedCubelet;12],[SlottedCubelet;8])
}

impl RubiksCube {
    //Assumption: 6 center cubelets, 8 corner cublets, 12 edge cubelets,
    //Possible moves: F+, F-, B+, B-, L+, L-, R+, R-, U+, U-, D+, D-
    //Select the cubelets index & select 9 for each move 
    // Apply transforms to the body as well as apply the internal state updates
    pub fn apply_move(&mut self, mv: CubeMove) {
        //Work on the body which has the list of all cubelets
        if let Some(b) = &self.body {
            {
                let children = b.borrow().get_children();
                let mut target_body = None;
                for child in children {
                    if child.borrow().get_name() == "core" {
                        target_body = Some(child.clone());
                    }
                }
                if let Some(tb) = target_body {
                    let a_indices: Vec<u32> = self.get_indices_for_move(mv.clone());
                    match *tb.borrow() {
                        Node::Body(ref t) => {
                            let children = t.get_children();
                            let mut index = 0;
                            for child in children.iter().rev() {
                                match *child.borrow_mut() {
                                    Node::Body(ref mut b) => {    
                                        // println!("Applying move to body node: {}",b.get_name());
                                        if a_indices.contains(&(index as u32)) {
                                            b.apply_added_rotations(Self::get_rotation_params(mv.clone()));
                                        }
                                        index += 1;
                                    },
                                    _ => ()
                                }
                            }
                        },
                        _ => ()
                    }
                }
            }
            self.update_rep(mv);
        }
    }

    fn get_rotation_params(mv: CubeMove) -> (f32,f32,f32) {
        match mv {
            CubeMove::LPlus => {
                (90.0,0.0,0.0)
            },
            CubeMove::LMinus => {
                (-90.0,0.0,0.0)
            },
            CubeMove::RPlus => {
                (-90.0,0.0,0.0)
            },
            CubeMove::RMinus => {
                (90.0,0.0,0.0)
            },
            CubeMove::UPlus => {
                (0.0,-90.0,0.0)
            },
            CubeMove::UMinus => {
                (0.0,90.0,0.0)
            },
            CubeMove::DPlus => {
                (0.0,90.0,0.0)
            },
            CubeMove::DMinus => {
                (0.0,-90.0,0.0)
            },
            CubeMove::FPlus => {
                (0.0,0.0,90.0)
            },
            CubeMove::FMinus => {
                (0.0,0.0,-90.0)
            },
            CubeMove::BPlus => {
                (0.0,0.0,-90.0)
            },
            CubeMove::BMinus => {
                (0.0,0.0,90.0)
            }
        }
    }

    fn update_rep(&mut self,mv: CubeMove) {
        let primary = match mv {
            CubeMove::LPlus | CubeMove::LMinus | CubeMove::RPlus | CubeMove::RMinus  => {             
                "x".to_string()
            },
            CubeMove::UPlus | CubeMove::UMinus | CubeMove::DPlus | CubeMove::DMinus => {             
                "z".to_string()
            },
            CubeMove::FPlus | CubeMove::FMinus | CubeMove::BPlus | CubeMove::BMinus => {             
                "y".to_string()           
            },
        };
        let mut slot_map = self.cube_slot_map.borrow_mut();
        //Get corner slots & perform necessary flips: 
        let slot_keys = Self::get_corner_slots(mv.clone());
        let mut last_slot_c: (u32,(u32,u32,u32)) = (0,(0,0,0));
        {
            let last_slot= slot_map.get(&slot_keys[3]).expect("Failure in getting cube slot");
            last_slot_c.0 = last_slot.get_index();
            if let SlottedCubelet::Corner(c, i) = last_slot {
                last_slot_c.1 = c.get_color_index(&primary);
            }
        }
        for index in 0..4 {
            let slot = slot_map.get_mut(&slot_keys[index]).expect("Failure in getting cube slot");
            let mut temp_slot_c: (u32,(u32,u32,u32)) = (0,(0,0,0));
            if let SlottedCubelet::Corner(c, i) = slot {
                temp_slot_c.0 = i.clone();
                temp_slot_c.1 = c.get_color_index(&primary);
                c.set_color_index(&primary, (last_slot_c.1.0,last_slot_c.1.2,last_slot_c.1.1));
            }
            slot.set_index(last_slot_c.0);
            last_slot_c = temp_slot_c;
        }
        //Get edge slots & perform necessary flips: 
        let slot_keys = Self::get_edge_slots(mv.clone());
        let mut last_slot_c: (u32,(u32,u32)) = (0,(0,0));
        {
            let last_slot= slot_map.get(&slot_keys[3]).expect("Failure in getting cube slot");
            last_slot_c.0 = last_slot.get_index();
            if let SlottedCubelet::Edge(c, i) = last_slot {
                last_slot_c.1 = c.get_color_index(&primary);
            }
        }
        for index in 0..4 {
            let slot = slot_map.get_mut(&slot_keys[index]).expect("Failure in getting cube slot");
            let mut temp_slot_c: (u32,(u32,u32)) = (0,(0,0));
            if let SlottedCubelet::Edge(c, i) = slot {
                temp_slot_c.0 = i.clone();
                temp_slot_c.1 = c.get_color_index(&primary);
                c.set_color_index(&primary, (last_slot_c.1.0,last_slot_c.1.1));
            }
            slot.set_index(last_slot_c.0);
            last_slot_c = temp_slot_c;
        }
    }

    fn get_edge_slots(mv: CubeMove) -> [String;4] {
        match mv {
            CubeMove::LPlus | CubeMove::LMinus  => {             
                [
                    "left-up".to_string(),
                    "left-front".to_string(),
                    "left-down".to_string(),
                    "left-back".to_string()
                ]
            },
            CubeMove::RPlus | CubeMove::RMinus => {             
                [
                    "right-up".to_string(),
                    "right-back".to_string(),
                    "right-down".to_string(),
                    "right-front".to_string()
                ]
            },
            CubeMove::UPlus | CubeMove::UMinus => {             
                [
                    "up-front".to_string(),
                    "left-up".to_string(),
                    "up-back".to_string(),
                    "right-up".to_string()
                ]
            },
            CubeMove::DPlus | CubeMove::DMinus => {                         
                [
                    "down-front".to_string(),
                    "right-down".to_string(),
                    "down-back".to_string(),
                    "left-down".to_string()
                ]
            },
            CubeMove::FPlus | CubeMove::FMinus => {                         
                [
                    "up-front".to_string(),
                    "right-front".to_string(),
                    "down-front".to_string(),
                    "left-front".to_string()
                ]
            },
            CubeMove::BPlus | CubeMove::BMinus => {                          
                [
                    "up-back".to_string(),
                    "left-back".to_string(),
                    "down-back".to_string(),
                    "right-back".to_string()
                ]
            }
        }
    }

    fn get_corner_slots(mv: CubeMove) -> [String;4] {
        match mv {
            CubeMove::LPlus | CubeMove::LMinus  => {             
                ["up-left-back".to_string(),
                "up-left-front".to_string(),
                "down-left-front".to_string(),
                "down-left-back".to_string()]
            },
            CubeMove::RPlus | CubeMove::RMinus => {            
                ["up-right-front".to_string(),
                "up-right-back".to_string(),
                "down-right-back".to_string(),
                "down-right-front".to_string()]
            },
            CubeMove::UPlus | CubeMove::UMinus => {              
                ["up-left-back".to_string(),
                "up-right-back".to_string(),
                "up-right-front".to_string(),
                "up-left-front".to_string()]
            },
            CubeMove::DPlus | CubeMove::DMinus => {             
                ["down-left-front".to_string(),
                "down-right-front".to_string(),
                "down-right-back".to_string(),
                "down-left-back".to_string()]
            },
            CubeMove::FPlus | CubeMove::FMinus => {             
                ["up-left-front".to_string(),
                "up-right-front".to_string(),
                "down-right-front".to_string(),
                "down-left-front".to_string()]
            },
            CubeMove::BPlus | CubeMove::BMinus => {              
                ["up-right-back".to_string(),
                "up-left-back".to_string(),
                "down-left-back".to_string(),
                "down-right-back".to_string()]
            }
        }
    }

    fn get_indices_for_move(&self,mv: CubeMove) -> Vec<u32> {
        let mut indices = Vec::new();
        let target = match mv {
            CubeMove::LPlus | CubeMove::LMinus  => {
                "left".to_string()
            },
            CubeMove::RPlus | CubeMove::RMinus => {
                "right".to_string()
            },
            CubeMove::UPlus | CubeMove::UMinus => {
                "up".to_string()
            },
            CubeMove::DPlus | CubeMove::DMinus => {
                "down".to_string()
            },
            CubeMove::FPlus | CubeMove::FMinus => {
                "front".to_string()
            },
            CubeMove::BPlus | CubeMove::BMinus => {
                "back".to_string()
            }
        };
        for key in self.cube_slot_map.borrow().keys() {
            if key.contains(&target) {
                // println!("Move index for cube {} for move {:?}",
                //     self.cube_slot_map.borrow().get(key).expect("Failure in getting cube slot").get_index(),
                //     mv);
                indices.push(self.cube_slot_map.borrow().get(key).expect("Failure in getting cube slot").get_index());
            }
        }
        indices
    }
}

impl RubiksCube {
    pub fn new() -> Self {
        let mut cubelets_map = HashMap::new();
        cubelets_map.insert("right".to_string(),SlottedCubelet::Center(0));
        cubelets_map.insert("left".to_string(),SlottedCubelet::Center(1));
        cubelets_map.insert("up".to_string(),SlottedCubelet::Center(2));
        cubelets_map.insert("down".to_string(),SlottedCubelet::Center(3));
        cubelets_map.insert("back".to_string(),SlottedCubelet::Center(4));
        cubelets_map.insert("front".to_string(),SlottedCubelet::Center(5));
        cubelets_map.insert("left-up".to_string(),SlottedCubelet::Edge(EdgeSlot::LeftUp(1,0), 10));
        cubelets_map.insert("left-down".to_string(),SlottedCubelet::Edge(EdgeSlot::LeftDown(1,0), 11));
        cubelets_map.insert("left-back".to_string(),SlottedCubelet::Edge(EdgeSlot::LeftBack(0,1), 12));
        cubelets_map.insert("left-front".to_string(),SlottedCubelet::Edge(EdgeSlot::LeftFront(0,1), 13));
        cubelets_map.insert("right-up".to_string(),SlottedCubelet::Edge(EdgeSlot::RightUp(0,1), 6));
        cubelets_map.insert("right-down".to_string(),SlottedCubelet::Edge(EdgeSlot::RightDown(1,0), 7));
        cubelets_map.insert("right-back".to_string(),SlottedCubelet::Edge(EdgeSlot::RightBack(0,1), 8));
        cubelets_map.insert("right-front".to_string(),SlottedCubelet::Edge(EdgeSlot::RightFront(0,1), 9));
        cubelets_map.insert("up-back".to_string(),SlottedCubelet::Edge(EdgeSlot::UpBack(0,1), 14));
        cubelets_map.insert("up-front".to_string(),SlottedCubelet::Edge(EdgeSlot::UpFront(0,1), 15));
        cubelets_map.insert("down-back".to_string(),SlottedCubelet::Edge(EdgeSlot::DownBack(0,1), 16));
        cubelets_map.insert("down-front".to_string(),SlottedCubelet::Edge(EdgeSlot::DownFront(0,1), 17));
        cubelets_map.insert("up-left-front".to_string(),SlottedCubelet::Corner(CornerSlot::UpLeftFront(1,0,2), 23));
        cubelets_map.insert("up-right-front".to_string(),SlottedCubelet::Corner(CornerSlot::UpRightFront(1,0,2), 19));
        cubelets_map.insert("up-left-back".to_string(),SlottedCubelet::Corner(CornerSlot::UpLeftBack(1,0,2), 22));
        cubelets_map.insert("up-right-back".to_string(),SlottedCubelet::Corner(CornerSlot::UpRightBack(1,0,2), 18));
        cubelets_map.insert("down-left-front".to_string(),SlottedCubelet::Corner(CornerSlot::DownLeftFront(1,0,2), 25));
        cubelets_map.insert("down-right-front".to_string(),SlottedCubelet::Corner(CornerSlot::DownRightFront(1,0,2), 21));
        cubelets_map.insert("down-left-back".to_string(),SlottedCubelet::Corner(CornerSlot::DownLeftBack(1,0,2), 24));
        cubelets_map.insert("down-right-back".to_string(),SlottedCubelet::Corner(CornerSlot::DownRightBack(1,0,2), 20));
        
        RubiksCube {
            body: None,
            cube_slot_map: RefCell::new(cubelets_map),
            cubelets: [
                Cubelet::Center(FaceColor::Red), //0
                Cubelet::Center(FaceColor::Orange), //1
                Cubelet::Center(FaceColor::Blue), //2
                Cubelet::Center(FaceColor::Green), //3
                Cubelet::Center(FaceColor::White), //4
                Cubelet::Center(FaceColor::Yellow), //5
                Cubelet::Edge(FaceColor::Blue,FaceColor::Red), //6
                Cubelet::Edge(FaceColor::Green,FaceColor::Red), //7
                Cubelet::Edge(FaceColor::Red,FaceColor::White), //8
                Cubelet::Edge(FaceColor::Red,FaceColor::Yellow), //9
                Cubelet::Edge(FaceColor::Blue,FaceColor::Orange), //10
                Cubelet::Edge(FaceColor::Green,FaceColor::Orange), //11
                Cubelet::Edge(FaceColor::Orange,FaceColor::White), //12
                Cubelet::Edge(FaceColor::Orange,FaceColor::Yellow), //13
                Cubelet::Edge(FaceColor::Blue,FaceColor::White), //14
                Cubelet::Edge(FaceColor::Blue,FaceColor::Yellow), //15
                Cubelet::Edge(FaceColor::Green,FaceColor::White), //16
                Cubelet::Edge(FaceColor::Green,FaceColor::Yellow), //17
                Cubelet::Corner(FaceColor::Blue,FaceColor::Red,FaceColor::White), //18
                Cubelet::Corner(FaceColor::Blue,FaceColor::Red,FaceColor::Yellow), //19
                Cubelet::Corner(FaceColor::Green,FaceColor::Red,FaceColor::White), //20
                Cubelet::Corner(FaceColor::Green,FaceColor::Red,FaceColor::Yellow), //21
                Cubelet::Corner(FaceColor::Blue,FaceColor::Orange,FaceColor::White), //22
                Cubelet::Corner(FaceColor::Blue,FaceColor::Orange,FaceColor::Yellow), //23
                Cubelet::Corner(FaceColor::Green,FaceColor::Orange,FaceColor::White), //24
                Cubelet::Corner(FaceColor::Green,FaceColor::Orange,FaceColor::Yellow) //25
            ]
        }
    }
}

impl Plugin for RubiksCube {
    fn process_sim_loop(&mut self,worldbody: Rc<RefCell<Node>>) {

    }

    fn process_model_load(&mut self,worldbody: Rc<RefCell<Node>>) {
        println!("Processing model load in Rubik's plugin");
        self.body = Some(worldbody);
        self.apply_move(CubeMove::FPlus);
        self.apply_move(CubeMove::BPlus);
        self.apply_move(CubeMove::UPlus);
        self.apply_move(CubeMove::RPlus);
        self.apply_move(CubeMove::DPlus);
        self.apply_move(CubeMove::LPlus);
        self.apply_move(CubeMove::RPlus);
        self.apply_move(CubeMove::RPlus);
        self.apply_move(CubeMove::DPlus);
        self.apply_move(CubeMove::FPlus);
    }
}