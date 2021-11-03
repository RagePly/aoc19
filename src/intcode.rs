pub type Int = i32;
type ProgramMemory = Vec<Int>;

const OPCODE_ADD:   Int = 1;
const OPCODE_MUL:   Int = 2;
const OPCODE_HALT:  Int = 99;

pub enum IntcodeStatus {
    Running,
    Halted
}

pub struct IntcodeState {
    backup_memory: ProgramMemory,
    pub memory: ProgramMemory,
    ip: usize,
    pub status: IntcodeStatus
}

impl IntcodeState {
    pub fn new(source: &str) -> Option<IntcodeState> {
        match parse_intcode_from_source(source) {
            Some(m) => Some(
                IntcodeState {
                    backup_memory: m.clone(),
                    memory: m,
                    ip: 0,
                    status: IntcodeStatus::Running
                }
            ),
            None => None
        }
    }

    pub fn reset(&mut self) {
        self.memory = self.backup_memory.clone();
        self.ip = 0;
        self.status = IntcodeStatus::Running;
    }

    pub fn step(&mut self) -> bool {
        match self.memory[self.ip] {
            OPCODE_ADD => {
                let a = self.memory[self.ip + 1] as usize;
                let b = self.memory[self.ip + 2] as usize;
                let c = self.memory[self.ip + 3] as usize;
                self.memory[c] = self.memory[a] + self.memory[b];
                self.ip += 4;
                true
            }
            OPCODE_MUL => {
                let a = self.memory[self.ip + 1] as usize;
                let b = self.memory[self.ip + 2] as usize;
                let c = self.memory[self.ip + 3] as usize;
                self.memory[c] = self.memory[a] * self.memory[b];
                self.ip += 4;
                true
            }
            OPCODE_HALT => {
                self.status = IntcodeStatus::Halted;
                false
            }
            p => panic!("invalid opcode {}", p)
        }
    }

    pub fn run(&mut self) {
        while self.step() {};
    }
}

fn parse_intcode_from_source(source: &str) -> Option<ProgramMemory> {
    let mut v: ProgramMemory = ProgramMemory::new();
    for number in source.split(",").into_iter() {
        match number.parse() {
            Ok(i) => v.push(i),
            Err(_) => return None
        }
    }
    Some(v)
}