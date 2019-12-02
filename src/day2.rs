fn parse_input(input: String) -> Vec<i64> {
    input
        .trim_end()
        .split(',')
        .map(|s| s.parse::<i64>().expect("failed to parse input"))
        .collect::<Vec<i64>>()
}

fn run_program(opcodes: &mut [i64], ip_addr: &mut usize) {
    loop {
        match opcodes[*ip_addr] {
            99 => {
                *ip_addr += 1;
                break;
            }
            1 => {
                opcodes[opcodes[*ip_addr + 3] as usize] = opcodes[opcodes[*ip_addr + 1] as usize]
                    + opcodes[opcodes[*ip_addr + 2] as usize]
            }
            2 => {
                opcodes[opcodes[*ip_addr + 3] as usize] = opcodes[opcodes[*ip_addr + 1] as usize]
                    * opcodes[opcodes[*ip_addr + 2] as usize]
            }
            _ => unreachable!("undefined opcode reached"),
        };
        *ip_addr += 4;
    }
}

pub fn one(input: String) -> i64 {
    let mut op_vec = parse_input(input);
    let opcodes = op_vec.as_mut_slice();
    opcodes[1] = 12;
    opcodes[2] = 2;
    run_program(opcodes, &mut 0usize);
    opcodes[0]
}

pub fn two(input: String) -> i64 {
    for noun in 0i64..=99 {
        for verb in 0i64..=99 {
            let mut op_vec = parse_input(input.clone());
            let opcodes = op_vec.as_mut_slice();
            opcodes[1] = noun;
            opcodes[2] = verb;
            run_program(opcodes, &mut 0usize);
            if opcodes[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    -1i64 // error
}
