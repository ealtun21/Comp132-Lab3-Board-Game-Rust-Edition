use grid::*;
use rand::Rng;
use std::cmp::Ordering;

fn read<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(requested_input) = input.trim().parse::<T>() {
            return requested_input;
        }
        println!("Please enter a correct value!");
    }
}

struct Player {
    points: i32,
    name: String,
    mistake: i32,
}

fn display_board(board: &Grid<i32>) {
    println!("Board:");
    for i in 0..board.rows() {
        for row in board.iter_row(i) {
            print!("{row} ");
        }
        println!();
    }
}

fn is_zero(board: &Grid<i32>) -> bool {
    board.iter().find(|x| **x != 0).is_none()
}

fn main() {
    println!("Let's play the Board Game!");
    println!("Please determine the board dimension: ");
    let dim = read();

    let mut player: Vec<Player> = vec![];
    println!("Player 1, what is your name?");
    player.push(Player {
        points: 0,
        name: read(),
        mistake: 3,
    });
    println!("Player 2, what is your name?");
    player.push(Player {
        points: 0,
        name: read(),
        mistake: 3,
    });

    //Board shown to player
    let mut board: Grid<i32> = Grid::new(dim, dim);
    board.iter_mut().for_each(|x| *x = -1);

    //Board for calcing score.
    let mut real_board: Grid<i32> = Grid::new(dim, dim);
    real_board
        .iter_mut()
        .for_each(|x| *x = rand::thread_rng().gen_range(1, 10));

    loop {
        for i in 0..2 {
            println!("{}'s Turn: ", player[i].name);
            display_board(&board);
            println!("Enter a row number: ");
            let mut row: usize = read();
            println!("Enter a column number: ");
            let mut column: usize = read();

            while board.get(row, column).is_none() {
                println!("This position is out of the bounds of the board!");
                println!("Enter a row number: ");
                row = read();
                println!("Enter a column number: ");
                column = read();
            }

            if board[row][column] != 0 {
                board[row][column] = 0;
                player[i].points += real_board[row][column];
            } else {
                println!("\nThis position was already used.");
                player[i].mistake -= 1;
                if player[0].mistake <= 0 || player[1].mistake <= 0 {
                    println!("{} has run out of changes!", player[i].name);
                    break;
                }
                println!(
                    "{} has {} input changes left.\n",
                    player[i].name, player[i].mistake
                );
            }
        }
        if is_zero(&board) || player[0].mistake <= 0 || player[1].mistake <= 0 {
            break;
        }
    }

    println!("----------------------- Result ---------------------------");
    println!(
        "\nThe player {} has the score of {} and the player {} has the score of {}.\n",
        player[0].name, player[0].points, player[1].name, player[1].points
    );
    if player[0].mistake > 0 && player[1].mistake > 0 {
        match player[0].points.cmp(&player[1].points) {
            Ordering::Equal => println!("It's a tie."),
            Ordering::Less => println!("{} is the winner!!", player[1].name),
            Ordering::Greater => println!("{} is the winner!!", player[0].name),
        }
    } else if player[0].mistake <= 0 {
        println!("{} made too many mistakes :(", player[0].name);
        println!("{} is the winner!!", player[1].name);
    } else if player[1].mistake <= 0 {
        println!("{} made too many mistakes :(", player[1].name);
        println!("{} is the winner!!", player[0].name);
    } else {
        println!("It's a tie.");
    }

    println!("--- The final status of the visible board is: ---");
    display_board(&board);

    println!("--- The hidden board was: ---");
    display_board(&real_board);
}
