pub fn larger(data: Vec<i64>) -> usize {
    data.windows(2).filter(|pair| pair[0] < pair[1] ).count()
}

pub fn larger_v2(data: Vec<i64>) -> usize {
    data.windows(3)
        .map(|triple| triple[0] + triple[1] + triple[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let measurements = vec![];
        assert_eq!(larger(measurements), 0);
    }

    #[test]
    fn example() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(larger(measurements), 7);
    }

    #[test]
    fn empty_v2() {
        let measurements = vec![];
        assert_eq!(larger_v2(measurements), 0);
    }

    #[test]
    fn example_v2() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(larger_v2(measurements), 5);
    }
}
