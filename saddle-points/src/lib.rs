pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    if input[0].len() != 0 {
        let mut row_maxes = vec![u64::MIN; input.len()];
        let mut col_mins = vec![u64::MAX; input[0].len()];
        for (rown, row) in input.iter().enumerate() {
            if let Some(row_max) = row.iter().max() {
                row_maxes[rown] = *row_max;
            }
            for (coln, val) in row.iter().enumerate() {
                if *val < col_mins[coln] {
                    col_mins[coln] = *val;
                }
            }
        }
        for (rown, row) in input.iter().enumerate() {
            for (coln, val) in row.iter().enumerate() {
                if *val >= row_maxes[rown] && *val <= col_mins[coln] {
                    ret.push((rown, coln));
                }
            }
        }
    }
    ret
}
