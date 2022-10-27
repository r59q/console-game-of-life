use std::{collections::HashMap};

use console_engine::KeyCode;
use super::{axis::Axis};

#[derive(Clone, Copy)]
pub struct AxialBinding {
    pub positive: KeyCode,
    pub negative: KeyCode
}

pub struct Bindings {
    axials: HashMap<Axis, Vec<AxialBinding>>
}

impl Bindings {
    pub fn new() -> Self {
        Self { axials: HashMap::new() }
    }

    pub fn get_axial_bindings(&mut self, axis: Axis) ->Vec<AxialBinding> {
        if !self.axials.contains_key(&axis) {
            Vec::<AxialBinding>::new();
        }
        return self.axials.entry(axis).or_default().to_vec();
    }

    pub fn bind_key_to_axis(&mut self, axis: Axis, positive_key: KeyCode, negative_key: KeyCode) {
        let vec = self.axials.entry(axis).or_insert(Vec::<AxialBinding>::new());
        vec.push(AxialBinding {positive: positive_key, negative: negative_key})
    }
}