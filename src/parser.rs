use bf::{BFCommand, IncPnt, DecPnt, IncData, DecData, OutData, InData, LoopStart, LoopStartMarker, LoopEnd};
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
    let cmd = match token {
      b'>' => IncPnt,
      b'<' => DecPnt,
      b'+' => IncData,
      b'-' => DecData,
      b'.' => OutData,
      b',' => InData,
      b'[' => {
        loop_stack.push( inc_ptr );
        LoopStartMarker
      }
      b']' => {
        let ptr = match loop_stack.pop() {
          Some(p) => p,
          None => return None
        };
        {
          let ref_slice: &mut [BFCommand] = program.as_mut_slice();
          match ref_slice.get_mut( ptr ) {
            Some( ref mut loop_start ) if **loop_start == LoopStartMarker => {
              **loop_start = LoopStart( inc_ptr );
            },
            _ => {}
          }
        }
        LoopEnd( ptr )
      },
      _ => continue
    };
    program.push( cmd );
    inc_ptr += 1;
  }

  Some( program )
}
