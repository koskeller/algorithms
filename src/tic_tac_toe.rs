#![allow(unused)]
/// Tic-tac-toe is a game played on an n × n board (typically n = 3)
/// where two players take consecutive turns placing “O” and “X” marks
/// onto the board cells. The game is won if n consecutive “O” or ‘X”
/// marks are placed in a row, column, or diagonal. Create a data
// structure with O(n) space that accepts a sequence of moves,
/// and reports in constant time whether the last move won the game.
pub struct Game {
    size: i32,
    rows: Vec<i32>,
    cols: Vec<i32>,
    diag: i32,
    anti_diag: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    X,
    O,
    Draw,
    Pending,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size: size as i32,
            rows: vec![0; size],
            cols: vec![0; size],
            diag: 0,
            anti_diag: 0,
        }
    }

    pub fn play(&mut self, moves: Vec<Vec<i32>>) -> Outcome {
        let mut count: i32 = 1;
        for mov in &moves {
            let row = mov[0] as usize;
            self.rows[row] += count;

            let col = mov[1] as usize;
            self.cols[col] += count;

            if row == col {
                self.diag += count;
            }

            if row + col == self.size as usize - 1 {
                self.anti_diag += count;
            }

            if count == 1 {
                count = -1;
            } else {
                count = 1;
            }

            if self.rows[row] == self.size
                || self.cols[col] == self.size
                || self.diag == self.size
                || self.anti_diag == self.size
            {
                return Outcome::X;
            }

            if self.rows[row] == -self.size
                || self.cols[col] == -self.size
                || self.diag == -self.size
                || self.anti_diag == -self.size
            {
                return Outcome::O;
            }
        }

        if moves.len() == (self.size * self.size) as usize {
            return Outcome::Draw;
        }

        return Outcome::Pending;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_a_win_for_x_in_a_row() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![2, 2],
                vec![0, 2]
            ]),
            Outcome::X
        );
    }

    #[test]
    fn it_detects_a_win_for_o_in_a_column() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 1],
                vec![2, 0],
                vec![2, 1]
            ]),
            Outcome::X
        );
    }

    #[test]
    fn it_detects_a_win_for_x_in_a_diagonal() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            Outcome::X
        );
    }

    #[test]
    fn it_detects_a_win_for_o_in_an_anti_diagonal() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![
                vec![0, 0],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2]
            ]),
            Outcome::O
        );
    }

    #[test]
    fn it_detects_a_draw() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            Outcome::Draw
        );
    }

    #[test]
    fn it_returns_pending_if_game_is_still_in_progress() {
        let mut game = Game::new(3);
        assert_eq!(
            game.play(vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 0]]),
            Outcome::Pending
        );
    }
}
