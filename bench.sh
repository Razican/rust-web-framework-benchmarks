#!/bin/sh

./wrk/wrk --latency -t12 -c1000 -d1m $1 > results/$2.txt