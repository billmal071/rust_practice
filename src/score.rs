pub fn betterThanAverage(classPoints: &[i32], yourPoints: i32) -> bool {
    let average: i32 = classPoints.iter().sum() / classPoints.len() as i32;
    yourPoints > average
}
