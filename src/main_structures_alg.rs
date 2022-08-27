struct Position {
    x: usize,
    y: usize,
}

struct Neighbours<'a> {
    top: Option<&'a mut Cell<'a>>,
    left: Option<&'a mut Cell<'a>>,
    right: Option<&'a mut Cell<'a>>,
    bottom: Option<&'a mut Cell<'a>>,
}

struct Cell<'a> {
    value: i32,
    position: Position,
    neighbours: Neighbours<'a>,
    viewed: bool,
}

fn get_combination<'a>(cell: &'a mut Cell) -> Vec<&'a Cell<'a>> {
    let mut combination = vec![];

    if cell.viewed {
        return combination;
    }

    cell.viewed = true;

    if let Some(top_cell) = cell.neighbours.top.as_deref_mut() {
        if top_cell.value == cell.value && !top_cell.viewed {
            combination.append(&mut get_combination(top_cell));
        }
    }

    if let Some(bottom_cell) = cell.neighbours.bottom.as_deref_mut() {
        if bottom_cell.value == cell.value && !bottom_cell.viewed {
            combination.append(&mut get_combination(bottom_cell));
        }
    }

    if let Some(left_cell) = cell.neighbours.left.as_deref_mut() {
        if left_cell.value == cell.value && !left_cell.viewed {
            combination.append(&mut get_combination(left_cell));
        }
    }

    if let Some(right_cell) = cell.neighbours.right.as_deref_mut() {
        if right_cell.value == cell.value && !right_cell.viewed {
            combination.append(&mut get_combination(right_cell));
        }
    }

    combination
}

fn main() {
    // let mut cell = Cell {
    //     value: 17,
    //     position: Position { x: 0, y: 0 },
    //     neighbours: Neighbours {
    //         top: None,
    //         left: None,
    //         right: None,
    //         bottom: None,
    //     },
    //     viewed: false,
    // };
    //
    // let combinations = Vec<Vec<Cell>>;
    //
    // for row in rows.iter_mut() {
    //     for cell in row.iter_mut() {
    //         let combination = get_combination(&mut cell);
    //
    //         if 5 <= combination.len() {
    //             combinations.push(combination); 
    //         }
    //     }
    // }
    


}
