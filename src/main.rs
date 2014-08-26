use parser::parse_stream;
use bf::run_program;
use std::os::args;
use std::io::fs::File;
use std::path::posix::Path;

#[main]
fn main() {
  let arg_vec = args();
  if arg_vec.len() < 1 {
    return;
  }
  
  let file_name = Path::new( arg_vec[1].clone() );
  let mut file = match File::open( &file_name ) {
    Ok(f) => f,
    Err(e) => {
      println!( "Error: {}", e );
      return;
    }
  };

  let program = match parse_stream( file.bytes() ) {
    Some(p) => p,
    None => return
  };

  run_program( program );


}
