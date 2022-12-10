use color_eyre::Report;
use itertools::Itertools;
use std::str::FromStr;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day10.txt").await?;

    let mut instructions: Vec<Instruction> = Vec::new();

    while let Some(line) = input.next().await {
        instructions.push(line.parse()?);
    }

    instructions.push(Instruction::Stop);

    let mut cpu_core = CPUCore {
        program: instructions,
        ..Default::default()
    };
    cpu_core.reset();

    let pause_cycles = [20, 60, 100, 140, 180, 220];

    let mut pause_cycle_values = Vec::new();

    while !cpu_core.stopped() {
        cpu_core.cycle();
        if pause_cycles.contains(&cpu_core.cycles) {
            let signal_strength = cpu_core.registers.x * cpu_core.cycles as i128;
            trace!("Signal strength added {signal_strength}");
            pause_cycle_values.push(signal_strength);
        }
    }

    Ok(Reportable {
        year: 2022,
        day: 10,
        part: 1.into(),
        result: TaskResult::I128(pause_cycle_values.iter().sum()),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day10.txt").await?;

    let mut instructions: Vec<Instruction> = Vec::new();

    while let Some(line) = input.next().await {
        instructions.push(line.parse()?);
    }

    instructions.push(Instruction::Stop);

    let mut cpu_core = CPUCore {
        program: instructions,
        ..Default::default()
    };
    cpu_core.reset();

    while !cpu_core.stopped() {
        cpu_core.cycle();
    }

    let output = cpu_core.crt.to_string();

    trace!("Output:\n{output}");

    Ok(Reportable {
        year: 2022,
        day: 10,
        part: 2.into(),
        result: TaskResult::String(output),
    })
}

#[derive(Clone, Debug)]
pub struct CRTScreen {
    display: [[bool; 40]; 6],
    cur_line: u8,
    cur_pixel: u8,
}

impl Default for CRTScreen {
    fn default() -> Self {
        Self {
            display: [[false; 40]; 6],
            cur_line: Default::default(),
            cur_pixel: Default::default(),
        }
    }
}

impl std::fmt::Display for CRTScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .display
            .iter()
            .map(|line| {
                line.iter()
                    .map(|pixel| if *pixel { "#" } else { " " })
                    .join("")
            })
            .join("\n");
        f.write_str(&lines)
    }
}

impl CRTScreen {
    fn draw_step(&mut self, registers: &Registers) {
        let sprite_pos = registers.x;
        // clamped register representation
        let covered_pixels = (sprite_pos - 1, sprite_pos, sprite_pos + 1);
        let cur_pixel = self.cur_pixel as i128;
        let current_pixel_covered = cur_pixel == covered_pixels.0
            || cur_pixel == covered_pixels.1
            || cur_pixel == covered_pixels.2;
        if current_pixel_covered {
            self.display[self.cur_line as usize][self.cur_pixel as usize] = true;
        }
        self.cur_pixel += 1;
        if self.cur_pixel > 39 {
            self.cur_pixel = 0;
            self.cur_line += 1;
        }
        if self.cur_line > 5 {
            self.cur_line = 0;
        }
    }
}

#[derive(Clone, Debug)]
pub struct CPUCore {
    cycles: u128,
    cycles_on_instr_rem: u8,
    curr_instr: Instruction,
    program: Vec<Instruction>,
    registers: Registers,
    crt: CRTScreen,
}

impl Default for CPUCore {
    fn default() -> Self {
        Self {
            cycles: 1,
            cycles_on_instr_rem: Default::default(),
            curr_instr: Default::default(),
            program: Default::default(),
            registers: Default::default(),
            crt: CRTScreen::default(),
        }
    }
}

impl CPUCore {
    pub fn reset(&mut self) {
        assert!(
            !self.program.is_empty(),
            "Attempted reset on CPU without program"
        );
        self.curr_instr = *self.program.first().unwrap();
        self.cycles_on_instr_rem = self.curr_instr.delay();
        self.registers = Registers::default();
    }
    pub fn stopped(&self) -> bool {
        self.curr_instr == Instruction::Stop
    }
    pub fn cycle(&mut self) {
        self.cycles += 1;
        trace!("Advancing CPU to Cycle {}", self.cycles);
        self.cycles_on_instr_rem = self
            .cycles_on_instr_rem
            .checked_sub(1)
            .expect("cycle delay counter underflowed");
        trace!("Stepping CRT");
        self.crt.draw_step(&self.registers);
        trace!("Checking instruction output");
        if self.cycles_on_instr_rem == 0 {
            if self.curr_instr != Instruction::Stop {
                self.curr_instr.apply_to_registers(&mut self.registers);
                self.registers.pc += 1;
                self.curr_instr = *self
                    .program
                    .get(self.registers.pc as usize)
                    .expect("program jumped outside execution limit");
            }
            self.cycles_on_instr_rem = self.curr_instr.delay();
            trace!(
                "Next instruction: {:?}, {} cycles",
                self.curr_instr,
                self.cycles_on_instr_rem
            );
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Registers {
    x: i128,
    pc: usize,
}

impl Default for Registers {
    fn default() -> Self {
        Self { x: 1, pc: 0 }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Instruction {
    #[default]
    Noop,
    AddX(i128),
    Stop,
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((op, args)) = s.split_once(' ') {
            Ok(match op {
                "addx" => {
                    let arg: i128 = args
                        .parse()
                        .context(report!("Invalid ADDX instruction parameter {s:?}"))?;
                    Instruction::AddX(arg)
                }
                v => return Err(report!("Invalid PAR1 instruction {v:?}")),
            })
        } else {
            Ok(match s {
                "noop" => Instruction::Noop,
                "stop" => Instruction::Stop,
                v => return Err(report!("Invalid PAR0 instruction {v:?}")),
            })
        }
    }
}

impl Instruction {
    pub fn delay(self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
            Instruction::Stop => 1,
        }
    }
    pub fn apply_to_registers(self, regs: &mut Registers) {
        match self {
            Instruction::Noop => trace!("No operation"),
            Instruction::AddX(v) => {
                trace!("Adding {v} to register X {}", regs.x);
                regs.x += v;
            }
            Instruction::Stop => trace!("STOP INSTRUCTION"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::aoc2022::day10::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_three_instr_prg() {
        let program = vec![
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
            Instruction::Stop,
        ];

        let mut cpu_core = CPUCore {
            program,
            ..Default::default()
        };
        cpu_core.reset();
        while !cpu_core.stopped() {
            cpu_core.cycle();
        }
        assert!(cpu_core.registers.x == -1, "Invalid register outcome");
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_long_prog() {
        let program = TEST_PROG_1;
        let mut program = program
            .split("\n")
            .map(|x| x.trim().parse().unwrap())
            .collect_vec();
        program.push(Instruction::Stop);

        let mut cpu_core = CPUCore {
            program,
            ..Default::default()
        };
        cpu_core.reset();
        while !cpu_core.stopped() {
            cpu_core.cycle();
            if cpu_core.cycles == 20 {
                assert!(
                    cpu_core.registers.x == 21,
                    "Wanted X to be 21, got {}",
                    cpu_core.registers.x
                );
            }
            if cpu_core.cycles == 60 {
                assert!(
                    cpu_core.registers.x == 19,
                    "Wanted X to be 19, got {}",
                    cpu_core.registers.x
                );
            }
            if cpu_core.cycles == 100 {
                assert!(
                    cpu_core.registers.x == 18,
                    "Wanted X to be 18, got {}",
                    cpu_core.registers.x
                );
            }
            if cpu_core.cycles == 140 {
                assert!(
                    cpu_core.registers.x == 21,
                    "Wanted X to be 21, got {}",
                    cpu_core.registers.x
                );
            }
            if cpu_core.cycles == 180 {
                assert!(
                    cpu_core.registers.x == 16,
                    "Wanted X to be 16, got {}",
                    cpu_core.registers.x
                );
            }
            if cpu_core.cycles == 220 {
                assert!(
                    cpu_core.registers.x == 18,
                    "Wanted X to be 18, got {}",
                    cpu_core.registers.x
                );
            }
        }
    }

    const TEST_PROG_1: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
}
