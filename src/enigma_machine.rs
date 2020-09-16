use crate::plugboard::*;
use crate::rotor::*;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct EnigmaM {
    /// How many different input can accept
    /// as same as Rotor length and reflect length in RotorChain
    cap: usize,

    /// RotorChain, including rotors and a reflector
    rc: RotorChain,

    /// elements in plugboard paris cannot beyond the cap, and it shouldn't have
    /// duplication
    plugboard: Plugboard,
}

impl EnigmaM {
    pub fn new(cap: usize, rotor_num: usize, pb: &Plugboard) -> Result<Self> {
        let rotors = {
            let temp = (0..rotor_num).into_iter().map(|_| Rotor::init(cap));

            if !temp.clone().all(|x| x.is_ok()) {
                return Err(Error::new(ErrorKind::InvalidData, "Cannot make rotors"));
            }

            temp.map(|x| x.unwrap()).collect()
        };

        let reflector = Reflector::init(cap)?;

        Ok(Self {
            cap,
            rc: RotorChain::new(&rotors, &reflector)?,
            plugboard: pb.clone(),
        })
    }

    pub fn input(&mut self, i: usize) -> Result<usize> {
        if i >= self.cap {
            return Err(Error::new(ErrorKind::InvalidInput, "Beyond the range"));
        }
        let a = self.plugboard.input(i);
        let b = self.rc.input(a)?;

        Ok(self.plugboard.input(b))
    }

    pub fn set_spin_status(&mut self, a: Vec<usize>) {
        self.rc.set_spin_status(a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let pb = Plugboard::default(); // empty
        let mut enigma_machine = EnigmaM::new(26, 3, &pb).unwrap();

        let output = enigma_machine.input(3).unwrap();
        let output1 = enigma_machine.input(3).unwrap();
        assert_ne!(output, output1); // has a tiny ability that output equal output1

        // manually set spin status
        enigma_machine.set_spin_status(vec![0, 0, 0]);
        assert_eq!(output, enigma_machine.input(3).unwrap());
        assert_eq!(output1, enigma_machine.input(3).unwrap());

        // test decrypt
        enigma_machine.set_spin_status(vec![0, 0, 0]);
        assert_eq!(3, enigma_machine.input(output).unwrap());
        assert_eq!(3, enigma_machine.input(output1).unwrap());

        ///////////////////////////////////
        ///////////////////////////////////
        // test plugboard
        let pb = Plugboard::new(&vec![(1, 23), (4, 9)]);
        enigma_machine = EnigmaM::new(26, 3, &pb).unwrap();

        let output = enigma_machine.input(3).unwrap();
        let output1 = enigma_machine.input(3).unwrap();
        assert_ne!(output, output1); // has a tiny ability that output equal output1

        // manually set spin status
        enigma_machine.set_spin_status(vec![0, 0, 0]);
        assert_eq!(output, enigma_machine.input(3).unwrap());
        assert_eq!(output1, enigma_machine.input(3).unwrap());

        // test decrypt
        enigma_machine.set_spin_status(vec![0, 0, 0]);
        assert_eq!(3, enigma_machine.input(output).unwrap());
        assert_eq!(3, enigma_machine.input(output1).unwrap());
    }
}
