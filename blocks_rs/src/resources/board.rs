use bevy::prelude::*;
#[derive(Resource)]
pub struct Board {
    height: usize,
    width: usize,
    matrix: Vec<Vec<Option<Entity>>>,
}

impl Board {
    pub fn new(width: usize, height: usize, matrix: Vec<Vec<Option<Entity>>>) -> Self {
        Self {
            width,
            height,
            matrix,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        if x >= self.width || y >= self.height {
            return None;
        }
        *self.matrix.get(x)?.get(y)?
    }

    pub fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn set(&mut self, x: usize, y: usize, entity: Option<Entity>) -> bool {
        if !self.is_valid_position(x, y) {
            return false;
        }
        if let Some(row) = self.matrix.get_mut(x) {
            if let Some(cell) = row.get_mut(y) {
                *cell = entity;
                return true;
            }
        }
        false
    }

    pub fn matrix(&self) -> &Vec<Vec<Option<Entity>>> {
        &self.matrix
    }
}
