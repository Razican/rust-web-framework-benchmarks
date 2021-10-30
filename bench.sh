#!/bin/sh

./wrk/wrk --latency -t4 -c2000 -d2m $1 > results/$2.txt