pub type Address = u16;
pub type DataUnit = u16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Add {
        dest: u16,
        src1: u16,
        src2: u16
    },
    And {
        dest: u16,
        src1: u16,
        src2: u16
    },
    Not {
        dest: u16,
        src: u16
    }
}

#[cfg(test)]
mod tests {
    use crate::Instruction;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        let add = Instruction::Add {
            dest: 2,
            src1: 16,
            src2: 23,
        };
        let add2 = add;
        assert_eq!(add, add2);
    }
}
