# rust-guessing-game
This game essentially is as you guessed it (pun intended) a guessing game. A number is picked and you can try to guess it until it is guessed correctly.
Run with:

```bash
# Build with
$ cargo build --release
   Compiling libc v0.2.80
   Compiling getrandom v0.1.15
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling rust-guessing-game v0.1.0 (/Users/larsvandersangen/Documents/Projects/rust-guessing-game)
    Finished release [optimized] target(s) in 4.76s
larsvandersangen@lltms-mbp Projects/rust-guessing-game (main %) » 

# Run with
$ ./target/release/rust-guessing-game 
I've picked a number! You try and guess it 😉
Please provide your guess:
66
You guessed: 66
Wrong! Too big!
Please provide your guess:
55
You guessed: 55
Wrong! Too big!
Please provide your guess:
44
You guessed: 44
Wrong! Too big!
Please provide your guess:
33
You guessed: 33
🎉
You win the interwebz! 🥳
🎉

```


