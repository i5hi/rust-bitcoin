extern crate bitcoin;

use std::{env, process};

use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::consensus::encode::deserialize;

fn main() {
  // This exmaple reads a base64 PSBT from stdin
  // converts it to a native PSBT struct
  // and prints it to stdout.

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      eprintln!("Missing arguments. Usage: cargo run --example psbt <PSBT>");
      process::exit(1);
  }

  let decoded_psbt = match base64::decode(&args[1]){
    Ok(psbt)=> psbt,
    Err(_)=>{
      eprintln!("Error decoding psbt from base64");
      process::exit(1);
    }
  };

  let psbt_struct: PartiallySignedTransaction = match deserialize(&decoded_psbt){
    Ok(psbt)=>psbt,
    Err(_)=>{
      eprintln!("Error deserialising psbt");
      process::exit(1);
    }
  };
  println!("{:#?}", &psbt_struct);
}