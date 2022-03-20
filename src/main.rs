/*******************************************************************************
* Tic Tac Toe in Rust - By : Collette Stapley
********************************************************************************/

use std::io;

/*******************************************************************************
* Struct for the game board 
********************************************************************************/
struct Board {
    board: [String; 9]
}

/*******************************************************************************
* This is the actual structre called Board, holds all necessary functions 
********************************************************************************/
impl Board {

    /***************************************************************************
    * Reset_Board: resets the board back to original starting number values 
    ****************************************************************************/
    fn reset_board(&mut self) {
        let mut number:usize = 1;
        while number < 10 {
            self.board[number - 1] = number.to_string();
            number += 1;
        }
    }

    /***************************************************************************
    * Update_Board: changes a slot in the board to the desired value
    ****************************************************************************/
    fn update_board(&mut self, choice: String, number: String) {
        let trimmed = number.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => self.board[i-1] = String::from(choice),
            Err(..) => println!("this was not an integer: {}", trimmed),
    };  
    }

    /***************************************************************************
    * Game_Over: Checks the board for win/game ending conditions and returns the
    * results.
    ****************************************************************************/
    fn game_over(&mut self, turn: i8) -> String {
        let player : String;
        if turn % 2 == 0 {
            player = String::from("X");
        } else if turn % 2 == 1 {
            player = String::from("O");
        } else {
            println!("Something is wrong here?");
            player = String::from("?");
        }

        let n:usize = 0;
        while n < 9 {
            if self.board[n] == player && 
                self.board[n+1] == player &&
                self.board[n+2] == player {
                    let str = format!("{} Wins! Thanks for Playing!", player);
                    return String::from(str);
            }
        }

        if (self.board[0] == player && self.board[4] == player && self.board[8] == player) ||
            (self.board[2] == player && self.board[4] == player && self.board[6] == player) {
                let str = format!("{} Wins! Thanks for Playing!", player);
                return String::from(str);
        }

        let mut count : usize = 0;

        while count < 9 {
            if self.board[count] != "X" && self.board[count] != "O" {
                count += 1;
            }
        }

        if count == 0 {
            return String::from("Cats Cradle, IT'S A TIE! Thanks for Playing!");
        }
        return String::from("HI")
    }

    /***************************************************************************
    * Display_board: displays the board
    ****************************************************************************/
    fn display_board(&mut self) {
        print!("{} ", self.board[0]);
        let count : usize = 1;
        while count < 9 {
            if count == 3 || count == 6 {
                println!("");
                println!("-+-+-");
                print!("{} ", self.board[count]);
            } else {
                print!("{} ", self.board[count]);
            }
        }
        println!("");
    }
}

/*******************************************************************************
* Get_Input: gets the users input
********************************************************************************/
fn get_input() -> String{
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    return number;
}

/*******************************************************************************
* Player_Turns: Keeps track of who's turn it is, and runs that turn accordingly.
********************************************************************************/
fn player_turns(turn: i8, b : &mut Board) -> i8 {
    if turn % 2 != 0 {
        print!("X's turn to choose a square (1-9): ");
        let slot : String = get_input();
        b.update_board(String::from("X"), slot)
    } else {
        print!("O's turn to choose a square (1-9): ");
        let slot : String = get_input();
        b.update_board(String::from("O"), slot)
    }
    let turns: i8 = turn + 1;
    print!("\n");
    return turns
}

/*******************************************************************************
* Options: Asks the user if they want to play again once a game is finished. 
********************************************************************************/
fn options() -> String {
    print!("Would you like to play again? y/n > ");
    let mut yes_no = String::new();
    io::stdin()
        .read_line(&mut yes_no)
        .expect("Failed to read line");
    return yes_no
}

/*******************************************************************************
* Main: runs the main loop of the game and initializes the structure. 
********************************************************************************/
fn main() {
    let mut b = Board{
        board: [String::from("1"), String::from("2"), String::from("3"), 
                String::from("4"), String::from("5"), String::from("6"), 
                String::from("7"), String::from("8"), String::from("9")]
    };

    println!("");
    b.display_board();
    let mut end : String = String::from("continue");
    let mut turn : i8 = 1;

    while end == String::from("continue") {
        turn = player_turns(turn, &mut b);
        b.display_board();
        end = b.game_over(turn);
        if end != String::from("continue") {
            println!("{}", end);
            end = options();
            println!("");
            if end == String::from("y") {
                b.reset_board();
                turn = 1;
                end = String::from("continue");
                b.display_board();
            }
        }
    }
    println!("Thanks for Playing!");
}

#[test]
fn should_fail() {
    unimplemented!();
}