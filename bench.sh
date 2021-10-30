#!/bin/sh

./wrk/wrk --latency -t16 -c1500 -d2m $1 > results/$2.txt