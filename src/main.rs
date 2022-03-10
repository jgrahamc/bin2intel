// bin2intel - convert binary file to Intel HEX format for use with a
// PROM programmer
//
// Binary file on stdin, hex file is output on stdout with up to 32
// bytes from the binary file per row.
//
// Copyright (c) 2022 John Graham-Cumming

use std::io;
use std::io::Read;

// output outputs a single row of the hex file reading bytes to be
// output from v and setting the base address as a
fn output(a: u16, v: &Vec<u8>) {
   if v.len() == 0 {
      return;
   }

   // :XXYYYY00
   // XX   = number of bytes being output
   // YYYY = base address
   // 00   = Data record
   
   print!(":{:02X}{:04X}00", v.len(), a);

   let mut checksum: u16 = 0;
   checksum += v.len() as u16;
   checksum += a & 0xff;
   checksum += a >> 8;
   
   for b in v.iter() {
       checksum += *b as u16;
       print!("{:02X}", b);
   }

   // Checksum is two's-complement of bottom byte of sum of the bytes
   // that were output
   
   checksum ^= 0xffff;
   checksum += 1;
   checksum &= 0xff;
   println!("{:02X}", checksum);
}

fn main() {
   let mut row: Vec<u8> = Vec::with_capacity(32);
   let mut a: u16 = 0;

   for b in io::stdin().bytes() {
       row.push(b.unwrap() as u8);  
       if row.len() == row.capacity() {
       	  output(a, &row);
	  a += row.len() as u16;
      	  row.clear();
       }
    }

    output(a, &row);

    // Terminator row. 0 bytes at address 0 with record type 01
    
    println!(":00000001FF");
}
