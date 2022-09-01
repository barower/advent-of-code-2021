pub struct BitCounter(Vec<SingleBitCounter>);

impl BitCounter {
    pub fn new(size: usize) -> Self {
        BitCounter(vec![SingleBitCounter{ones: 0, zeros: 0}; size])
    }

    pub fn update(&mut self, s: &str) {
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => self.0[i].zeros += 1,
                '1' => self.0[i].ones += 1,
                _ => panic!("Oops"),
            }
        }
    }

    pub fn gamma_rate(&self) -> u64 {
        let s: String = self.0.iter().map(|sbc| if sbc.ones > sbc.zeros { '1' } else { '0' }).collect();

        u64::from_str_radix(s.as_ref(), 2).unwrap()
    }

    pub fn epsilon_rate(&self) -> u64 {
        let s: String = self.0.iter().map(|sbc| if sbc.zeros > sbc.ones { '1' } else { '0' }).collect();

        u64::from_str_radix(s.as_ref(), 2).unwrap()
    }

    pub fn power_consumption(&self) -> u64 {
        self.gamma_rate() * self.epsilon_rate()
    }
}

#[derive(Clone)]
struct SingleBitCounter {
    ones: u64,
    zeros: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let indata = vec!["00100",
                          "11110",
                          "10110",
                          "10111",
                          "10101",
                          "01111",
                          "00111",
                          "11100",
                          "10000",
                          "11001",
                          "00010",
                          "01010"];

        let mut bc = BitCounter::new(indata[0].len());

        for line in indata {
            bc.update(line);
        }

        assert_eq!(bc.0[0].ones, 7);
        assert_eq!(bc.0[0].zeros, 5);

        assert_eq!(bc.0[2].ones, 8);
        assert_eq!(bc.0[2].zeros, 4);

        assert_eq!(bc.gamma_rate(), 22);
        assert_eq!(bc.epsilon_rate(), 9);

        assert_eq!(bc.power_consumption(), 198);
    }
}
