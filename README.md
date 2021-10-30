# Benchmarks for Rust web frameworks

This repository contains easy programs that can be benchmarked using wrk. This gives you an
approximate idea on how they perform. It's still WIP.

Features:
 - All frameworks share common dependencies (when possible), by using a Rust workspace
 - All frameworks produce an equivalent response
 - Static response (ping-pong) benchmarks
 - Benchmarks for Actix-web, Rocket 0.4, Rocket 0.5, Tide and Warp

Things that would be nice to have in the future:
 - Benchmarking even more frameworks and versions
 - Benchmarking more complex flows than a ping-pong
 - Benchmarking with templating engines
 - Benchmarking with database access
 