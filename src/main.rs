// use std::vec;

struct Piece {
    cells: Vec<Vec<i8>>,
}

fn build_piece(c: Vec<Vec<i8>>) -> Piece {
    Piece {
        cells: c,
    }
}

fn print_piece(p: Piece) {
    for r in p.cells {
        for c in r {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let p = build_piece(vec![vec![0,1], vec![0,1]]);
    print_piece(p);
}
