# Frog Zone

## Getting Started
This is a bun monorepo. Get [bun](https://bun.sh) if you don't have it
Then, `bun install`
Client (`packages/client`) is vite + SWC + ts + rxjs + phaser + react. Run with `bun dev`
Server (`packages/server`) is rust (rocket). Run with `cargo run`

## Git Stuff
- rebase don't merge
- [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) within PRs, and then squash and merge to main

## Notes
- right now server responds with all events on `/state`, it should only respond with events after a certain id passed by client