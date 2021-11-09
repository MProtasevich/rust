pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }
    let mut mask: Vec<Vec<i8>> = vec![vec![]; minefield.len()];
    for i in 0..mask.len() {
        mask[i].resize(minefield[0].len(), 0);
    }

    for (i, &row) in minefield.iter().enumerate() {
        for (j, chr) in row.chars().enumerate() {
            if chr == '*' {
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        let x = (i as isize + x_offset) as usize;
                        let y = (j as isize + y_offset) as usize;
                        if (0..minefield.len()).contains(&x) && (0..row.len()).contains(&y) {
                            mask[x][y] += 1;
                        }
                    }
                }
                mask[i][j] = i8::MIN;
            }
        }
    }

    let mut result = Vec::with_capacity(minefield.len());
    for i in 0..(mask.len()) {
        let row = mask[i].iter()
            .map(|minecell| match minecell {
                i8::MIN..=-1 => '*',
                0 => ' ',
                &x => x.to_string().pop().unwrap(),
            })
            .collect::<String>();
        result.push(row);
    }
    result
}
