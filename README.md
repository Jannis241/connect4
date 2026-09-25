# Connect4

Connect Four in the terminal, written in Rust, with a bot you can play against.

You play as **Red (R)** against the bot, which plays as **Yellow (Y)**. Who starts is chosen randomly.

## Run

```sh
cargo run --release
```

## How to play

Enter a number from 1 to 7 to choose a column.

```
 1 2 3 4 5 6 7
┌───────────────┐
|. . . . . . . |
|. . . . . . . |
|. . . . . . . |
|. . . Y . . . |
|. . R Y . . . |
|. R R Y R . . |
└───────────────┘
```

## How the bot works

The bot (`src/bot.rs`) uses the minimax algorithm with a search depth of 5.

**Looking ahead (`minimax`):** The bot tries out all possible move sequences up to 5 moves ahead. It always assumes that both players play their best move.

**Rating a position (`evaluate`):** After 5 moves, the bot looks at every possible line of four on the board and gives points:

| The line contains | Points |
|---|---|
| 3 own pieces, no opponent pieces | +5 |
| 2 own pieces, no opponent pieces | +2 |
| 3 opponent pieces, no own pieces | −5 |
| 2 opponent pieces, no own pieces | −2 |

A win counts much more than all of these. If the bot can win, it takes the fastest win.

### Limitations

- The bot can't see traps that are more than 5 moves away.
- With alpha-beta pruning it could calculate faster and look further ahead.

## What I learned

- How minimax works and how to write it recursively
- How to rate a game position with a simple scoring function
- Really useful for building bots in general and for other games like chess.

