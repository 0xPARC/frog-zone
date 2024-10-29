#!/bin/sh

log() {
    if hash node 2>/dev/null; then
        echo "$1"
        node -e 'console.log.call(console, "   ", util.inspect(JSON.parse(process.argv[1])).replace(/\n/g, "\n    "))' "$2"
    else
        echo "$1" "$2"
    fi
}

# Run initial cleanup
kill $(lsof -t -i :8000) >/dev/null 2>&1
kill $(lsof -t -i :8001) >/dev/null 2>&1
kill $(lsof -t -i :8002) >/dev/null 2>&1
kill $(lsof -t -i :8003) >/dev/null 2>&1
kill $(lsof -t -i :8004) >/dev/null 2>&1
kill $(lsof -t -i :8005) >/dev/null 2>&1
kill $(lsof -t -i :8006) >/dev/null 2>&1
kill $(lsof -t -i :8007) >/dev/null 2>&1
kill $(lsof -t -i :8008) >/dev/null 2>&1

# Server

cd ./server

echo "Building server..."

cargo build --release >/dev/null 2>&1

echo "Starting server..."

nohup cargo run --release http://localhost:8005,http://localhost:8006,http://localhost:8007,http://localhost:8008 >../logs.txt 2>&1 &

sleep 1

cd ..

# Worker

cd ./worker

echo "Building worker..."

cargo build --release >/dev/null 2>&1

echo "Starting worker..."

nohup cargo run --release 8005 >../logs.txt 2>&1 &
nohup cargo run --release 8006 >../logs.txt 2>&1 &
nohup cargo run --release 8007 >../logs.txt 2>&1 &
nohup cargo run --release 8008 >../logs.txt 2>&1 &

sleep 2

cd ..

# Client

cd ./phantom-client

echo "Building phantom-client..."

cargo build --release >/dev/null 2>&1

echo "Starting phantom-client..."

nohup cargo run --release 8001 0 http://localhost:8000 http://localhost:8002,http://localhost:8003,http://localhost:8004 >../logs.txt 2>&1 &
nohup cargo run --release 8002 1 http://localhost:8000 http://localhost:8001,http://localhost:8003,http://localhost:8004 >../logs.txt 2>&1 &
nohup cargo run --release 8003 2 http://localhost:8000 http://localhost:8001,http://localhost:8002,http://localhost:8004 >../logs.txt 2>&1 &
nohup cargo run --release 8004 3 http://localhost:8000 http://localhost:8001,http://localhost:8002,http://localhost:8003 >../logs.txt 2>&1 &

sleep 2

echo "Setting player ids..."

curl -sS --header "Content-Type: application/json" --request POST --data '{"player_id":0}' -o /dev/null http://localhost:8001/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{"player_id":1}' -o /dev/null http://localhost:8002/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{"player_id":2}' -o /dev/null http://localhost:8003/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{"player_id":3}' -o /dev/null http://localhost:8004/set_id

echo "Submitting round 1 keys..."

curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8001/submit_r1
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8002/submit_r1
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8003/submit_r1
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8004/submit_r1

echo "Get public key..."

curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8001/get_pk
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8002/get_pk
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8003/get_pk
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8004/get_pk

echo "Submitting round 2 keys..."

curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8001/submit_r2
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8002/submit_r2
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8003/submit_r2
curl -sS --header "Content-Type: application/json" --request POST --data '{}' -o /dev/null http://localhost:8004/submit_r2

echo "Getting player data..."

log "  Player 1:" $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8001/get_player)
log "  Player 2:" $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8002/get_player)
log "  Player 3:" $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8003/get_player)
log "  Player 4:" $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8004/get_player)

cd ..

echo "Done!"

tail -f logs.txt
