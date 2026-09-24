# Connect4

Vier gewinnt im Terminal, programmiert in Rust.

Du spielst als **Rot (R)** gegen einen Bot als **Gelb (Y)**. Wer anfängt, wird zufällig bestimmt.

## Starten

```sh
cargo run --release
```

## Spielen

Man kann die Nummern 1-7 eingeben um eine Spalte auszuwählen.

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

## Bot

Der Bot (`src/bot.rs`) nutzt Minimax mit einer Tiefe von 5 und einer einfachen Bewertung.

### Wie er funktioniert

**Vorausdenken (`minimax`):** Der Bot probiert alle Zugfolgen bis zu 5 Züge im Voraus durch. Er geht immer vom besten Move aus.

**Stellung bewerten (`evaluate`):** Nach 5 Zügen zählt der Bot alle möglichen vierer Reihen auf dem Brett:

| Reihe enthält | Punkte |
|---|---|
| 3 eigene, keine gegnerischen | +5 |
| 2 eigene, keine gegnerischen | +2 |
| 3 gegnerische, keine eigenen | −5 |
| 2 gegnerische, keine eigenen | −2 |

### Grenzen
- Fallen, die weiter als 5 Züge entfernt sind, sieht er nicht
- Mit Alpha-Beta-Pruning könnte er schneller und tiefer rechnen
