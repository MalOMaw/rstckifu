use std::env;
use walkdir::DirEntry;
use walkdir::WalkDir;
use std::fs;

fn main() {
    let mut board: [u8;19*19] = [b'e';19*19];
    println!("Here is it: {}", board[21] as char);
    let args: Vec<String> = env::args().collect();
    let input_directory = &args[1];
    //let output_directory = &args[2];
    let walker = WalkDir::new(input_directory).into_iter();
    let mut i = 1;
    for entry in walker {
        let entry = entry.unwrap();
        if is_sgf(&entry) {
            let mut EV: &str = "NaN";
            let mut PB: &str;
            let mut PW: &str;
            let mut WR: &str;
            let mut KM: f32;
            let mut RE: &str;
            let mut DT: &str;
            let mut black_moves: Vec<[u8; 2]>;
            let mut white_moves: Vec<[u8; 2]>;
            let contents = fs::read_to_string(entry.path()).expect("Weird, you should have all access you need.");
            println!("Here is kifu: \n\n{contents}");
            break
        }
    }
}

//test change

fn move_add(board: &mut [u8], mv: &[u8; 2], is_black: bool) {
    // Cast ASCII into alphabet number, which will correspond to actuall coordinates.
    let x = mv[0] - 96;
    let y = mv[1] - 96;
    let bsize = board.len();
    let index: usize = x as usize * bsize + y as usize;
    board[index] = if is_black {b'w'} else {b'b'};
}

fn is_sgf(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with(".sgf"))
         .unwrap_or(false)
}