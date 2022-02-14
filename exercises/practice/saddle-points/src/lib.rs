pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, cell_value) in row.iter().cloned().enumerate() {
            let smallest_in_a_column = input.iter().all(|row| cell_value <= row[j]);
            let greatest_in_a_row = row.iter().all(|&cell| cell_value >= cell);
            if smallest_in_a_column && greatest_in_a_row {
                res.push((i, j));
            }
        }
    }
    res
}
