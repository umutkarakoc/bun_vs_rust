# bun_vs_rust

# Install bun, rust and wrk

https://bun.sh/
http://rustup.com/
https://github.com/wg/wrk

# Start bun server
`bun index.js`

# start rust server
`cargo build --release`
`./target/release/bun_vs_rust`

# How to benchmark bun
`wrk -t2 -c100 -d10s 'http://localhost:3000/hello?name=john&num1=5&num2=0'`
`wrk -t2 -c100 -d10s 'http://localhost:3000/hello_html?name=john&num1=5&num2=0'`

# How to benchmark rust
`wrk -t2 -c100 -d10s 'http://localhost:3001/hello?name=john&num1=5&num2=0'`
`wrk -t2 -c100 -d10s 'http://localhost:3001/hello_html?name=john&num1=5&num2=0'`
