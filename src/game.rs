use super::{Player,Item,Position};

pub type BoardLayout = [[Option<Player>;8];8];

#[derive(Show)]
pub struct Game {
    board: BoardLayout,
    captured: Vec<Player>,
    active: Player, //use a fake piece to set who is current active player
}

impl Game {
    pub fn new() -> Game  {
        let mut board = [[None;8];8];

        //setup pawns row
        board[1] = [Some(Player::White(Item::Pawn));8];
        board[6] = [Some(Player::Black(Item::Pawn));8];

        //setup home row
        board[0] = [Some(Player::White(Item::Rook)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Queen)),
                    Some(Player::White(Item::King)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Rook))];

        board[7] = [Some(Player::Black(Item::Rook)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Queen)),
                    Some(Player::Black(Item::King)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Rook))];
        Game { board:board, captured:Vec::new(), active: Player::White(Item::Pawn) }
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

    pub fn play(&mut self, from:Position,to:Position) -> bool {
        println!("{:?}",self.get_player(from));
        println!("{:?}",self.get_player(to));

        if let &Some(player) = self.get_player(from) {

            //current active player is playing?
            match (player,self.active) {
                (Player::White(_),Player::Black(_)) | 
                (Player::Black(_),Player::White(_)) => return false,
                _ => (),
            }
            
            //only capturing other players pieces?
            if let &Some(oppo) =  self.get_player(to) {
                match (player, oppo) {
                    (Player::White(_),Player::White(_)) | 
                    (Player::Black(_),Player::Black(_)) => return false,
                    _ => (),
                }
            }


            if player.play_isvalid(from,to, self.capturing(from,to)) {
                let path = player.play_path(from,to).pop(); //get path and remove dest

                let res = path.iter().find(|&n| self.get_player(*n).is_some());

                if res.is_some() { 
                    println!("blocked! {:?}",res); 
                    return false 
                }

                if let Some(_p) = self.swap_pos(to,Some(player)) {
                    println!("captured {:?}",_p);
                    self.captured.push(_p);
                }
                self.swap_pos(from,None);

                return true
            }
        }
        false
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
}
