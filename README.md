# rust-chess

#### The game of chess, in rust ####

###### On the radar: ######
- full game of chess
- GUI
- networking (lan/multicast, internet server/client)
- game validation/anti cheat
- ai to combat

basic example:
```rust
let mut game = Game::new();

game.play((1,1),(2,1)); //player white
game.play((6,1),(5,1)); //player black
println!("valid move? {:?}",game.play((0,2),(2,0)));

println!("{:?}",game); //prints out current gameboard layout
```

[detailed example](https://github.com/viperscape/rust-chess/blob/master/src/main.rs)
