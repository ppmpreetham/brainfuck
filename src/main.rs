#[derive(Debug)]
enum ParseErr {
  InvalidOpen(usize),
  InvalidClose(usize),
}

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
  let program = "--[----->+<]>----.[--->+<]>----.+++[->+++<]>++.++++++++.+++++.--------.-[--->+<]>--.+[->+++<]>+.++++++++.";
  let mut tape = [0u8; 50000];
  let mut ptr = 0usize;
  let mut ctr = 0usize;

  let programb = program.as_bytes();
  let bimap = map(programb)?;

  while ctr < programb.len() {
    match programb[ctr] {
      b'>' => ptr += 1,
      b'<' => ptr -= 1,
      b'+' => tape[ptr] += 1,
      b'-' => tape[ptr] -= 1,
      b'.' => print!("{}", tape[ptr] as char),
      b',' => {},
      b'[' if tape[ptr] == 0 => ctr = bimap[ctr],
      b']' if tape[ptr] != 0 => ctr = bimap[ctr] - 1, // right before the loop
      _ => {}
    }
    ctr += 1;
  }

  Ok(())
}
