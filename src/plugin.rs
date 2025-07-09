use crate::node::Node;
use std::rc::{Rc};
use std::cell::RefCell;

pub struct PluginCapabilities {
    pub process_model_load: bool,
    pub process_sim_loop: bool,
    pub load_model: bool
}

impl PluginCapabilities {
    pub fn default() -> Self {
        Self {
            process_model_load: true,
            process_sim_loop: false,
            load_model: false
        }
    }
}

pub trait Plugin {
    fn process_sim_loop(&mut self,worldbody: Rc<RefCell<Node>>);
    fn process_model_load(&mut self,worldbody: Rc<RefCell<Node>>);
}

pub struct PluginManager {
    registered_plugins: Vec<(PluginCapabilities,Rc<RefCell<dyn Plugin>>)>
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registered_plugins: Vec::new()
        }
    }

    pub fn register_plugin(&mut self, plugin: Rc<RefCell<dyn Plugin>>, cap: PluginCapabilities) {
        self.registered_plugins.push((cap,plugin));
    }

    pub fn process_sim_loop(&self,worldbody: Rc<RefCell<Node>>) {
        for plugin in &self.registered_plugins {
            if plugin.0.process_sim_loop {
                plugin.1.borrow_mut().process_sim_loop(worldbody.clone());
            }
        }
    }

    pub fn process_model_load(&self,worldbody: Rc<RefCell<Node>>) {
        for plugin in &self.registered_plugins {
            if plugin.0.process_model_load {
                plugin.1.borrow_mut().process_model_load(worldbody.clone());
            }
        }
    }
} 