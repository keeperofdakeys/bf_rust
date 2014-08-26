use std::io::stdio::stdin_raw;
use std::io::stdio::stdout_raw;

pub enum BFCommand {
  IncPnt,
  DecPnt,
  IncData,
  DecData,
  OutData,
  InData,
  LoopStart( uint ),
  LoopEnd( uint )
}

pub fn run_program( prog: Vec<BFCommand> ) {
  let mut data: Vec<u8> = Vec::from_elem( 1000000, 0 );
  let mut ptr = 0u;
  let mut inc_ptr = 0u;
  let mut input = stdin_raw();
  let mut output = stdout_raw();
  while inc_ptr < prog.len() {
    match prog[inc_ptr] {
      IncPnt => ptr += 1,
      DecPnt => ptr -= 1,
      IncData => *data.get_mut( ptr ) += 1,
      DecData => *data.get_mut( ptr ) -= 1,
      InData => *data.get_mut( ptr ) = in_byte( &mut input ),
      OutData => out_byte( &mut output, data[ptr] ),
      LoopStart( i ) => {
        if data[ptr] == 0 {
          inc_ptr = i + 1;
          continue;
        }
      }
      LoopEnd( i ) => {
        if data[ptr] != 0 {
          inc_ptr = i + 1;
          continue;
        }
      }
    }
    inc_ptr += 1
  }
}

fn in_byte<T: Reader>( input: &mut T ) -> u8 { 
  match input.read_byte() {
    Ok(c) => c,
    Err(_) => 0
  }
}

fn out_byte<T: Writer>( output: &mut T, byte: u8 ) {
  let _ = output.write_u8( byte );
}
