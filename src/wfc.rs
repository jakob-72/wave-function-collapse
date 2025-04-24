use crate::Result;
use crate::matrix::Matrix;
use crate::rules::Ruleset;
use crate::shared::WfcError;
use crate::vec2i::{DOWN, LEFT, RIGHT, UP, Vec2i};
use rand::prelude::IndexedRandom;
use rand::random_range;

pub struct Wfc {
    matrix: Matrix,
    ruleset: Ruleset,
    available_fields: Vec<i8>,
    fields_to_process: Vec<Vec2i>,
}

impl Wfc {
    pub fn new(cols: usize, rows: usize, ruleset: Ruleset) -> Self {
        Wfc {
            matrix: Matrix::new(cols, rows),
            available_fields: ruleset.all_fields(),
            ruleset,
            fields_to_process: vec![],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Running WFC algorithm...");
        let start = std::time::Instant::now();
        let x = random_range(0..self.matrix.cols);
        let y = random_range(0..self.matrix.rows);
        self.fields_to_process.push(Vec2i::new(x as i32, y as i32));

        while let Some(next_pos) = self.fields_to_process.pop() {
            self.eval_position(next_pos)?;
        }

        println!("Finished in {:.2?}", start.elapsed());
        Ok(())
    }

    fn eval_position(&mut self, pos: Vec2i) -> Result<()> {
        if !self.is_in_bounds(pos) {
            return Ok(());
        }
        if self.matrix[(pos.x as usize, pos.y as usize)] != -1 {
            return Ok(());
        }
        let possible_states = self.get_possible_states(pos)?;
        if possible_states.is_empty() {
            return Err(WfcError::new(format!(
                "No possible states for position: {:?}",
                pos
            )));
        }
        let selected_field = possible_states.choose(&mut rand::rng()).unwrap();
        self.matrix[(pos.x as usize, pos.y as usize)] = *selected_field;

        for dir in [UP, RIGHT, DOWN, LEFT] {
            let neighbor_coords = pos + dir;
            if self.is_in_bounds(neighbor_coords)
                && self.matrix[(neighbor_coords.x as usize, neighbor_coords.y as usize)] == -1
            {
                self.fields_to_process.push(neighbor_coords);
            }
        }
        Ok(())
    }

    fn get_possible_states(&self, coords: Vec2i) -> Result<Vec<i8>> {
        let mut result = self.available_fields.clone();
        for dir in [UP, RIGHT, DOWN, LEFT] {
            let neighbor_coords = coords + dir;
            if self.is_in_bounds(neighbor_coords) {
                let neighbor_field =
                    self.matrix[(neighbor_coords.x as usize, neighbor_coords.y as usize)];
                if neighbor_field == -1 {
                    continue;
                }
                let allowed_fields = self.ruleset.get_allowed_fields(neighbor_field, dir.inv())?;
                result.retain(|&field| allowed_fields.contains(&field));
            }
        }
        Ok(result)
    }

    fn is_in_bounds(&self, coords: Vec2i) -> bool {
        coords.x < self.matrix.cols as i32
            && coords.x >= 0
            && coords.y < self.matrix.rows as i32
            && coords.y >= 0
    }

    pub fn print_matrix(&self) {
        if self.matrix.cols * self.matrix.rows > 10_000 {
            println!("Matrix is too large to display.");
            return;
        }
        self.matrix.display_colorful();
    }
}
