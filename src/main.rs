// use std::vec;

struct Piece {
    cells: Vec<Vec<i8>>,
}

struct PieceClass {
    pieces: Vec<Piece>,
}

fn build_piece(c: Vec<Vec<i8>>) -> Piece {
    Piece {
        cells: c,
    }
}

fn has_piece( ps: &Vec<Piece>, p: &Piece) -> bool {
    for i in ps {
        if i.equal(p) {
            return true;
        }
    }
    false
}

fn build_piece_class(p: Piece) ->PieceClass {
        let mut pieces: Vec<Piece> =  Vec::new();

        let p1 = p.rotate();
        let p2 = p1.rotate();
        let p3 = p2.rotate();

        let pf = p.flip();
        let pf1 = pf.rotate();
        let pf2 = pf1.rotate();
        let pf3 = pf2.rotate();

        pieces.push(p);
        if !has_piece(&pieces, &p1) {
            pieces.push(p1)
        }
        if !has_piece(&pieces, &p2) {
            pieces.push(p2)
        }
        if !has_piece(&pieces, &p3) {
            pieces.push(p3)
        }
        if !has_piece(&pieces, &pf) {
            pieces.push(pf)
        }
        if !has_piece(&pieces, &pf1) {
            pieces.push(pf1)
        }
        if !has_piece(&pieces, &pf2) {
            pieces.push(pf2)
        }
        if !has_piece(&pieces, &pf3) {
            pieces.push(pf3)
        }

        PieceClass {
            pieces: pieces,
        }

}

impl PieceClass {
    fn print(&self) {
        for r in &self.pieces {
            r.print();
        }
    }
}

impl Piece {
    fn print(&self) {
        for r in &self.cells {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }
    
    fn rotate(&self) -> Piece {
        let mut i = 0;
        let mut cells: Vec<Vec<i8>> =  Vec::new();
        for r in &self.cells {
            let mut j = 0;
            for c in r {
                if i == 0 {
                    cells.push(Vec::new())
                }
                cells[j].push(*c);
                j += 1;
            }
            i += 1;
        };

        i = 0;
        while i<cells.len() {
            cells[i].reverse();
            i += 1;
        }

        build_piece(cells)
    }

    fn flip(&self) -> Piece {
        let mut cells: Vec<Vec<i8>> =  Vec::new();
        let mut i = self.cells.len()-1;

        loop {
            cells.push(self.cells[i].clone());
            if i ==0 {
                break;
            }
            i -= 1;
        }
        build_piece(cells)
    }

    fn equal(&self, p: &Piece) -> bool {
        if self.cells.len() != p.cells.len() {
            return false;
        }

        if self.cells[0].len() != p.cells[0].len() {
            return false;
        }

        let mut i = 0;

        while i < self.cells.len() {
            let mut j = 0;
            while j < self.cells[0].len() {
                if self.cells[i][j] != p.cells[i][j] {
                    return false;
                }
                j += 1;
            }
            i += 1;
        }
        return true;
    }

}

fn main() {
    let p = build_piece(vec![vec![1,1,0], vec![0,1,0],vec![0,1,1]]);
    let c = build_piece_class(p);
    c.print();
}
