use std::io::{self, Write};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
    X,
    O,
}
impl Player {
    fn opposite(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
fn player_to_str(player: Option<Player>) -> String {
    match player {
        Some(Player::X) => "X".to_string(),
        Some(Player::O) => "O".to_string(),
        None => "-".to_string(),
    }
}

#[derive(Copy, Clone)]
struct Game {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}
impl Game{
    fn new() -> Self {
        Game {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }

    fn print_board(&self){
        print!("\n");
        print!("  1 2 3\n");
        print!(" -------\n");
        print!("1|{}|{}|{}|\n", player_to_str(self.board[0][0]), player_to_str(self.board[0][1]), player_to_str(self.board[0][2]));
        print!("2|{}|{}|{}|\n", player_to_str(self.board[1][0]), player_to_str(self.board[1][1]), player_to_str(self.board[1][2]));
        print!("3|{}|{}|{}|\n", player_to_str(self.board[2][0]), player_to_str(self.board[2][1]), player_to_str(self.board[2][2]));
    }

    fn make_move(&mut self, row: usize, col: usize) -> bool{
        if row < 3 && col < 3 && self.board[row][col].is_none() {
            self.board[row][col] = Some(self.current_player);
            true
        } 
        else {
            false
        }
    }

    fn switch_player(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    fn check_winner(&self) -> Option<Player> {
        let lines = &[
            // Horizontal lines
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Vertical lines
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonal lines
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for line in lines {
            let (r1, c1) = line[0];
            let (r2, c2) = line[1];
            let (r3, c3) = line[2];
            
            if self.board[r1][c1].is_none() || self.board[r1][c1] != self.board[r2][c2] || self.board[r1][c1] != self.board[r3][c3] {
                continue;
            }
            return self.board[r1][c1];
        }

        None
    }

    fn play(&mut self) {
        loop {
            print!("---------------------------------------------\n");
            self.print_board();
            println!("\n\nPlayer {:?}, enter your move (row column):", self.current_player);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            print!("\n");

            let coordinates: Vec<usize> = input
                .split_whitespace()
                .filter_map(|coord| coord.parse().ok())
                .collect();

            if coordinates.len() != 2 {
                println!("Invalid input. Please enter row and column numbers separated by a space.");
                continue;
            }

            let mut row = coordinates[0];
            let mut col = coordinates[1];
            
            if row > 3 || col > 3 || row < 1 || col < 1 {
                println!("Invalid input. Please enter row and column numbers between 1 and 3.");
                continue;
            }

            row -= 1;
            col -= 1;

            if !self.make_move(row, col) {
                println!("Invalid move. Please try again.");
                continue;
            }

            self.switch_player();

            if let Some(winner) = self.check_winner() {
                print!("{} wins!", player_to_str(Some(winner)));
                print!("\n---------------------------------------------\n");
                break;
            }
        }
    }
}


fn main() {
    //clear screen
    print!("\x1B[2J\x1B[H");

    //make a new game and call play
    let mut game = Game::new();
    game.play();

    loop{
        print!("Play again? (y/n): ");
        io::stdout().flush().expect("Failed to flush stdout.");
        //restart game if user wants to
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if input.trim() == "y" {
            main();
            continue;
        } else {
            break;
        }
    }
}