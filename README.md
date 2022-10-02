# Overview
This Project was a HUGE learning curve for me. I have never touched rust before and have never done any networking or sockets or anything like that so I went into this with a huge learning curve but am happ ywith the end Project!
I did  include old code that with extra time would have implemented a proper client for receiving the seconf tcp stream but I ran into too many errors due to me being new to this.

This Repo includes my tests in making a functioning Rust Network that can take in a Request from a Curl HTTP and add the data provided to a list and send that list to a different request, originally this was to be done on a Rasberry pi pico to be able to send sensor data to a server to be processed, but I ran into a ton of issues with the Pico so I had so simplify it.


Run "curl --request POST 'http://127.0.0.1:8181/<number-to-add>'" 
     or in browser "http://127.0.0.1:8181/<number-to-add>"
     
Run "ncat -l 8182 -k" to see the output of second socket

[Software Demo Video](https://youtu.be/mNyTF_aaVwU)

# Development Environment

Rust:
	tokio
	mini-redis
Cargo
Vim
Ubuntu
Netcat
curl


# Useful Websites

* [rust-lang-nursery](https://rust-lang-nursery.github.io/rust-cookbook/)
* [Rust Lang](https://www.rust-lang.org/)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}
* I want it to also delete the server-side list when the secondary device asks for it do it doesnt infinitley add to it.
* Further process the Http request into something that is more easily implementable
* Figure out why my second TCP streaming socket wont work and how to get that working as TCP or move to UDP

# Rust-Network
 Rust project

### Commands

#Makes neww application
cargo new my_rust_app 

# Compiles the rust code into executable
rustc rustnetwork.rs

# run the code from application with compiling
cargo run

# install library
cargo install <package>
