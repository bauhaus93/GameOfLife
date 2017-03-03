
pub fn rule_1(cell: bool, neighbours: u8) -> Option<bool> {
    match cell == false && neighbours == 3 {
        true => Some(true),
        false => None
    }
}

pub fn rule_2(cell: bool, neighbours: u8) -> Option<bool> {
    match cell == true && neighbours < 2 {
        true => Some(false),
        false => None
    }
}

pub fn rule_3(cell: bool, neighbours: u8) -> Option<bool> {
    match cell == true && (neighbours == 2 || neighbours == 3) {
        true => Some(true),
        false => None
    }
}

pub fn rule_4(cell: bool, neighbours: u8) -> Option<bool> {
    match cell == true && neighbours > 3 {
        true => Some(false),
        false => None
    }
}
