#!/bin/sh

kill $(lsof -t -i :8000) >/dev/null 2>&1
kill $(lsof -t -i :8001) >/dev/null 2>&1
kill $(lsof -t -i :8002) >/dev/null 2>&1
kill $(lsof -t -i :8003) >/dev/null 2>&1
kill $(lsof -t -i :8004) >/dev/null 2>&1

cd ../server

echo "Building server..."

cargo build --release >/dev/null 2>&1

echo "Starting server..."

nohup cargo run --release >/dev/null 2>&1 &

cd ../phantom-client

echo "Building phantom-client..."

cargo build --release >/dev/null 2>&1

echo "Starting phantom-client..."

nohup cargo run --release 8001 0 http://localhost:8002,http://localhost:8003,http://localhost:8004 >/dev/null 2>&1 &
nohup cargo run --release 8002 1 http://localhost:8001,http://localhost:8003,http://localhost:8004 >/dev/null 2>&1 &
nohup cargo run --release 8003 2 http://localhost:8001,http://localhost:8002,http://localhost:8004 >/dev/null 2>&1 &
nohup cargo run --release 8004 3 http://localhost:8001,http://localhost:8002,http://localhost:8003 >/dev/null 2>&1 &

sleep 1s

echo "Setting player ids..."

curl -sS --header "Content-Type: application/json" --request POST --data '{player_id:0}' -o /dev/null http://localhost:8001/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{player_id:1}' -o /dev/null http://localhost:8002/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{player_id:2}' -o /dev/null http://localhost:8003/set_id
curl -sS --header "Content-Type: application/json" --request POST --data '{player_id:3}' -o /dev/null http://localhost:8004/set_id

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

echo "  Player 1: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8001/get_player)
echo "  Player 2: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8002/get_player)
echo "  Player 3: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8003/get_player)
echo "  Player 4: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8004/get_player)

echo "Moving around..."

echo "  Player 1 (Up): " $(curl -sS --header "Content-Type: application/json" --request POST --data '{"direction":"Up"}'    http://localhost:8001/move)
echo "  Player 2 (Down): " $(curl -sS --header "Content-Type: application/json" --request POST --data '{"direction":"Down"}'  http://localhost:8002/move)
echo "  Player 3 (Left): " $(curl -sS --header "Content-Type: application/json" --request POST --data '{"direction":"Left"}'  http://localhost:8003/move)
echo "  Player 4 (Right): " $(curl -sS --header "Content-Type: application/json" --request POST --data '{"direction":"Right"}' http://localhost:8004/move)

echo "Getting updated player data..."

echo "  Player 1: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8001/get_player)
echo "  Player 2: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8002/get_player)
echo "  Player 3: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8003/get_player)
echo "  Player 4: " $(curl -sS --header "Content-Type: application/json" --request POST --data '{}' http://localhost:8004/get_player)

echo "All work!"

kill $(lsof -t -i :8000)
kill $(lsof -t -i :8001)
kill $(lsof -t -i :8002)
kill $(lsof -t -i :8003)
kill $(lsof -t -i :8004)
