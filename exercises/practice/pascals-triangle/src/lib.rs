pub struct PascalsTriangle {
    row_count: usize
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count as usize }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = Vec::with_capacity(self.row_count);
        if self.row_count >= 1 {
            res.push(vec![1]);
        }
        for row_id in 1..self.row_count {
            let prev_row = &res[row_id - 1];
            let mut row = Vec::with_capacity(row_id + 1);

            for cell_id in 0..=row_id {
                let left_num = prev_row.get(cell_id.overflowing_sub(1).0).unwrap_or(&0);
                let right_num = prev_row.get(cell_id).unwrap_or(&0);
                row.push(left_num + right_num);
            }
            res.push(row);
        }
        res
    }
}
