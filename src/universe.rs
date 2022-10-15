use std::{fmt, u32};

use rand::Rng;

use super::cell::Cell;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Universe{
    pub width: u32,
    height: u32,
    cells: Vec<Cell>,
    cell_size: u32,
}
impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }   
    fn get_neigbours(&self, row: u32, col: u32) -> u8{
        let mut neighbours = 0;
        for delta_row in [self.height -1 , 0, 1 ].iter().cloned(){
            for delta_col in [self.width -1, 0 , 1].iter().cloned(){
                if delta_row == 0 && delta_col == 0{
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_column = (col + delta_col) % self.height;
                let neighbour_index = self.get_index(neighbour_row, neighbour_column);
                neighbours += self.cells[neighbour_index] as u8;
            }
        }
        neighbours
    }
}

impl Universe {
    pub fn tick(&mut self) {
        let mut next_gen = self.cells.clone();
        for row in 0..self.height{
            for col in 0..self.width{
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let neighbours = self.get_neigbours(row, col);
                let next_cell = match (cell, neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next_gen[index] =  next_cell;
            }
        }
        self.cells = next_gen;
    }

    pub fn new() -> Universe{
        let width = 64;
        let height = 64;
        let cell_size = 5;
        let mut rng = rand::thread_rng();
        let cells = (0..height*width)
        .map(|_i|{
            let chance:f32 = rng.gen(); 
            if chance > 0.3{
                Cell::Alive
            }else {
                Cell::Dead
            }
        }).collect();
        Universe { width, height, cells, cell_size }
    }
    pub fn draw_grid(&self, painter: &egui::Painter){
        for i in 0..(self.width+1){
            let start_x:f32 = (i * (self.cell_size + 1) + 1) as f32;
            let end_y:f32 = (self.height * (self.cell_size + 1) + 1) as f32;
            painter.add(
                egui::Shape::line_segment([
                    egui::Pos2::new(start_x, 0.0),
                    egui::Pos2::new(start_x, end_y)
                ],
                egui::Stroke::new(1.0, egui::Color32::BLACK)
            ));
        };
        for j in 0..(self.height+1){
            let start_y:f32 = (j * (self.cell_size + 1) + 1) as f32;
            let end_x:f32 = (self.width * (self.cell_size + 1) + 1) as f32;
            painter.add(
                egui::Shape::line_segment([
                    egui::Pos2::new(0.0, start_y),
                    egui::Pos2::new(end_x, start_y)
                ],
                egui::Stroke::new(1.0, egui::Color32::BLACK)
            ));
        };
    }
    pub fn draw_cells(&self, painter: &egui::Painter){
        for x in 0..self.width{
            for y in 0..self.height{
                let index = self.get_index(y, x);
                let startx = (x * (self.cell_size +1) +1) as f32;
                let starty = (y * (self.cell_size + 1) +1) as f32;
                let color = match self.cells[index] {
                    Cell::Alive => egui::Color32::BLACK,
                    Cell::Dead => egui::Color32::WHITE
                };
                painter.rect_filled(egui::Rect::from_two_pos(
                    egui::Pos2 { x: startx , y: starty }, 
                    egui::Pos2 { x: startx + self.cell_size as f32, y: starty + self.cell_size as f32 }),
                egui::Rounding::none(), color);
            } 
        }
    }
}
impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize){
            for &cell in line{
                let sprite = if cell == Cell::Alive {"◼"} else {"◻"};
                write!(f, "{ }", sprite)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}