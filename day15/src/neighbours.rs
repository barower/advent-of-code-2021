use std::collections::HashSet;

enum NeighboursDirections {
    Up,
    Down,
    Left,
    Right
}

pub struct NeighboursIterator {
    point_of_interest: (usize, usize),
    rel_points_to_check: Vec<NeighboursDirections>,
    width: usize,
    height: usize,
}

impl NeighboursIterator {
    pub fn new(point_of_interest: (usize, usize), width: usize, height: usize) -> NeighboursIterator {
        NeighboursIterator {
            point_of_interest,
            rel_points_to_check: vec![NeighboursDirections::Up, NeighboursDirections::Down, NeighboursDirections::Left, NeighboursDirections::Right],
            width,
            height,
        }
    }
}

impl Iterator for NeighboursIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.point_of_interest;

        while let Some(direction) = self.rel_points_to_check.pop() {
            match direction {
                NeighboursDirections::Up => {
                    if let Some(newy) = y.checked_sub(1) {
                        return Some((x, newy));
                    }
                }
                NeighboursDirections::Down => {
                    let newy = y + 1;
                    if newy < self.height {
                        return Some((x, newy));
                    }
                }
                NeighboursDirections::Left => {
                    if let Some(newx) = x.checked_sub(1) {
                        return Some((newx, y));
                    }
                }
                NeighboursDirections::Right => {
                    let newx = x + 1;
                    if newx < self.width {
                        return Some((newx, y));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_inside() {
        let neighbours_it = NeighboursIterator::new((3, 3), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(2, 3), (3, 2), (4, 3), (3, 4)].into_iter())
        );
    }

    #[test]
    fn neighbours_left_edge() {
        let neighbours_it = NeighboursIterator::new((0, 3), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(0, 2), (0, 4), (1, 3)].into_iter())
        );
    }

    #[test]
    fn neighbours_right_edge() {
        let neighbours_it = NeighboursIterator::new((9, 3), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(9, 2), (9, 4), (8, 3)].into_iter())
        );
    }

    #[test]
    fn neighbours_lower_edge() {
        let neighbours_it = NeighboursIterator::new((5, 9), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(4, 9), (6, 9), (5, 8)].into_iter())
        );
    }

    #[test]
    fn neighbours_upper_edge() {
        let neighbours_it = NeighboursIterator::new((5, 0), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(4, 0), (6, 0), (5, 1)].into_iter())
        );
    }

    #[test]
    fn neighbours_left_upper_corner() {
        let neighbours_it = NeighboursIterator::new((0, 0), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(1, 0), (0, 1)].into_iter())
        );
    }

    #[test]
    fn neighbours_right_upper_corner() {
        let neighbours_it = NeighboursIterator::new((9, 0), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(8, 0), (9, 1)].into_iter())
        );
    }

    #[test]
    fn neighbours_left_lower_corner() {
        let neighbours_it = NeighboursIterator::new((0, 9), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(0, 8), (1, 9)].into_iter())
        );
    }

    #[test]
    fn neighbours_right_lower_corner() {
        let neighbours_it = NeighboursIterator::new((9, 9), 10, 10);
        let neighbours: HashSet<(usize, usize)> = HashSet::from_iter(neighbours_it);

        assert_eq!(
            neighbours,
            HashSet::from_iter(vec![(9, 8), (8, 9)].into_iter())
        );
    }
}
