#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Add,
    Calldataload,
    MStore,
    Push(usize),
    Return,
    Dup(usize),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add => write!(f, "add"),
            Instruction::Calldataload => write!(f, "calldataload"),
            Instruction::MStore => write!(f, "mstore"),
            Instruction::Push(n) => write!(f, "push {}", n),
            Instruction::Return => write!(f, "return"),
            Instruction::Dup(n) => write!(f, "dup {}", n),
        }
    }
}

impl Instruction {
    pub fn argument_count(&self) -> usize {
        match self {
            Instruction::Push(_) => 0,
            Instruction::Calldataload => 1,
            _ => 2,
        }
    }
}

#[derive(Debug)]
pub struct Node<'a> {
    instruction: Instruction,
    dependencies: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(instruction: Instruction, dependencies: Vec<&'a Node>) -> Self {
        if instruction.argument_count() != dependencies.len() {
            panic!("invalid argument count")
        }

        Self { instruction, dependencies }
    }

    pub fn traverse(&self) -> Vec<Instruction> {
        let mut instructions = vec![];
        let mut iter = self.dependencies.iter();

        while let Some(dep) = iter.next() {
            instructions.append(&mut dep.traverse());
        }

        instructions.push(self.instruction.clone());

        instructions
    }
}

pub fn main() {
    let i_push0 = Node::new(Instruction::Push(0), vec![]);
    let i_push1 = Node::new(Instruction::Push(1), vec![]);
    let i_push32 = Node::new(Instruction::Push(32), vec![]);
    let i_calldataload = Node::new(Instruction::Calldataload, vec![&i_push0]);
    let i_add = Node::new(Instruction::Add, vec![&i_push1, &i_calldataload]);
    let i_mstore = Node::new(Instruction::MStore, vec![&i_push0, &i_add]);
    let i_return = Node::new(Instruction::Return, vec![&i_push0, &i_push32]);

    let program = vec![i_mstore, i_return];

    println!("graph: {:#?}\n", program);

    let instructions: Vec<Instruction> = program.iter().map(|node| node.traverse()).flatten().collect();

    println!("instructions:");
    instructions.iter().for_each(|i| println!("{}", i));
}

