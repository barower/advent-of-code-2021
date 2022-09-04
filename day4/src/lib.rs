use std::collections::VecDeque;
use thiserror::Error;

const BINGO_BOARD_SIZE: usize = 5;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GameError {
    #[error("No draws left")]
    NoDrawsLeft,
}

#[derive(Clone, Debug)]
pub struct Player {
    board: [[u64; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE],
    // Vector of marked board indexes
    marked: Vec<(usize,usize)>,
}

impl Player {
    fn new(board: [[u64; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE]) -> Self {
        Player {
            board,
            marked: vec![],
        }
    }

    fn handle_new_draw(&mut self, draw: u64) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == draw {
                    self.marked.push((i,j));
                    return;
                }
            }
        }
    }

    fn check_bingo(&self) -> bool {
        for row in 0..BINGO_BOARD_SIZE {
            if self.check_bingo_row(row) == true {
                return true;
            }
        }

        for col in 0..BINGO_BOARD_SIZE {
            if self.check_bingo_col(col) == true {
                return true;
            }
        }

        false
    }

    fn check_bingo_row(&self, row: usize) -> bool {
        (0..BINGO_BOARD_SIZE).map(|col| (row, col)).all(|pair| self.marked.contains(&pair))
    }

    fn check_bingo_col(&self, col: usize) -> bool {
        (0..BINGO_BOARD_SIZE).map(|row| (row, col)).all(|pair| self.marked.contains(&pair))
    }

    pub fn sum_of_unmarked(&self) -> u64 {
        let mut sum: u64 = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if !self.marked.contains(&(i,j)) {
                    sum += val;
                }
            }
        }
        sum
    }
}

impl std::str::FromStr for Player {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board: [[u64; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE] = [[0; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE];
        for (row, line) in s.lines().enumerate() {
            board[row] = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>().try_into().unwrap();
        }
        Ok(Player::new(board))
    }
}

pub struct Game {
    players: Vec<Player>,
    draws: VecDeque<u64>,
}

impl Game {
    pub fn new(draws: Vec<u64>) -> Self {
        Game { players: vec![], draws: draws.into(), }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn play(&mut self) -> Result<Option<(u64, Player)>, GameError> {
        while !self.draws.is_empty() {
            let draw = self.draws.pop_front().ok_or(GameError::NoDrawsLeft)?;

            for player in &mut self.players {
                player.handle_new_draw(draw);
                if player.check_bingo() {
                    return Ok(Some((draw, player.to_owned())));
                }
            }
        }
        Ok(None)
    }
}

impl std::str::FromStr for Game {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let draws = s.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
        Ok(Game::new(draws))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str() -> Result<(), Box<dyn std::error::Error>> {
        let input: String = "21,37,09,43,56".to_string();
        let game: Game = std::str::FromStr::from_str(&input)?;
        assert_eq!(game.draws[2], 9);
        Ok(())
    }

    #[test]
    fn player_from_str() -> Result<(), Box<dyn std::error::Error>> {
        let input: String = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19".to_string();

        let player: Player = std::str::FromStr::from_str(&input)?;
        assert_eq!(player.board[0][1], 13);
        assert_eq!(player.board[2][2], 14);
        Ok(())
    }

    #[test]
    fn single_player_draw() {

        let mut player = Player::new([[22, 13, 17, 11, 0],
                                      [8,  2,  23, 4,  24],
                                      [21, 9,  14, 16, 7],
                                      [6,  10, 3,  18, 5],
                                      [1,  12, 20, 15, 19]]);

        player.handle_new_draw(23);
        assert_eq!(player.marked, vec![(1,2)]);

        player.handle_new_draw(6);
        assert_eq!(player.marked, vec![(1,2), (3,0)]);

        player.handle_new_draw(5555);
        assert_eq!(player.marked, vec![(1,2), (3,0)]);

    }

    #[test]
    fn single_player_row_bingo() {
        let mut player = Player::new([[22, 13, 17, 11, 0],
                                      [8,  2,  23, 4,  24],
                                      [21, 9,  14, 16, 7],
                                      [6,  10, 3,  18, 5],
                                      [1,  12, 20, 15, 19]]);

        player.handle_new_draw(8);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(2);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(23);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(4);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(24);
        assert_eq!(player.check_bingo(), true);
    }

    #[test]
    fn single_player_column_bingo() {
        let mut player = Player::new([[22, 13, 17, 11, 0],
                                      [8,  2,  23, 4,  24],
                                      [21, 9,  14, 16, 7],
                                      [6,  10, 3,  18, 5],
                                      [1,  12, 20, 15, 19]]);

        player.handle_new_draw(17);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(23);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(14);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(3);
        assert_eq!(player.check_bingo(), false);

        player.handle_new_draw(20);
        assert_eq!(player.check_bingo(), true);
    }

    #[test]
    fn single_player_sum_of_unmarked() {
        let board = [[22, 13, 17, 11, 0 ],
                     [8,  2,  23, 4,  24],
                     [21, 9,  14, 16, 7 ],
                     [6,  10, 3,  18, 5 ],
                     [1,  12, 20, 15, 19]];

        let mut player = Player::new(board);

        player.handle_new_draw(17);
        player.handle_new_draw(19);

        assert_eq!(player.sum_of_unmarked(), board.iter().flatten().sum::<u64>()-17-19);
    }


    #[test]
    fn part1() {
        let mut game = Game::new(vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);

        game.add_player(Player::new([[22, 13, 17, 11, 0],
                                     [8,  2,  23, 4,  24],
                                     [21, 9,  14, 16, 7],
                                     [6,  10, 3,  18, 5],
                                     [1,  12, 20, 15, 19]]));

        game.add_player(Player::new([[3, 15,  0,  2, 22],
                                     [9, 18, 13, 17,  5],
                                     [19,  8,  7, 25, 23],
                                     [20, 11, 10, 24,  4],
                                     [14, 21, 16, 12,  6]]));

        game.add_player(Player::new([[14, 21, 17, 24,  4],
                                     [10, 16, 15,  9, 19],
                                     [18,  8, 23, 26, 20],
                                     [22, 11, 13,  6,  5],
                                     [ 2,  0, 12,  3,  7]]));

        let (winning_draw, winner) = game.play().unwrap().unwrap();
        assert_eq!(winner.board[0][0], 14);
        assert_eq!(winning_draw, 24);
        assert!(winner.marked.contains(&(1,3)));
    }
}
