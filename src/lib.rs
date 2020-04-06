use std::convert::TryFrom;
use text_io::read;
use std::io;
use std::io::Write;

pub fn compute(mut mailbox: [i32; 100]) -> u32 {
    let mut execute = true;
    let mut program_cnt: usize = 0;
    let mut accumulator: i32 = 0;

    while execute {
        let instruction = mailbox[program_cnt];
        let index : usize = usize::try_from(instruction % 100).unwrap();
        let code : i32 = instruction / 100;

        match code {
            1 => accumulator += mailbox[index],    // ADD
            2 => accumulator -= mailbox[index],    // SUBSTRACT
            3 => mailbox[index] = accumulator,     // STORE
            5 => accumulator = mailbox[index],     // LOAD
            6 => {                                 // BRANCH
                program_cnt = index;
                continue;
            },
            7 => {                                 // BRANCH IF ZERO
                if accumulator == 0 {
                    program_cnt = index;
                    continue;
                }
            },
            8 => {                                 // BRANCH IF POSITIVE
                if accumulator >= 0 {
                    program_cnt = index;
                    continue;
                }
            },
            9 => {
                if index == 1 {                    // INPUT
                    print!("Enter a number: ");
                    io::stdout().flush().unwrap();
                    let value: String = read!("{}\n");
                    accumulator = value.parse().expect("Invalid input value"); // TODO: retry
                } else if index == 2 {             // OUTPUT
                    println!("Output: {}", accumulator);
                } else {
                    panic!(format!("Unknown instruction: {}", mailbox[program_cnt]));
                }
            },
            0 => {                                 // HALT
                    execute = false;
                    continue;

            }
            _ => {
                panic!(format!("Unknown instruction: {}", mailbox[program_cnt]));
                // TODO sigint
            }
        }
        program_cnt += 1;
    }

    return 0;
}
