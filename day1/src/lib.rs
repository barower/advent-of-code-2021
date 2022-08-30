pub fn larger(data: Vec<i64>) -> usize {
    let mut it = data.into_iter();
    let mut prev = match it.next() {
        Some(val) => val,
        None => { return 0; },
    };
    let mut counter = 0;

    for i in it {
        if i > prev {
            counter += 1;
        }
        prev = i;
    }

    counter
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
