# Frog Zone

## Getting Started
Client is managed with bun. Get [bun](https://bun.sh) if you don't have it.
Then, `bun install` in `packages/client`.
Client (`packages/client`) is vite + SWC + ts + rxjs + phaser + react. Run with `bun dev`. Don't forget to add a `.env` with appropriate env variables.


Backend and local client (`packages/phantom-client`, `packages/server`, `packages/worker`) are rust (rocket). Install dependencies, build, and run with `sh start.sh` from `packages`. Test with `sh test.sh`. Sometimes stuff is really finnicky so you might have to try a couple of times.


## Running the Login Server Locally:

The login server controls players logging in, their status and ending the game. You can run it locally or use the live version url.

If running locally, you will need to create a postgress database and add the credentials to `.env` in `packages/login` or ask someone for credentials.å

Login Server (`packages/client`) is next js + react + prisma. Run with `bun dev`
Update `LOGIN_SERVER_URL` to .env to point to your local login server, ex: `http://localhost:3000`
Run `bun dev` in `packages/client`

## Running the Plain text version of the game:

Add IS_MOCK=true to .env in `packages/client`
Rerun `sh start.sh` in  `packages/server`

## Running with Dev features:
Add IS_MOCK=true to .env in `packages/client`, this enables certain dev features like bypassing the login and the TileMapEditor

