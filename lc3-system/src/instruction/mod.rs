#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    AddRegister {
        dest: u8,
        src1: u8,
        src2: u8
    },
    AddImmediate {
        dest: u8,
        src: u8,
        imm: u8
    },
    AndRegsiter {
        dest: u8,
        src1: u8,
        src2: u8
    },
    AndImmediate {
        dest: u8,
        src: u8,
        imm: u8
    },
    Not {
        dest: u8,
        src: u8
    },
}

impl From<[u8;2]> for Instruction {
    fn from(bytes: [u8;2]) -> Self {
        Instruction::from(u16::from_ne_bytes(bytes))
    }
}

impl From<u16> for Instruction {

    fn from(raw: u16) -> Self {
        let op_code = extract_value(raw, 0, 4);
        match op_code {
            0b0110 => Instruction::Not {
                dest: extract_value(raw, 4, 3),
                src: extract_value(raw, 7, 3)
            },
            _ => unimplemented!()
        }
    }
}

fn extract_value(raw: u16, start: u8, length: u8) -> u8 {
    let num_lower_bits = 16 - start - length;
    let mask = ((1 << length) - 1) << num_lower_bits;
    ((raw & mask) >> num_lower_bits).try_into().expect("only bytes will be extracted")
}

pub const fn add(dest: u8, src1: u8, src2: u8) -> Instruction {
    Instruction::AddRegister {
        dest,
        src1,
        src2
    }
}

pub const fn and(dest: u8, src1: u8, src2: u8) -> Instruction {
    Instruction::AndRegsiter {
        dest, 
        src1, 
        src2
    }
}

pub const fn not(dest: u8, src: u8) -> Instruction {
    Instruction::Not {
        dest, 
        src
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, extract_value};


    #[test]
    fn test_extract_value() {
        let raw = 0b1001_011_101_111111;
        let dest = extract_value(raw, 4, 3);
        assert_eq!(dest, 0b011);
        let op_code = extract_value(raw, 0, 4);
        assert_eq!(op_code, 0b1001);
        let src = extract_value(raw, 7, 3);
        assert_eq!(src, 0b101);
        let tail = extract_value(raw, 10, 6);
        assert_eq!(tail, 0b111111);
    }

    #[test]
    fn test_not_from_u16() {
        let raw = 0b0110_011_101_111111;
        let instruction = From::from(raw);
        let expected = Instruction::Not {
            src: 0b101,
            dest: 0b011
        };
        assert_eq!(expected, instruction);
    }
}