# rust-chess

#### The game of chess, in rust ####

###### On the radar: ######
- full game of chess
- GUI
- networking (lan/multicast, internet server/client)
- ai to combat


![screenshot](screenshot.png?raw=true)


basic example:
```rust
let mut game = Game::new();

game.play((1,1),(2,1)); //player white
game.play((6,1),(5,1)); //player black
println!("valid move? {:?}",game.play((0,2),(2,0)));

println!("{:?}",game); //prints out current gameboard layout
```

extended examples in [tests](tests/lib.rs)