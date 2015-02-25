use super::{Player,Item,Position,MoveType,Move,Capture,BoardLayout};

#[derive(Debug)]
pub struct Game {
    board: BoardLayout,
    captured: Vec<Player>,
    active: Player, //use a fake piece to set who is current active player, we match against this
    check: Option<Player>, //which player is in check
    id: u64,
}

impl Game {
    pub fn build_board() -> BoardLayout{
        let mut board = [[None;8];8];

        //setup pawns row
        board[1] = [Some(Player::White(Item::Pawn));8];
        board[6] = [Some(Player::Black(Item::Pawn));8];

        //setup home row
        board[0] = [Some(Player::White(Item::Rook(false))),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Queen)),
                    Some(Player::White(Item::King(false))),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Rook(false)))];

        board[7] = [Some(Player::Black(Item::Rook(false))),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Queen)),
                    Some(Player::Black(Item::King(false))),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Rook(false)))];

        board
    }

    pub fn new() -> Game  {
        let board = Game::build_board();
        Game { board:board, captured:Vec::new(), 
               active: Player::White(Item::Pawn),
               check: None,
               id: 0 }
    }

    pub fn start (&mut self, id:u64) {
        self.id = id;
    }

    pub fn is_started (&mut self) -> bool {
        if self.id > 0 { true }
        else {false}
    }

    pub fn get_player (&self,at:Position) -> &Option<Player> {
        &self.board[at.0][at.1]
    }

    /// swap out destination, and return original
    fn swap_pos (&mut self,at:Position, p:Option<Player>) -> Option<Player> {
        let oldp;
        if let &Some(_p) = self.get_player(at) {
            oldp = Some(_p);
        }
        else {oldp=None;}
        self.board[at.0][at.1] = p;
        oldp
    }

    pub fn play (&mut self, from:Position,to:Position) -> Result<MoveValid,MoveIllegal> {
        if let &Some(player) = self.get_player(from) { //must select an actual piece

            let _board = Box::new(self.board); //copy original board, to swap back to if needed

            //current active player is playing?
            match (player,self.active) {
                (Player::White(_),Player::Black(_)) | 
                (Player::Black(_),Player::White(_)) => return Err(MoveIllegal::Invalid),
                _ => (),
            }

            // is play logically validated?
            if let Some(mt) = player.play_isvalid(from,to, self.capturing(from,to)) {
                //only "to" other player's pieces, or nothing at all? unless castling!
                if let &Some(oppo) = self.get_player(to) {
                    match mt {
                        MoveType::Castle => (),
                        _ => {
                            match (player, oppo) {
                                (Player::White(_),Player::White(_)) | 
                                (Player::Black(_),Player::Black(_)) => return Err(MoveIllegal::Invalid),
                                _ => (),
                            }
                        }
                    }
                }


                let path = player.play_path(from,to).pop(); //get path and remove dest
                let res = path.iter().find(|&n| self.get_player(*n).is_some());

                if res.is_some() { //blocked
                    return Err(MoveIllegal::Blocked(*res.unwrap()));
                }


                // determine capture
                let _cap: Option<Capture>; //capture result to return
                match mt {
                    MoveType::Castle => _cap = None,
                    _ => {
                        if let Some(_p) = self.swap_pos(to,Some(player)) { //captured
                            let _item = match _p { //I should consider an unwrap fn
                                Player::White(item) => item,
                                Player::Black(item) => item,
                            };
                            match _item {
                                Item::EnPass(pos) => {
                                    let _p = self.get_player(pos).unwrap();
                                    _cap = Some((_p,pos));
                                    self.captured.push(_p);
                                },
                                _ => {
                                    _cap = Some((_p,to));
                                    self.captured.push(_p);
                                },
                            }
                        }
                        else { _cap = None; }
                    }
                }


                // movetypes need different handling for board mutation
                match mt {
                    MoveType::Castle => {
                        let (kp,rp) = player.castle_path(from,to);
                        self.swap_pos(from,None);
                        self.swap_pos(to,None);

                        match player {
                            Player::White(_) => {
                                self.swap_pos(kp,Some(Player::White(Item::King(true))));
                                self.swap_pos(rp,Some(Player::White(Item::Rook(true))));
                            },
                            _ => {
                                self.swap_pos(kp,Some(Player::Black(Item::King(true))));
                                self.swap_pos(rp,Some(Player::Black(Item::Rook(true))));
                            }
                        }
                    },
                    MoveType::Double(pos) => { 
                        //swap in the enpass ghost
                        match player {
                            Player::White(_) => {self.swap_pos(pos,Some(Player::White(Item::EnPass(to))));},
                            Player::Black(_) => {self.swap_pos(pos,Some(Player::White(Item::EnPass(to))));},
                        }
                        self.swap_pos(from,None);
                    },
                    MoveType::Upgrade => {
                        match player {
                            Player::White(_) => {self.swap_pos(to,Some(Player::White(Item::Queen)));},
                            Player::Black(_) => {self.swap_pos(to,Some(Player::Black(Item::Queen)));},
                        }
                        self.swap_pos(from,None);
                    },
                    MoveType::Regular => { //clear the space it came from
                        self.swap_pos(from,None);
                    },
                }


                //must clear out all en passant ghost holders for opposing side, we had our chance
                for r in self.board.iter_mut() {
                    for c in r.iter_mut() {
                        if let Some(p) = *c {
                            match (p,self.active) {
                                (Player::White(item),Player::Black(_)) => {
                                    match item {
                                        Item::EnPass(_) => {*c = None;},
                                        _ => (),
                                    }
                                }
                                (Player::Black(item),Player::White(_)) => {
                                    match item {
                                        Item::EnPass(_) => {*c = None;},
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                            
                        }
                    }
                }

                
                //look for potential king checks
                //start with collecting the two kings' positions
                let kings = self.get_kings();
                //iter thru and validate checks
                let rkme: (Position,Option<Position>);
                let rkthem: (Position,Option<Position>);
                match (self.get_player(kings[0]).unwrap(),self.active) {
                    (Player::Black(_),Player::Black(_)) |
                    (Player::White(_),Player::White(_)) => { 
                        rkme = (kings[0],self.check_isvalid(kings[0]));
                        rkthem = (kings[1],self.check_isvalid(kings[1])); 
                    }
                    _ => {
                        rkme = (kings[1],self.check_isvalid(kings[1])); 
                        rkthem = (kings[0],self.check_isvalid(kings[0])); 
                    }
                }

                if let Some(check) = rkme.1 { //put myself in check?
                    self.board = *_board; // swap back original board
                    if _cap.is_some() { self.captured.pop(); } //pop off captured player if needed
                    return Err(MoveIllegal::Check(check,rkme.0));
                }
                else { //move is totally valid
                    //flip active player
                    match self.active {
                        Player::Black(item) => {self.active = Player::White(item);},
                        Player::White(item) => {self.active = Player::Black(item);},
                    }

                    if let Some(check) = rkthem.1 { 
                        self.check = Some(self.get_player(rkthem.0).unwrap());
                        return Ok(MoveValid { item: player,
                                              cap: _cap,
                                              check: Some((check,rkthem.0)),
                                              mt: mt,
                                              mv: (from,to) });
                    }
                    else { 
                        self.check = None;
                        return Ok(MoveValid { item: player,
                                              cap: _cap,
                                              check: None,
                                              mt: mt,
                                              mv: (from,to) }); 
                    }
                }
            }
        }
        
        Err(MoveIllegal::Invalid)
    }

    fn capturing (&self, from: Position, to: Position) -> bool {
        if let &Some(p) = self.get_player(to) {
            let res = match (p,self.get_player(from).unwrap()) {
                (Player::Black(_),Player::White(_)) => true,
                (Player::White(_),Player::Black(_)) => true,
                _ => false,
            };
            return res
        }
        false
    }

    /// get kings' positions
    fn get_kings (&self) -> Vec<Position> {
        let mut kings: Vec<Position> = vec!();

        for (i,r) in self.board.iter().enumerate() { //collect kings positions
            for (j,c) in r.iter().enumerate() {
                if let Some(_p) = *c {
                    match _p {
                        Player::White(Item::King(_)) |
                        Player::Black(Item::King(_)) => kings.push((i,j)),
                        _ => (),
                    }
                }
            }
            if kings.len() == 2 {break}
        }
        kings
    }

    fn check_isvalid (&self, king: Position) -> Option<Position> {
        for (i,r) in self.board.iter().enumerate() {
            for (j,c) in r.iter().enumerate() {
                if let Some(p) = *c {
                    
                    match (p,self.active) {
                        (Player::White(_),Player::White(_)) |
                        (Player::Black(_),Player::Black(_))  => (),
                        _ => {
                            if (i,j) == king { break; } //exclude kings

                            let res = p.play_isvalid((i,j),king,true);

                            if let Some(mt) = res { 
                                match mt {
                                    MoveType::Regular => { 
                                        let path = p.play_path((i,j),king).pop();
                                        let res = path.iter().find(|&n| self.get_player(*n).is_some());
                                        if !res.is_some() { //not blocked
                                            return Some((i,j)); 
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

}

#[derive(Debug,Copy)]
pub struct MoveValid {
    pub item: Player,
    pub cap: Option<Capture>,
    pub check: Option<Move>, //from piece and to king
    pub mt: MoveType,
    pub mv: Move, //from,to; for render
}

#[derive(Debug,Copy)]
pub enum MoveIllegal {
    Blocked(Position),
    Check(Position,Position), //from piece and to king
    Invalid, //not a valid move, according to logic
}
