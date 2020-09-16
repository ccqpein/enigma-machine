use rand::seq::SliceRandom;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug)]
pub struct Rotor {
    /// How many "contacter"
    len: usize,

    /// Index of this vec is input, like output_vec[0] = 3
    /// means first contactor connected to fourth contactor
    output_vec: Vec<usize>,
}

impl Rotor {
    pub fn init(contactor_len: usize) -> Result<Self> {
        if contactor_len % 2 != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "input numbers has to be even",
            ));
        }

        let mut rng = rand::thread_rng();
        let mut contactors: Vec<usize> =
            (0..contactor_len).into_iter().map(|x| x as usize).collect();

        Ok(Self {
            len: contactor_len as usize,
            output_vec: {
                contactors.shuffle(&mut rng); // randomize contactors
                contactors
            },
        })
    }

    fn len(&self) -> usize {
        self.len
    }

    /// for manually set rotor without randomize
    fn set(&mut self, output: Vec<usize>) -> Result<()> {
        if output.len() != output.len() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "input length does not match output length",
            ));
        }

        self.len = output.len();
        self.output_vec = output;

        Ok(())
    }

    /// input the index of contacter of one side, return the index of other side.
    /// both sides are mirror to each other
    fn input(&self, i: usize) -> Result<usize> {
        if i >= self.len() {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("input {} cannot found", i.to_string()),
            ))
        } else {
            Ok(self.output_vec[i])
        }
    }

    /// input the index of contactor of another side, compare with input(),
    /// return the index of other side
    fn rev_input(&self, i: usize) -> Result<usize> {
        if let Some(p) = self.output_vec.iter().position(|x| *x == i) {
            Ok(p)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("input {} cannot found", i.to_string()),
            ))
        }
    }

    /// input with offset of rotor rotate
    fn input_with_offset(&self, i: usize, offset: usize) -> Result<usize> {
        let a = (i + offset) % self.len();
        let mut re = self.input(a).unwrap() as i32 - offset as i32;

        if re < 0 {
            re += self.len() as i32;
        }
        Ok(re as usize)
    }

    /// input with offset of rotor rotate from other side
    fn rev_input_with_offset(&self, i: usize, offset: usize) -> Result<usize> {
        let a = (i + offset) % self.len();
        let mut re = self.rev_input(a).unwrap() as i32 - offset as i32;
        if re < 0 {
            re += self.len() as i32;
        }
        Ok(re as usize)
    }
}

impl Default for Rotor {
    fn default() -> Self {
        Self::init(26).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Reflector {
    input: Vec<usize>,
    output: Vec<usize>,
}

impl Reflector {
    /// reflector input and output should be if A->B, then B->A.
    /// diff than Rotor. Rotor can be A->B, B->C, C->A, and follow A<-B
    /// that's why the output is input's reverse
    pub fn init(length: usize) -> Result<Self> {
        if length % 2 != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "input numbers has to be even",
            ));
        }

        let mut rng = rand::thread_rng();
        let mut ll = (0..length).into_iter().collect::<Vec<usize>>();
        ll.shuffle(&mut rng);

        Ok(Self {
            input: ll.clone(),
            output: {
                ll.reverse();
                ll
            },
        })
    }

    /// reflector input
    fn input(&self, i: usize) -> Result<usize> {
        if let Some(p) = self.input.iter().position(|x| *x == i) {
            Ok(self.output[p])
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("input {} cannot found", i.to_string()),
            ))
        }
    }

    fn len(&self) -> usize {
        self.input.len()
    }
}

#[derive(Debug)]
pub struct RotorChain {
    pub spin_status: Vec<usize>,
    chain: Vec<Rotor>,
    reflect: Reflector,
}

impl RotorChain {
    pub fn new(rotors: &Vec<Rotor>, reflect: &Reflector) -> Result<Self> {
        if !rotors.iter().all(|x| x.len() == reflect.len()) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "rotors length does not match reflect length",
            ));
        }

        Ok(Self {
            spin_status: rotors.iter().map(|_| 0).collect(),
            chain: rotors.clone(),
            reflect: reflect.clone(),
        })
    }

    /// Side effect: will change self.spin_status
    /// major part of whole enigma machine
    pub fn input(&mut self, i: usize) -> Result<usize> {
        let mut temp = i;

        // handle input in order
        for ind in 0..self.chain.len() {
            temp = self.chain[ind].input_with_offset(temp, self.spin_status[ind])?;
        }

        temp = self.reflect.input(temp)?;

        // in rev order
        for ind in (0..self.chain.len()).rev() {
            temp = self.chain[ind].rev_input_with_offset(temp, self.spin_status[ind])?
        }

        // spin all rotors
        for ind in 0..self.chain.len() {
            self.spin_status[ind] += 1;
            if self.spin_status[ind] == self.chain[ind].len() {
                self.spin_status[ind] = 0;
            } else {
                break;
            }
        }

        Ok(temp)
    }

    fn reset_spin_status(&mut self) {
        self.spin_status = self.chain.iter().map(|_| 0).collect();
    }

    /// set status
    pub fn set_spin_status(&mut self, a: Vec<usize>) {
        self.spin_status = a
    }

    pub fn len(&self) -> usize {
        self.reflect.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotor_input() {
        let r = Rotor::default();
        let o = r.input(2).unwrap();
        let o1 = r.rev_input(o).unwrap();
        assert_eq!(2, o1);

        let o = r.input_with_offset(2, 1).unwrap();
        let mut o2 = r.input(3).unwrap();
        if o2 == 0 {
            o2 = 26
        }
        assert_eq!(o2 - 1, o);
    }

    #[test]
    fn input_vs_output() {
        let r1: Rotor = Default::default();
        let r2: Rotor = Default::default();
        let re: Reflector = Reflector::init(26).unwrap(); // reflect

        let mut c1 = RotorChain::new(&vec![r1.clone(), r2.clone()], &re).unwrap();
        // need the new rotorchain for test
        let mut c2 = RotorChain::new(&vec![r1.clone(), r2.clone()], &re).unwrap();

        let output = c1.input(3).unwrap();
        assert_eq!(3, c2.input(output).unwrap());

        // output shoud different
        let output_1 = c1.input(3).unwrap();
        assert!(output_1 != output); // two times answers should not same as each other
        assert_eq!(3, c2.input(output_1).unwrap());

        // reset r1, because everytime, rotor spin one after input
        assert!(c2.spin_status[0] == c1.spin_status[0]);
        assert!(c2.spin_status[0] == 2);
        c2.reset_spin_status();
        assert!(output == c2.input(3).unwrap());

        // check spin status overflow
        c1.reset_spin_status();
        c1.spin_status = vec![25, 25];
        c1.input(3);
        assert_eq!(c1.spin_status, vec![0, 0]);
        assert_eq!(output, c1.input(3).unwrap());
    }
}
