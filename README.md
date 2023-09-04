# Chers

Chers is a chess engine written in Rust.

The engine uses:

- BitBoards to represent the board (although not yet using their potential for ray generation)
- the UCI protocol to communicate with the GUI
- iterative deepening as a time management strategy and base for the alpha-beta search
- captures first move ordering

## Resources / Credits

- Chers is heavily inspired by [rustic](https://github.com/mvanthoor/rustic), which also helped with its great [wiki](https://rustic-chess.org/)
- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Chess programming Reddit](https://www.reddit.com/r/chessprogramming/)
