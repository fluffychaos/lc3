use lc3_system;

fn main() {
    let x = lc3_system::instruction::Instruction::AddRegister {
        dest: 0,
        src1: 0,
        src2: 0
    };
    println!("Hello, world!");
    print!("Instruction: {:?}", x);
}
