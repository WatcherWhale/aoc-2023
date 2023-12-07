mod input;
mod game;

use self::input::read_file;
use self::game::Game;

fn main() {
    let lines: Vec<String> = read_file();

    let mut sum : i32 = 0;

    for line in lines {
        // I know creating a struct for the parsing is not needed, I just wanted to use it
        // I also like how clean it looks in the main function :-)
        let game = Game::from(line);

        if game.is_possible(12, 13, 14) {
            sum += game.id
        }
    }

    println!("{}", sum);
}
