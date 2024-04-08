struct Piece {
    cells: Vec<Vec<i8>>,
}

struct PieceClass {
    pieces: Vec<Piece>,
    id: i8,
}

struct PieceSet {
    classes: Vec<PieceClass>,
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

fn build_piece_class(p: Piece, id: i8) ->PieceClass {
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
            id: id,
        }

}

impl PieceClass {
    fn print(&self) {
        println!("PieceClass: {}", self.id);
        for r in &self.pieces {
            r.print();
        }
    }
}

fn set_bit(bitmask: i64, x: i8, y: i8) -> i64{
    let pos = x + y*7;
    return  bitmask | (1<<pos); 
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

        for i in 0..cells.len() {
            cells[i].reverse();
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

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                if self.cells[i][j] != p.cells[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn to_bitmask(&self, x: i8, y: i8) -> i64 {

        let sx = self.cells[0].len() as i8;
        let sy = self.cells.len() as i8;

        if sx + x > 7 {
            return -1;
        }

        if sy  + y > 7 {
            return -1;
        }
        let mut bitmask: i64 = 0;
        for i in 0..sy {
            for j in  0..sx {
                if self.cells[i as usize][j as usize] != 0 {
                    bitmask = set_bit(bitmask, j+x, y+i);
                }
            }
        }

        return bitmask;
    }

}

fn build_piece_set() -> PieceSet {
    let mut classes: Vec<PieceClass> =  Vec::new();

    let mut p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![1,0],
            vec![1,0]
            ]);
    classes.push(build_piece_class(p, 1));

    p = build_piece(
        vec![
            vec![1,1],
            vec![1,0],
            vec![1,0],
            vec![1,0]
            ]);
    classes.push(build_piece_class(p, 2));
    
    p = build_piece(
        vec![
            vec![1,1],
            vec![1,0],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 3));
    
    p = build_piece(
        vec![
            vec![1,1,0],
            vec![0,1,0],
            vec![0,1,1],
            ]);
    classes.push(build_piece_class(p, 4));

    p = build_piece(
        vec![
            vec![1,1],
            vec![1,1],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 5));
    
    p = build_piece(
        vec![
            vec![1,0,0],
            vec![1,0,0],
            vec![1,1,1],
            ]);
    classes.push(build_piece_class(p, 6));
    
    p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![0,1],
            vec![0,1]
            ]);
    classes.push(build_piece_class(p, 7));
    
    p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 8));

    PieceSet {
        classes: classes,
    }

}

impl PieceSet {
    fn print(&self) {
        for pc in &self.classes {
            pc.print();
        }
    }

    fn iterate(&self, class: i8, state: i64, finish: i64) -> bool {
        if (class as usize) > self.classes.len() {
            return false;
        }

        for piece in &self.classes[class as usize].pieces {
            for x in 0..7 {
                for y in 0..7 {
                    let mask = piece.to_bitmask(x, y);
                    if mask < 0 {
                        continue;
                    }
                    if mask & state != 0 {
                        continue;
                    }
                    let new_state = mask | state;
                    if new_state == finish {
                        println!("Solution found!");
                        println!("at {}:{}", x, y);
                        piece.print();
                        return true;
                    }
                    if self.iterate(class+1, new_state, finish) {
                        println!("at {}:{}", x, y);
                        piece.print();
                        return true;
                    }
                }
            }
        }

        return false;

    }

    fn solve(&self, state: i64, finish: i64) {
        if self.iterate(0, state, finish) {
            println!("found");
        } else {
            println!("not found");
        }
    }
}

fn initial_state() -> i64 {
    let mut mask: i64 = 0;
    mask = set_bit(mask, 6, 0);
    mask = set_bit(mask, 6, 1);

    mask = set_bit(mask, 3,6);
    mask = set_bit(mask, 4,6);
    mask = set_bit(mask, 5,6);
    mask = set_bit(mask, 6,6);

    //23 Feb
    mask = set_bit(mask, 1,0);
    mask = set_bit(mask, 1,5);

    return mask;
}

fn final_state() -> i64 {
    return i64::pow(2, 49) - 1;
}


fn main() {
    let ps = build_piece_set();

    let state = initial_state();
    let finish = final_state();
    println!("from {:b} to {:b}", state, finish);

    ps.solve(state,finish)
}
