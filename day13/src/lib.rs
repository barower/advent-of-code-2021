use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PaperError {
    #[error("Unknown error")]
    UnknownError,
    #[error("Out of bounds")]
    OutOfBounds,
}

pub struct FoldPaper(Vec<Vec<bool>>);

impl FoldPaper {
    pub fn new(points: Vec<(usize, usize)>) -> Self {
        let furthest_x: usize = points
            .iter()
            .fold(0, |maxx, (x, _)| if *x > maxx { *x } else { maxx });
        let furthest_y: usize = points
            .iter()
            .fold(0, |maxy, (_, y)| if *y > maxy { *y } else { maxy });

        let mut column_vec: Vec<Vec<bool>> = Vec::new();
        column_vec.reserve_exact(furthest_y + 1);
        for _ in 0..(furthest_y + 1) {
            column_vec.push(vec![false; furthest_x + 1]);
        }

        for (x, y) in &points {
            column_vec[*y][*x] = true;
        }

        FoldPaper(column_vec)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn fold_along_x(&mut self, fold_x: usize) -> Result<(), PaperError> {
        if fold_x >= self.width() {
            return Err(PaperError::OutOfBounds);
        }

        let mut points_to_or: Vec<(usize, usize)> = vec![];

        for (y, row) in self.0.iter().enumerate() {
            for (x, elem) in row.iter().skip(fold_x).enumerate().skip(1) {
                if *elem {
                    points_to_or.push((fold_x - x, y));
                }
            }
        }

        for (x, y) in points_to_or {
            self.0[y][x] |= true;
        }

        for row in &mut self.0 {
            row.truncate(fold_x);
        }

        Ok(())
    }

    pub fn fold_along_y(&mut self, fold_y: usize) -> Result<(), PaperError> {
        if fold_y >= self.height() {
            return Err(PaperError::OutOfBounds);
        }

        let mut points_to_or: Vec<(usize, usize)> = vec![];

        for (y, row) in self.0.iter().skip(fold_y).enumerate().skip(1) {
            for (x, elem) in row.iter().enumerate() {
                if *elem {
                    points_to_or.push((x, fold_y - y));
                }
            }
        }

        for (x, y) in points_to_or {
            self.0[y][x] |= true;
        }

        self.0.truncate(fold_y);

        Ok(())
    }

    pub fn count_dots(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .filter(|elem| **elem == true)
            .count()
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }
}

impl Debug for FoldPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for c in row.iter().map(|elem| if *elem { '#' } else { '.' }) {
                write!(f, "{c}")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foldpaper_new() {
        let points = vec![(0, 0), (10, 6), (5, 15)];
        let paper = FoldPaper::new(points);

        assert_eq!(paper.width(), 11);
        assert_eq!(paper.height(), 16);

        assert!(!paper.at(1, 1));
        assert!(paper.at(0, 0));
        assert!(paper.at(5, 15));

        assert_eq!(paper.count_dots(), 3);
    }

    #[test]
    fn foldpaper_fold_x() {
        let points = vec![(0, 0), (10, 6), (5, 15)];
        let mut paper = FoldPaper::new(points);

        assert!(paper.fold_along_x(100).is_err());
        assert!(paper.fold_along_x(7).is_ok());
        assert_eq!(paper.width(), 7);

        assert!(paper.at(4, 6));

        assert_eq!(paper.count_dots(), 3);
    }

    #[test]
    fn foldpaper_fold_y() {
        let points = vec![(0, 0), (10, 6), (5, 15)];
        let mut paper = FoldPaper::new(points);

        assert!(paper.fold_along_y(100).is_err());
        assert!(paper.fold_along_y(10).is_ok());
        assert_eq!(paper.height(), 10);

        assert!(paper.at(5, 5));
        assert!(paper.at(10, 6));
        assert!(paper.at(0, 0));

        assert_eq!(paper.count_dots(), 3);
    }

    #[test]
    fn example() -> Result<(), PaperError> {
        let points = vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ];

        let mut paper = FoldPaper::new(points);
        assert_eq!(paper.count_dots(), 18);

        println!("{:?}", &paper);

        paper.fold_along_y(7)?;
        assert_eq!(paper.count_dots(), 17);

        println!("{:?}", &paper);

        paper.fold_along_x(5)?;
        assert_eq!(paper.count_dots(), 16);

        println!("{:?}", &paper);

        Ok(())
    }
}
