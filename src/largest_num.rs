pub fn largest_num(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    let _ = &list.iter().for_each(|x| if x > largest { largest = x });

    largest
}
