pub mod instruction;

pub type Address = u16;
pub type DataUnit = u16;

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        let add = Instruction::AddRegister {
            dest: 2,
            src1: 16,
            src2: 23,
        };
        let add2 = add;
        assert_eq!(add, add2);
    }
}
