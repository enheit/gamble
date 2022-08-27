use colored::{ColoredString, Colorize};
use rand::Rng;
use std::collections::HashSet;

fn colorize(value: i32) -> ColoredString {
    let value_str = format!("{}", value);

    match value {
        0 => value_str.blue(),
        1 => value_str.red(),
        2 => value_str.green(),
        3 => value_str.yellow(),
        4 => value_str.purple(),
        5 => value_str.cyan(),
        6 => value_str.white(),
        _ => value_str.white(),
    }
}

fn get_combination(
    ri: usize,
    ci: usize,
    pri: usize,
    pci: usize,
    matrix: &[[i32; 7]; 7],
    viewed_indexes: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];

    if viewed_indexes.contains(&(ri, ci)) {
        return result;
    }

    viewed_indexes.insert((ri, ci));

    result.push((ri, ci));

    if 0 < ri && 0 < ri - 1 {
        if ri - 1 != pri {
            let value = matrix[ri][ci];
            let neighbour_value = matrix[ri - 1][ci];

            if value == neighbour_value {
                let mut combination = get_combination(ri - 1, ci, ri, ci, matrix, viewed_indexes);

                result.append(&mut combination);
            }
        }
    }

    if ci + 1 < matrix[ri].len() {
        if ci + 1 != pci {
            let value = matrix[ri][ci];
            let neighbour_value = matrix[ri][ci + 1];

            if value == neighbour_value {
                let mut combination = get_combination(ri, ci + 1, ri, ci, matrix, viewed_indexes);

                result.append(&mut combination);
            }
        }
    }

    if ri + 1 < matrix.len() {
        if ri + 1 != pri {
            let value = matrix[ri][ci];
            let neighbour_value = matrix[ri + 1][ci];

            if value == neighbour_value {
                let mut combination = get_combination(ri + 1, ci, ri, ci, matrix, viewed_indexes);

                result.append(&mut combination);
            }
        }
    }

    if 0 < ci && 0 < ci - 1 {
        if ci - 1 != pci {
            let value = matrix[ri][ci];
            let neighbour_value = matrix[ri][ci - 1];

            if value == neighbour_value {
                let mut combination = get_combination(ri, ci - 1, ri, ci, matrix, viewed_indexes);

                result.append(&mut combination);
            }
        }
    }

    result
}

fn main() {
    let mut viewed_indexes: HashSet<(usize, usize)> = HashSet::new();
    let mut combinations: Vec<Vec<(usize, usize)>> = vec![];

    let mut matrix: [[i32; 7]; 7] = [[0; 7]; 7];
    let mut rng = rand::thread_rng();

    for row in matrix.iter_mut() {
        for value in row.iter_mut() {
            *value = rng.gen_range(0..7);
        }
    }

    // let matrix = [
    //     [5, 4, 2, 5, 2, 5, 3],
    //     [1, 4, 5, 4, 1, 6, 0],
    //     [2, 4, 4, 4, 1, 6, 0],
    //     [2, 2, 5, 4, 2, 3, 1],
    //     [2, 2, 2, 3, 6, 5, 2],
    //     [5, 0, 0, 0, 0, 0, 0],
    //     [0, 2, 6, 3, 4, 4, 2],
    // ];

    for (row_index, row) in matrix.iter().enumerate() {
        for (cell_index, _) in row.iter().enumerate() {
            if viewed_indexes.contains(&(row_index, cell_index)) == false {
                let combination = get_combination(
                    row_index,
                    cell_index,
                    row_index,
                    cell_index,
                    &matrix,
                    &mut viewed_indexes,
                );

                if 5 <= combination.len() {
                    combinations.push(combination);
                }
            }
        }
    }

    let mut winning_indexes: HashSet<(usize, usize)> = HashSet::new();

    for combination in combinations.iter() {
        for position in combination.iter() {
            winning_indexes.insert(*position);
        }
    }

    println!("");
    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let colorized_value = colorize(*value);
            let underline_value = if winning_indexes.contains(&(row_index, col_index)) {
                colorized_value.underline()
            } else {
                colorized_value
            };

            print!("{} ", underline_value);
        }
        println!("");
    }
    println!("");

    for combination in combinations.iter() {
        for position in combination.iter() {
            print!("({},{}) ", position.0, position.1);
        }
        println!("");
    }
    println!("");
}

// Matrix:
//
// 5, 4, 2, 5, 2, 5, 3
// 1, 4, 5, 4, 1, 6, 0
// 2, 4, 4, 4, 1, 6, 0
// 2, 2, 5, 4, 2, 3, 1
// 2, 2, 2, 3, 6, 5, 2
// 5, 0, 0, 0, 0, 0, 0
// 0, 2, 6, 3, 4, 4, 2
//
// Result:
//
// 4, 4, 4, 4, 4, 4, 4  (7)
// 2, 2, 2, 2, 2        (5)
// 0, 0, 0, 0, 0, 0     (6)
//
// or
//
// (0, 1), (1, 1), (2, 1), (2, 2), (2, 3), (3, 3), (1, 3)
// (3, 0), (4, 0), (3, 1), (4, 1), (4, 2)
// (5, 1), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6)
