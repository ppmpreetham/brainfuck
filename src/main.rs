enum ParseErr {
  InvalidOpen(char),
  InvalidClose(char),
}

fn map(text: &str) -> Result<Vec<usize>, ParseErr>{
  let mut res = String::new();
  todo!()
}

fn main() {
  let program = "--[----->+<]>----.[--->+<]>----.+++[->+++<]>++.++++++++.+++++.--------.-[--->+<]>--.+[->+++<]>+.++++++++.";
  let mut tape = [0u8; 50000];
  let mut ptr = 0usize;
  let mut ctr = 0usize;

}
