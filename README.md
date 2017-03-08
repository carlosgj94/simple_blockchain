# simple_blockchain ![Blockchain](http://pix.iemoji.com/images/emoji/apple/ios-9/256/chains.png)
Simple blockchain written in Rust from 0
It has a simple mining proof of work. It consist simply in changing the nonce number in order to get "00000" at the beginning of the hash512.

## Instalation:
Just install Rust language and:
$ git clone https://github.com/carlosgj94/simple_blockchain.git
$ cd simple_blockchain
$ cargo run

## Next steps: 
- Add peers in order to being able to check fake chains and delete them
- Add a database
- Add a simple server/app to create the block outside the terminal
- Make a harder proof of work to add more security
