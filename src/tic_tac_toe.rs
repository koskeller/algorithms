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
    columns: Vec<i32>,
    diag: i32,
    anti_diag: i32,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size: size as i32,
            rows: vec![0; size],
            columns: vec![0; size],
            diag: 0,
            anti_diag: 0,
        }
    }

    pub fn play(&mut self, moves: Vec<Vec<i32>>) -> bool {
        let mut x: i32 = 1;
        for m in moves {
            let row = m[0] as usize;
            self.rows[row] += x;

            let col = m[1] as usize;
            self.columns[col] += x;

            if row == col {
                self.diag += x;
            }

            if row + col == self.size as usize - 1 {
                self.anti_diag += x;
            }

            if x == 1 {
                x = -1;
            } else {
                x = 1;
            }

            if self.rows[row] == self.size
                || self.columns[col] == self.size
                || self.diag == self.size
                || self.anti_diag == self.size
            {
                return true;
            }
        }

        return false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(3);
        assert_eq!(game.size, 3);
        assert_eq!(game.rows, vec![0; 3]);
        assert_eq!(game.columns, vec![0; 3]);
        assert_eq!(game.diag, 0);
        assert_eq!(game.anti_diag, 0);
    }

    #[test]
    fn test_play_no_winner() {
        let mut game = Game::new(3);
        let moves = vec![vec![0, 0], vec![1, 1], vec![0, 2], vec![2, 2]];
        assert_eq!(game.play(moves), false);
    }

    #[test]
    fn test_play_winner_row() {
        let mut game = Game::new(3);
        let moves = vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![2, 2], vec![0, 2]];
        assert_eq!(game.play(moves), true);
    }

    #[test]
    fn test_play_winner_column() {
        let mut game = Game::new(3);
        let moves = vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![2, 2], vec![2, 0]];
        assert_eq!(game.play(moves), true);
    }

    #[test]
    fn test_play_winner_left_diag() {
        let mut game = Game::new(3);
        let moves = vec![vec![0, 0], vec![0, 1], vec![1, 1], vec![0, 2], vec![2, 2]];
        assert_eq!(game.play(moves), true);
    }

    #[test]
    fn test_play_winner_right_diag() {
        let mut game = Game::new(3);
        let moves = vec![vec![0, 2], vec![0, 1], vec![1, 1], vec![0, 0], vec![2, 0]];
        assert_eq!(game.play(moves), true);
    }
}
