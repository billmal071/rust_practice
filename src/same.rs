pub fn same(vec1: &mut Vec<usize>, vec2: &mut Vec<usize>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    vec1.sort();
    vec2.sort();
    // let mut result: Vec<bool> = vec![];
    vec1.iter()
        .enumerate()
        .map(|(idx, val)| {
            val.pow(2) == vec2[idx]
        })
        .all(|x| x)
}
