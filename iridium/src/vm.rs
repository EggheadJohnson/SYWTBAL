use instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,

}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    fn decode_opcode(self: &mut Self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool{
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered!");
                return true;

            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPF => {
                let delta = self.registers[self.next_8_bits() as usize];
                self.pc += delta as usize;
            },
            Opcode::JMPB => {
                let delta = self.registers[self.next_8_bits() as usize];
                self.pc -= delta as usize;
            },
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            },
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = false;
                } else {
                    self.equal_flag = true;
                }
                self.next_8_bits();
            }
            _ => {
                println!("Unrecognized command! Terminating!");
                return true;
            }
        }

        false
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc+1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        return VM::new();
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 20, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 20);
        assert_eq!(test_vm.registers[2], 30);
    }

    #[test]
    fn test_opcode_sub_pos_result() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 20, 1, 1, 0, 10, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 20);
        assert_eq!(test_vm.registers[1], 10);
        assert_eq!(test_vm.registers[2], 10);
    }

    #[test]
    fn test_opcode_sub_neg_result() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 20, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 20);
        assert_eq!(test_vm.registers[2], -10);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 20, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 20);
        assert_eq!(test_vm.registers[2], 200);
    }

    #[test]
    fn test_opcode_div_no_remainder() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 20, 1, 1, 0, 10, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 20);
        assert_eq!(test_vm.registers[1], 10);
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_opcode_div_with_remainder() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 3, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 3);
        assert_eq!(test_vm.registers[2], 3);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jmp() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);

    }
    #[test]
    fn test_jmpf() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);

    }
    #[test]
    fn test_jmpb() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_eq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
    #[test]
    fn test_neq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }
}
