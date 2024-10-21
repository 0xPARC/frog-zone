#!/bin/sh

kill $(lsof -t -i :8000) >/dev/null 2>&1
kill $(lsof -t -i :8001) >/dev/null 2>&1
kill $(lsof -t -i :8002) >/dev/null 2>&1
kill $(lsof -t -i :8003) >/dev/null 2>&1
kill $(lsof -t -i :8004) >/dev/null 2>&1

echo "Starting server..."

cd ../server
cargo build --release >/dev/null 2>&1
nohup cargo run --release >/dev/null 2>&1 &
cd ../phantom-client

echo "Starting phantom-clients..."

cargo build --release >/dev/null 2>&1
nohup cargo run --release 8001 0 >/dev/null 2>&1 &
nohup cargo run --release 8002 1 >/dev/null 2>&1 &
nohup cargo run --release 8003 2 >/dev/null 2>&1 &
nohup cargo run --release 8004 3 >/dev/null 2>&1 &

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

echo "All work!"

kill $(lsof -t -i :8000)
kill $(lsof -t -i :8001)
kill $(lsof -t -i :8002)
kill $(lsof -t -i :8003)
kill $(lsof -t -i :8004)
