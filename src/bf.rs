use std::io::stdio::stdin_raw;
use std::io::stdio::stdout_raw;

#[deriving(PartialEq)]
pub enum BFCommand {
  IncPnt,
  DecPnt,
  IncData,
  DecData,
  OutData,
  InData,
  LoopStart( uint ),
  LoopStartMarker,
  LoopEnd( uint )
}

pub fn run_program( prog: Vec<BFCommand> ) {
  let mut data: Vec<u8> = Vec::new();
  let mut ptr = 0u;
  let mut inc_ptr = 0u;
  let mut input = stdin_raw();
  let mut output = stdout_raw();
  while inc_ptr < prog.len() {
    match prog[inc_ptr] {
      IncPnt => ptr += 1,
      DecPnt => ptr -= 1,
      IncData => data_inc( ptr, &mut data ),
      DecData => data_dec( ptr, &mut data ),
      InData => data.grow_set( ptr, &0, in_byte( &mut input ) ),
      OutData => out_byte( &mut output, *data.as_slice().get(ptr).unwrap_or(&0) ),
      LoopStart( i ) => {
        if *vec_get( &mut data, ptr ) == 0 {
          inc_ptr = i + 1;
          continue;
        }
      }
      LoopStartMarker => break,
      LoopEnd( i ) => {
        if *vec_get( &mut data, ptr ) != 0 {
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

fn data_inc( index: uint, data: &mut Vec<u8> ) {
  let cur_data = vec_get( data, index ).clone();
  *data.get_mut( index ) = cur_data + 1;
}

fn data_dec( index: uint, data: &mut Vec<u8> ) {
  let cur_data = vec_get( data, index ).clone();
  *data.get_mut( index ) = cur_data - 1;
}

fn vec_get<'a>( data: &'a mut Vec<u8>, index: uint ) -> &'a u8 {
  if data.len() <= index {
    let new_len = 1 + index - data.len();
    data.grow( new_len, &0 );
  }
  &(*data)[index]
}
