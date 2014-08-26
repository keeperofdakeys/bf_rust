use bf::{BFCommand, IncPnt, DecPnt, IncData, DecData, OutData, InData, LoopStart, LoopEnd};
use std::io::IoResult;

pub fn parse_stream<T: Iterator<IoResult<u8>>>( mut input: T ) -> Option<Vec<BFCommand>> {
  let mut program = Vec::new();
  let mut loop_stack = Vec::new();
  let mut inc_ptr = 0u;

  for result_token in input {
    let token = match result_token {
      Ok(t) => t,
      Err(_) => break
    };
    program.push(
      match token {
        b'>' => IncPnt,
        b'<' => DecPnt,
        b'+' => IncData,
        b'-' => DecData,
        b'.' => OutData,
        b',' => InData,
        b'[' => {
          loop_stack.push( inc_ptr );
          LoopStart( inc_ptr )
        }
        b']' => {
          let ptr = match loop_stack.pop() {
            Some(p) => p,
            None => return None
          };
          LoopEnd( ptr )
        },
        _ => continue
      }
    );
    inc_ptr += 1;
  }

  Some( program )
}
