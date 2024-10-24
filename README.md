# Frog Zone

## Getting Started

Client is managed with bun. Get [bun](https://bun.sh) if you don't have it.
Then, `bun install` in `packages/client`.
Client (`packages/client`) is vite + SWC + ts + rxjs + phaser + react. Run with `bun dev`. Don't forget to add a `.env` with appropriate env variables.


Backend and local client (`packages/phantom-client`, `packages/server`, `packages/worker`) are rust (rocket). Install dependencies, build, and run with `sh start.sh` from `packages`. Test with `sh test.sh`. Sometimes stuff is really finnicky so you might have to try a couple of times.
