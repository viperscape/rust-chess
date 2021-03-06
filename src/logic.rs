use ::{Position,Move};


#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub enum Item {
    Pawn,
    King(bool),
    Queen,
    Rook(bool),
    Knight, 
    Bishop,
    EnPass(Position), //position to original pawn
}

#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
enum PawnMove {
    Single,
    Double,
    Capture,
}

// generic move types, needed at a higher level, so pass this back to Game
#[derive(Debug,Clone,Copy)]
pub enum MoveType {
    Regular,
    Castle, // rook or king castling
    Double(Position), //pawns double contains enpass ghost item's position
    Upgrade, //consider calling this queen, since that's the upgrade for pawn
}

//convenience function
fn abs_pos (from:Position,to:Position) -> Position {
    let r = (to.0 - from.0).abs();
    let c = (to.1 - from.1).abs();
    (r,c)
}

impl Item {
    fn castling_logic  (&self, from:Position, to:Position) -> bool {
        if (to.0 == 0 && from.0 == 0) ||
            (to.0 == 7 && from.0 == 7) {
                if to.1 == 4 { //castling? towards king from rook
                    if from == (0,0) ||
                        from == (0,7) ||
                        from == (7,0) ||
                        from == (7,7)
                    { return true; }
                }
                else if to.1 == 0 || to.1 == 7 { //castling? towards rook from king
                    if from == (0,4) ||
                        from == (7,4)
                    { return true; }
                }
            }

        false
    }

    fn rook_logic (&self, from:Position, to:Position, hasmoved:bool) -> Option<MoveType> {
        if self.castling_logic(from,to) && !hasmoved { Some(MoveType::Castle) }
        else if to.0 == from.0 ||
            to.1 == from.1 {Some(MoveType::Regular)}
        else {None}
    }

    fn king_logic (&self, from:Position, to:Position, hasmoved:bool) -> Option<MoveType> {
        let (r,c) = abs_pos(from,to);

        if self.castling_logic(from,to) && !hasmoved { Some(MoveType::Castle) }
        else if (r == 0 && c == 1) ||
            (r == 1 && c == 0) ||
            (r == 1 && c == 1)
        {Some(MoveType::Regular)}
        else {None}
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

    fn play_isvalid (&self, from:Position, to:Position) -> Option<MoveType> {
        if from == to {return None}
        match *self {
            Item::Queen => { // queen is in essence rook, bishop, and king combined
                //todo: clean this up!
                match self.rook_logic(from,to,true) {
                    Some(MoveType::Regular) => {return Some(MoveType::Regular);},
                    _ => (),
                }
                match self.king_logic(from,to,true) {
                    Some(MoveType::Regular) => {return Some(MoveType::Regular);},
                    _ => (),
                }
                if self.bishop_logic(from,to) {Some(MoveType::Regular)}
                else {None}
            },
            Item::Knight => { if self.knight_logic(from,to) {Some(MoveType::Regular)}
                              else {None} },
            Item::Bishop => { if self.bishop_logic(from,to) {Some(MoveType::Regular)}
                              else {None} },
            _ => None,
        }
    }

    /// internal convenience function
    fn pawn_isvalid (&self, from: Position , to: Position, inverted:bool, capturing:bool) -> Option<MoveType> {
        if let Some(res) = self.pawn_logic(from,to,inverted) {
            match res {
                PawnMove::Single => {
                    if capturing { None }
                    else { Some(MoveType::Regular) }
                },
                PawnMove::Double => {
                    if capturing { None }
                    else if inverted { Some(MoveType::Double((from.0-1, from.1))) }
                    else { Some(MoveType::Double((from.0+1, from.1))) }
                },
                PawnMove::Capture => {
                    if capturing {Some(MoveType::Regular)}
                    else {None}
                },
            }
        }
        else {None}
    }

// pathing

    fn queen_path (&self, from:Position, to:Position) -> Vec<Position> {
        match self.rook_logic(from,to,true) {
            Some(MoveType::Regular) => {return self.rook_path(from,to);},
            _ => (),
        }
        
        if self.bishop_logic(from,to) { return self.bishop_path(from,to) }

        match self.king_logic(from,to,true) {
            Some(MoveType::Regular) => {return vec!(from,to);},
            _ => (),
        }
        vec!()
    }

    fn rook_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();

        // heading down row or column?
        if from.0 != to.0 { //row not same? moving along column
            if from.0 < to.0 {
                for n in (from.0..to.0) { v.push((n,from.1)); }
            }
            else { //reverse
                for n in (to.0..from.0+1).rev() { v.push((n,from.1)); }
            }
        }
        else {
            if from.1 < to.1 {
                for n in (from.1..to.1) { v.push((from.0,n)); }
            }
            else { //reverse
                for n in (to.1..from.1+1).rev() { v.push((from.0,n)); }
            }
        }

        v.push(to);
        v
    }

    fn bishop_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();
        let mut c = from.1;
        
        if from.0 < to.0 {
            for r in (from.0..to.0) {
                v.push((r,c));

                //adjust column
                if from.1 > to.1 { c-=1; }
                else { c+=1; }
            }
        }
        else {
            for r in (to.0..from.0+1).rev() { //offset, rev range is tricky
                v.push((r,c));

                //adjust column
                if from.1 > to.1 { c-=1; }
                else { c+=1; }
            }
        }

        v.push(to);
        v
    }

    /// gets play path, to be checked later for if legal
    fn play_path (&self, from:Position, to:Position) -> Vec<Position> {
        match *self {
            Item::Queen => self.queen_path(from,to),
            Item::Rook(_) =>  self.rook_path(from,to),
            Item::Bishop => self.bishop_path(from,to),
            _ => vec!(from,to), //single space destination
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Player {
    Black(Item),
    White(Item),
}

impl Player {
    /// check play logic for valid moves
    /// anonymous matches on enums would be helpful in removing duplicated code
    pub fn play_isvalid (&self, from: Position , to: Position, capturing: bool) -> Option<MoveType> {
        match *self {
            Player::Black(item) => {
                match item {
                    Item::Pawn => { //pawn logic is special
                        item.pawn_isvalid(from,to,true,capturing)
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
                        item.pawn_isvalid(from,to,false,capturing)
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

    /// special pathing for castling, returns tuple of new positions for (king,rook)
    pub fn castle_path (&self, from:Position, to:Position) -> Move {
        match *self {
            Player::Black(item) => {
                match item {
                    Item::Rook(_) => {
                        if from.1 == 7 { ((7,6),(7,5)) }
                        else { ((7,2),(7,3)) }
                    },
                    _ => {
                        if to.1 == 7 { ((7,6),(7,5)) }
                        else { ((7,2),(7,3)) }
                    }
                }
            },
            Player::White(item) => {
                match item {
                    Item::Rook(_) => {
                        if from.1 == 7 { ((0,6),(0,5)) }
                        else { ((0,2),(0,3)) }
                    },
                    _ => {
                        if to.1 == 7 { ((0,6),(0,5)) }
                        else { ((0,2),(0,3)) }
                    }
                }
            },
        }
    }
}
