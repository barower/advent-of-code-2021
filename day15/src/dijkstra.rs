use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight<T> {
    Inf,
    NonInf(T),
}

impl<T: PartialOrd> PartialOrd for Weight<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Weight::Inf, Weight::Inf) => None,
            (Weight::NonInf(_), Weight::Inf) => Some(Ordering::Less),
            (Weight::Inf, Weight::NonInf(_)) => Some(Ordering::Greater),
            (Weight::NonInf(left), Weight::NonInf(right)) => left.partial_cmp(right),
        }
    }
}

impl<T> Default for Weight<T> {
    fn default() -> Self {
        Weight::Inf
    }
}

#[derive(Default, Clone)]
struct DijkstraEntry {
    weight: Weight<u64>,
    from: Option<(usize, usize)>,
    visited: bool,
}

pub struct DijkstraMap(Vec<Vec<DijkstraEntry>>);

impl DijkstraMap {
    pub fn new(width: usize, height: usize, start: (usize, usize)) -> Self {
        let mut retvec = vec![vec![DijkstraEntry::default(); width]; height];

        let (x, y) = start;
        retvec[y][x] = DijkstraEntry {
            weight: Weight::NonInf(0),
            from: None,
            visited: false,
        };

        DijkstraMap(retvec)
    }

    pub fn update(&mut self, (x, y): (usize, usize), from: (usize, usize), newweight: u64) {
        if Weight::NonInf(newweight) < self.0[y][x].weight {
            self.0[y][x].weight = Weight::NonInf(newweight);
            self.0[y][x].from = Some(from);
        }
    }

    pub fn mark_visited(&mut self, (x, y): (usize, usize)) {
        self.0[y][x].visited = true;
    }

    pub fn get_smallest_non_visited(&self) -> Option<(usize, usize)> {
        let mut current_weight = Weight::Inf;
        let mut ret_position: Option<(usize, usize)> = None;

        for (y, row) in self.0.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                let weight = entry.weight;
                if !self.is_visited((x, y)) && weight < current_weight {
                    current_weight = weight;
                    ret_position = Some((x, y));
                }
            }
        }

        ret_position
    }

    pub fn is_visited(&self, (x, y): (usize, usize)) -> bool {
        self.0[y][x].visited
    }

    pub fn get_weight(&self, (x, y): (usize, usize)) -> Weight<u64> {
        self.0[y][x].weight
    }

    pub fn get_from(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        self.0[y][x].from
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_weight() {
        assert!(Weight::NonInf(5) > Weight::NonInf(3));
        assert!(Weight::NonInf(5) == Weight::NonInf(5));
        assert!(Weight::NonInf(5) < Weight::Inf);
        assert!(Weight::Inf > Weight::NonInf(5));
        assert_eq!(Weight::<u64>::Inf.partial_cmp(&Weight::Inf), None);
    }
}
