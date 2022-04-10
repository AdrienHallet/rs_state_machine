/// Represents an Adjascency Matrix for the [Graph]
#[derive(Debug, Default)]
pub struct Matrix {
    pub matrix: Vec<Vec<Option<String>>>
}

impl Matrix {

    pub fn new() -> Matrix {
        Self {
            matrix: vec![],
        }
    }

    pub fn add_row(&mut self) {
        self.matrix.iter_mut().for_each(|row| row.push(None));
        self.matrix.push(vec![None; self.matrix.len() + 1]);
    }

    pub fn set_option(&mut self, x: usize, y: usize, option: Option<String>) {
        self.matrix[x][y] = option;
    }
}