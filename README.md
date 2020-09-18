# README #

Enigma machine [Wiki](https://en.wikipedia.org/wiki/Enigma_machine)

> The Enigma machine is an encryption device developed and used in the early- to mid-20th century to protect commercial, diplomatic and military communication. It was employed extensively by Nazi Germany during World War II, in all branches of the German military.

Wiki has given the explanation of design of enigma machine. This repo is trying to use code to simulate enigma machine.

## Rotors ##

Rotor implement in `src/rotor.rs`. Rotor is designed like, input n-th electrical contact, then return the index of result electrical contact.

For example, if the `output_vec` is `[2,3,5,1,4,0]`, it means index 0 electrical contact will return `2`, the index of result electrical contact.

Same rotor as above, `rev_input` is simulating from result electrical contact to input electrical contact, reverse data flow. For example, if I `rev_input` `2`, it will return me `0`.

## Reflector ##

Reflector is self-reflected, like input 2, reflector give me 5. Then I input 5, reflector give me 2. 

## RotorChain ##

The chain of rotors. In real-world enigma machine, it has three rotors and a reflector become one rotor chain. In code world, I can give as many as rotor I want in `Vec<Rotor>`.

However, it works like real enigma machine that 

> input -> rotors -> reflector -> rotors reverse -> output

and rotor chain will store each of rotors offset, like enigma machine does.

## Plugboard ##

Plugboard is pretty simply, just replace one index to another before input and after output. 

> input -> plugboard -> rotors -> reflector -> rotors reverse -> plugboard -> output

## Enigma machine ##

In `src/enigma_machine.rs`, one `RotorChain` and one `Plugboard` can assemble an enigma machine. 

And there is a running simple in `src/main.rs` 
