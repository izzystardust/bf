use std::io;
use std::io::Read;

fn brainfuck(prg: &str, input: &mut io::Read, output: &mut io::Write) {
    let mut buffer = vec![0 as u8; 1024];
    let mut iptr = 0;
    let mut mptr = 0;
    loop {
        match prg.as_bytes()[iptr] {
            b'>' => mptr += 1,
            b'<' => mptr -= 1,
            b'+' => buffer[mptr] += 1,
            b'-' => buffer[mptr] -= 1,
            b'.' => {output.write(&mut buffer[mptr .. mptr+1]);},
            b',' => {
                match input.read(&mut buffer[mptr .. mptr+1]) {
                    Ok(_) => { },
                    Err(e) => { panic!(e) },
                }
            }
            b'[' => {
                if buffer[mptr] == 0 {
                    let mut ctr = 1;
                    iptr += 1;
                    while ctr != 0 {
                        match prg.as_bytes()[iptr] {
                            b'[' => { ctr += 1; iptr += 1; },
                            b']' => { ctr -= 1; iptr += 1; },
                            _    => { iptr += 1; },
                        }
                    }
                    iptr += 1;
                }
            },
            b']' => {
                if buffer[mptr] != 0 {
                    let mut ctr = 1;
                    iptr -= 1;
                    while ctr != 0 {
                        match prg.as_bytes()[iptr] {
                            b'[' => { ctr -= 1; iptr -= 1; },
                            b']' => { ctr += 1; iptr -= 1; },
                            _    => iptr -= 1,
                        }
                    }
                }
            },
            _    => {},
        }
        iptr += 1;
        if iptr >= prg.len() {
            break;
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    //let mut program = String::new();
    //stdin.read_to_string(&mut program);
    //println!("{}", program);
    //brainfuck(program.as_ref(), &mut stdin)
    brainfuck("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
              &mut stdin,
              &mut io::stdout(),
              );
    //brainfuck("++[-]", &mut stdin);
}

#[test]
fn test_brainfuck() {
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut output = Vec::new();
    brainfuck(hello_world, &mut io::stdin(), &mut output);
    assert!(String::from_utf8(output).unwrap() == "Hello, world!");
}
