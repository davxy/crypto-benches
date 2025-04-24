#!/bin/bash
 
set -e

sudo -v

echo "Starting benchmark with hich CPU and IO priority..."

cargo bench $1 &
bench_pid=$!
echo "WAIT FOR $bench_pid"
sleep 1
sudo ionice -c 1 -n 0 -p $bench_pid 
sudo renice -n -20 -p $bench_pid
sudo taskset -cp 2 $bench_pid
wait "$bench_pid"

echo "Benchmarking done"
