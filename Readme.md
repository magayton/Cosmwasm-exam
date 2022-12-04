Coswasm exam

Implementation of the bidding contract 

Just to let you know, I had to add this in my Cargo.toml  

getrandom = { version = "0.2", features = ["js"] }  
else my cargo wasm would not compile (I am on a mac)