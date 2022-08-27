use rand::Rng;

#[derive(Debug)]
struct Symbol {
    value: i32,
}

#[derive(Debug)]
struct Neighbours {
    top: Option<Position>,
    bottom: Option<Position>,
    left: Option<Position>,
    right: Option<Position>,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Cell {
    symbol: Symbol,
    position: Position,
    neighbours: Neighbours,
    viwed: bool,
}

fn traverse (matri cell: Cell) {
    if let Some(top_position) = cell.neighbours.top {

    }

    if let Some(bottom_position) = cell.neighbours.bottom {

    }

    if let Some(left_position) = cell.neighbours.left {

    }

    if let Some(right_position) = cell.neighbours.right &&  {

    }
}

fn combinations(matrix: &mut Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let combinations = vec![];

    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            if cell.viwed == false {
                cell.viwed = true;


            }
        }
    }

    combinations
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut matrix2: Vec<Vec<Cell>> = vec![];

    for row_index in 0..7 {
        let mut row: Vec<Cell> = vec![];

        for cell_index in 0..7 {
            let has_right_space = cell_index + 1 < 7;
            let has_left_space = 0 < cell_index - 1;
            let has_top_space = 0 < row_index - 1;
            let has_bottom_space = row_index + 1 < 7;

            let cell = Cell {
                symbol: Symbol {
                    value: rng.gen_range(0..6),
                },
                position: Position {
                    x: cell_index,
                    y: row_index,
                },
                neighbours: Neighbours {
                    top: if has_top_space {
                        Some(Position {
                            x: cell_index,
                            y: row_index - 1,
                        })
                    } else {
                        None
                    },
                    left: if has_left_space {
                        Some(Position {
                            x: cell_index - 1,
                            y: row_index,
                        })
                    } else {
                        None
                    },
                    right: if has_right_space {
                        Some(Position {
                            x: cell_index + 1,
                            y: row_index,
                        })
                    } else {
                        None
                    },
                    bottom: if has_bottom_space {
                        Some(Position {
                            x: cell_index,
                            y: row_index + 1,
                        })
                    } else {
                        None
                    },
                },
                viwed: false,
            };

            row.push(cell);
        }

        matrix2.push(row);
    }

    // println!("{:#?}", matrix2);

    let result = combinations(&mut matrix2);

    println!("{:#?}", matrix2);
}
