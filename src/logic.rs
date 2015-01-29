#[derive(Show,Copy)]
pub enum Item {
    Pawn,
    King(bool),
    Queen,
    Rook(bool),
    Knight, 
    Bishop,
    EnPass,
}

enum PawnMove {
    Single,
    Double,
    Capture,
}

pub type Position = (usize,usize); //change to u8 when rust gets changed!
fn abs (v: usize) -> usize { //wtf happened to std abs? also change to u8 soon
    let v = v as i32;
    if v < 0i32 {
        (v * -1) as usize
    } 
    else {v as usize}
}
fn abs_pos (from:Position,to:Position) -> (usize,usize) {
    let r = abs(to.0 - from.0);
    let c = abs(to.1 - from.1);
    (r,c)
}

impl Item {
    fn castling_logic  (&self, from:Position, to:Position) -> bool {
        if to.0 != 0 { return false; } //not on home row?
        if to.1 == 4 { //castling? to king
            if from == (0,0) ||
                from == (0,7) { return true; }
        }
        else if to.1 == 0 || to.1 == 7 { //castling? to rook
            if from == (7,0) ||
                from == (7,7) { return true; }
        }

        false
    }

    fn rook_logic (&self, from:Position, to:Position, hasmoved:bool) -> bool {
        if to.0 == from.0 ||
            to.1 == from.1 {true}
        else if self.castling_logic(from,to) && !hasmoved {true}
        else {false}
    }

    fn king_logic (&self, from:Position, to:Position, hasmoved:bool) -> bool {
        let (r,_) = abs_pos(from,to);

        if r < 2 &&
            r < 2 {true}
        else if self.castling_logic(from,to) && !hasmoved {true}
        else {false}
    }

    fn bishop_logic (&self, from:Position, to:Position) -> bool {
        let (r,c) = abs_pos(from,to);

        if r == c {true}
        else {false}
    }

    fn pawn_logic (&self, from:Position, to:Position, inverted:bool) -> Option<PawnMove> {
        let (_,c) = abs_pos(from,to);
        if !inverted {
            if to.0 - from.0 == 1 &&
                c == 1 {Some(PawnMove::Capture)}  // diagonally on capture
            else if to.0 - from.0 == 2 && c == 0 && 
                from.0 == 1 {Some(PawnMove::Double)}
            else if to.0 - from.0 == 1 &&
                c == 0 {Some(PawnMove::Single)} //forward, 1
            else {None}
        }
        else { //black is playing
            if from.0 - to.0 == 1 &&
                c == 1 {Some(PawnMove::Capture)}  // diagonally on capture
            else if from.0 - to.0 == 2 && c == 0 && 
                from.0 == 6  {Some(PawnMove::Double)}
            else if from.0 - to.0 == 1 &&
                c == 0 {Some(PawnMove::Single)} //forward, 1
            else {None}
        }
    }

    fn knight_logic (&self, from:Position, to:Position) -> bool {
        let (r,c) = abs_pos(from,to);

        if r < 1 || c < 1 ||
            r > 2 || c > 2 {false}
        else { 
            if r == 2 && c == 1 {true}
            else if r == 1 && c == 2 {true}
            else {false}
        }
    }

    fn play_isvalid (&self, from:Position, to:Position) -> bool {
        if from == to {return false}
        match *self {
            Item::Queen => { // queen is in essence rook, bishop, and king combined
                if self.rook_logic(from,to,true) ||
                    self.bishop_logic(from,to) ||
                    self.king_logic(from,to,true) {true}
                else {false}
            },
            Item::Knight => self.knight_logic(from,to),
            Item::Bishop => self.bishop_logic(from,to),
            _ => false,
        }
    }

    fn queen_path (&self, from:Position, to:Position) -> Vec<Position> {
        if self.rook_logic(from,to,true) { self.rook_path(from,to) }
        else if self.bishop_logic(from,to) { self.bishop_path(from,to) }
        else { vec!(to) }
    }

    fn rook_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();

        // heading down row or column?
        if from.0 != to.0 {
            for n in range(from.0,to.0) { v.push((n,from.1)) }
        }
        else {
            for n in range(from.1,to.1) { v.push((from.0,n)) }
        }

        v
    }

    fn bishop_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();
        let mut m = from.1;

        for n in range(from.0,to.0) {
            if from.1 > to.1 { m-=1; }
            else { m+=1; }
            v.push((n,m));
        }

        v
    }

    /// gets play path, to be checked later for if legal
    fn play_path (&self, from:Position, to:Position) -> Vec<Position> {
        match *self {
            Item::Queen => self.queen_path(from,to),
            Item::Rook(_) =>  self.rook_path(from,to),
            Item::Bishop => self.bishop_path(from,to),
            _ => vec!(to), //single space destination
        }
    }
}

#[derive(Show,Copy)]
pub enum Player {
    Black(Item),
    White(Item),
}

impl Player {
    /// check play logic for valid moves
    pub fn play_isvalid (&self, from: Position , to: Position, capturing: bool) -> bool {
        match *self {
            Player::Black(item) => {
                match item {
                    Item::Pawn => { //pawn logic is special
                        if let Some(res) = item.pawn_logic(from,to,true) {
                            match res {
                                PawnMove::Single | PawnMove::Double => true,
                                PawnMove::Capture => {
                                    if capturing {true}
                                    else {false}
                                },
                            }
                        }
                        else {false}
                    },
                    Item::King(hasmoved) => { //king logic is special
                        item.king_logic(from,to,hasmoved)
                    },
                    Item::Rook(hasmoved) => { //rook logic is special
                        item.rook_logic(from,to,hasmoved)
                    },
                    _ => item.play_isvalid(from,to),
                }
            }
            Player::White(item) => {
                match item {
                    Item::Pawn => {
                        if let Some(res) = item.pawn_logic(from,to,true) {
                            match res {
                                PawnMove::Single | PawnMove::Double => true,
                                PawnMove::Capture => {
                                    if capturing {true}
                                    else {false}
                                },
                            }
                        }
                        else {false}
                    },
                     Item::King(hasmoved) => { //king logic is special
                        item.king_logic(from,to,hasmoved)
                    },
                    Item::Rook(hasmoved) => { //rook logic is special
                        item.rook_logic(from,to,hasmoved)
                    },
                    _ => item.play_isvalid(from,to),
                }
            }
        }
    }

    pub fn play_path (&self, from: Position , to: Position) -> Vec<Position> {
        match *self {
            Player::Black(item) => item.play_path(from,to),
            Player::White(item) => item.play_path(from,to),
        }
    }
}
