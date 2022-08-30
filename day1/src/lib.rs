pub fn larger(data: Vec<i64>) -> usize {
    data.windows(2).filter(|pair| pair[0] < pair[1] ).count()
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
}
