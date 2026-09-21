use std::io::{Read, stdin};

#[derive(Debug)]
enum ParseErr {
  InvalidOpen(usize),
  InvalidClose(usize),
}

/// builds a bimap for the entire program.
/// better for cache locality than using the bimap itself
fn map(text: &[u8]) -> Result<Vec<usize>, ParseErr> {
    let mut res = vec![0usize; text.len()];
    let mut stack = Vec::new();
    for (i, &val) in text.iter().enumerate() {
        match val {
            b'[' => stack.push(i),
            b']' => {
                if let Some(open) = stack.pop() {
                    res[open] = i;
                    res[i] = open;
                } else {
                    return Err(ParseErr::InvalidClose(i));
                }
            }
            _ => {}
        }
    }
    if let Some(idx) = stack.pop() {
        return Err(ParseErr::InvalidOpen(idx));
    }
    Ok(res)
}

fn main() -> Result<(), ParseErr> {
  print!("Input your brainfuck syntax: ");
  let mut program = String::new();
  _ = stdin().read_line(&mut program);

  let mut tape = [0u8; 50000];
  let mut ptr = 0usize;
  let mut ctr = 0usize;

  let programb = program.as_bytes();
  let bimap = map(programb)?;

  while ctr < programb.len() {
    match programb[ctr] {
      b'>' => ptr += 1,
      b'<' => ptr -= 1,
      b'+' => tape[ptr] = tape[ptr].wrapping_add(1), // to prevent overflow
      b'-' => tape[ptr] = tape[ptr].wrapping_sub(1),
      b'.' => print!("{}", tape[ptr] as char),
      b',' => _ = stdin().read_exact(&mut tape[ptr..=ptr]),
      b'[' if tape[ptr] == 0 => ctr = bimap[ctr],
      b']' if tape[ptr] != 0 => ctr = bimap[ctr] - 1, // right before the loop
      _ => {}
    }
    ctr += 1;
  }

  Ok(())
}
