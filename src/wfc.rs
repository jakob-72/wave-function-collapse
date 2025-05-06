use crate::Result;
use crate::matrix::Matrix;
use crate::rules::Ruleset;
use crate::shared::WfcError;
use crate::vec2i::{DOWN, LEFT, RIGHT, UP, Vec2i};
use rand::{Rng, random_range};
use std::collections::{HashMap, VecDeque};

/// The Wave Function Collapse (WFC) algorithm implementation.
pub struct Wfc {
    /// The matrix representing the current state of the WFC algorithm.
    /// Will be initialized with -1 (representing superposition). After the algorithm runs,
    /// it will contain the final state.
    matrix: Matrix,
    /// The ruleset defining the constraints of each state. Provided by the user.
    ruleset: Ruleset,
    /// The list of all available fields (states) for the WFC algorithm provided by the ruleset.
    available_fields: Vec<i8>,
    /// The list of fields to process. This is a list of coordinates that have not been processed yet.
    /// This is an alternative to using a recursive function to avoid stack overflow.
    /// This way we can process the fields in a non-recursive manner and keep track of the fields on the heap.
    fields_to_process: VecDeque<Vec2i>,
}

impl Wfc {
    pub fn new(cols: usize, rows: usize, ruleset: Ruleset) -> Self {
        Wfc {
            matrix: Matrix::new(cols, rows),
            available_fields: ruleset.all_fields(),
            ruleset,
            fields_to_process: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Running WFC algorithm...");
        let start = std::time::Instant::now();
        let x = random_range(0..self.matrix.cols);
        let y = random_range(0..self.matrix.rows);
        self.fields_to_process
            .push_front(Vec2i::new(x as i32, y as i32));

        while let Some(next_pos) = self.fields_to_process.pop_front() {
            self.eval_position(next_pos)?;
        }

        println!("Finished in {:.2?}", start.elapsed());
        Ok(())
    }

    pub fn print_matrix(&self, colorful: bool) {
        if self.matrix.cols * self.matrix.rows > 10_000 {
            println!("Matrix is too large to display.");
            return;
        }
        if colorful {
            self.matrix.display_colorful(&self.ruleset);
        } else {
            println!("{}", self.matrix);
        }
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
            let mut neighbors = HashMap::new();
            for dir in [UP, RIGHT, DOWN, LEFT] {
                let neighbor_coords = pos + dir;
                if self.is_in_bounds(neighbor_coords) {
                    let neighbor_field =
                        self.matrix[(neighbor_coords.x as usize, neighbor_coords.y as usize)];
                    if neighbor_field != -1 {
                        neighbors.insert(
                            match dir {
                                UP => "up",
                                RIGHT => "right",
                                DOWN => "down",
                                LEFT => "left",
                                _ => unreachable!(),
                            },
                            neighbor_field,
                        );
                    }
                }
            }
            return Err(WfcError::new(format!(
                "No possible states for position: {:?}, neighbors: {:?} - check ruleset",
                pos, neighbors
            )));
        }
        let selected_field = choose_weighted(possible_states)?;
        self.matrix[(pos.x as usize, pos.y as usize)] = selected_field;

        for dir in [UP, RIGHT, DOWN, LEFT] {
            let neighbor_coords = pos + dir;
            if self.is_in_bounds(neighbor_coords)
                && self.matrix[(neighbor_coords.x as usize, neighbor_coords.y as usize)] == -1
            {
                self.fields_to_process.push_back(neighbor_coords);
            }
        }
        Ok(())
    }

    fn get_possible_states(&self, coords: Vec2i) -> Result<HashMap<i8, f32>> {
        let mut result = HashMap::from_iter(
            self.available_fields
                .iter()
                .map(|&field| (field, 1.0))
                .collect::<Vec<_>>(),
        );
        for dir in [UP, RIGHT, DOWN, LEFT] {
            let neighbor_coords = coords + dir;
            if self.is_in_bounds(neighbor_coords) {
                let neighbor_field =
                    self.matrix[(neighbor_coords.x as usize, neighbor_coords.y as usize)];
                if neighbor_field == -1 {
                    continue;
                }
                let allowed_fields = self.ruleset.get_allowed_fields(neighbor_field, dir.inv())?;
                result.retain(|&field, _| allowed_fields.contains_key(&field));
                for (field, weight) in allowed_fields {
                    if let Some(existing_weight) = result.get_mut(field) {
                        if existing_weight != weight {
                            *existing_weight *= weight;
                        }
                    }
                }
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
}

fn choose_weighted(options: HashMap<i8, f32>) -> Result<i8> {
    let total_weight: f32 = options.values().sum();
    let mut rng = rand::rng();
    let random_value: f32 = rng.random_range(0.0..total_weight);
    let mut cumulative_weight = 0.0;
    for (field, weight) in options {
        cumulative_weight += weight;
        if random_value < cumulative_weight {
            return Ok(field);
        }
    }
    Err(WfcError::new(
        "choose_weighted -> No field selected - this should not happen with valid weights"
            .to_string(),
    ))
}
