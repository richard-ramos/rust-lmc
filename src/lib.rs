use std::convert::TryFrom;
use text_io::read;
use std::io;
use std::io::Write;

pub fn compute(mut mailbox: [i32; 100]) {
    let mut program_cnt: usize = 0;
    let mut accumulator: i32 = 0;

    loop {
        let instruction = mailbox[program_cnt];
        let index : usize = usize::try_from(instruction % 100).unwrap();
        let code : i32 = instruction / 100;
        match code {
            1 => accumulator += mailbox[index], // ADD
            2 => accumulator -= mailbox[index], // SUBSTRACT
            3 => mailbox[index] = accumulator, // STORE
            5 => accumulator = mailbox[index], // LOAD
            6 => { // BRANCH
                program_cnt = index;
                continue;
            },
            7 => { // BRANCH IF ZERO
                if accumulator == 0 {
                    program_cnt = index;
                    continue;
                }
            },
            8 => { // BRANCH IF POSITIVE
                if accumulator >= 0 {
                    program_cnt = index;
                    continue;
                }
            },
            9 => match index {
                    1 => { // INPUT
                        print!("Input: ");
                        io::stdout().flush().unwrap();
                        let value: String = read!("{}\n");
                        accumulator = value.parse().expect("Invalid input value");
                    },
                    2 => println!("Output: {}", accumulator), // OUTPUT
                    _ => panic!(format!("Unknown instruction: {}", mailbox[program_cnt]))
                },
            0 => return, // HALT
            _ => panic!(format!("Unknown instruction: {}", mailbox[program_cnt]))
        }
        program_cnt += 1;
    }
}
