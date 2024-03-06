use revm_interpreter::{
    analysis::to_analysed,
    instructions::{memory, stack},
    BytecodeLocked, Contract, DummyHost, Interpreter,
};
use revm_primitives::{Bytecode, Bytes, Env};

fn main() {
    let mut contract = Contract::default();
    // PUSH1
    // PUSH1
    // MSTORE
    let bc = Bytecode::new_raw(Bytes::from_iter([0x60, 0x80, 0x60, 0x40, 0x52].into_iter()))
        .to_checked();
    let bc = to_analysed(bc);
    contract.bytecode = BytecodeLocked::try_from(bc).unwrap();
    let mut host = DummyHost::new(Env::default());
    let mut intrp = Interpreter::new(contract, u64::MAX, true);

    println!("{:?}", intrp.instruction_pointer);
    stack::push::<1>(&mut intrp, &mut host);
    println!("{:?}", intrp.instruction_pointer);
    stack::push::<1>(&mut intrp, &mut host);
    println!("{:?}", intrp.instruction_pointer);
    memory::mstore(&mut intrp, &mut host);

    println!("{:?}", intrp.instruction_result);
}
