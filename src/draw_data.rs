use allegro::Color;

pub struct DrawData {
    cell_color: Color,
    cell_size: f32,
    cell_type: CellType
}

pub enum CellType {
    Circle,
    Square
}

impl DrawData {

    pub fn new(cell_count: (u32, u32), screen_size: (u32, u32), cell_color: (u8, u8, u8), cell_type: CellType) -> DrawData {
        let cell_color = Color::from_rgb(cell_color.0, cell_color.1, cell_color.2);

        let cell_x = screen_size.0 as f32 / cell_count.0 as f32;
        let cell_y = screen_size.1 as f32 / cell_count.1 as f32;

        let cell_size = match cell_x < cell_y {
            true => cell_x,
            false => cell_y
        };

        DrawData {
            cell_color: cell_color,
            cell_size: cell_size,
            cell_type: cell_type
        }
    }

    pub fn get_cell_color(&self) -> Color {
        self.cell_color
    }

    pub fn get_cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn get_cell_type(&self) -> &CellType {
        &self.cell_type
    }
}
