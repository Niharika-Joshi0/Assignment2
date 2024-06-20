use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::{ Person,PersonList};

use clap::{App, Arg};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader,Write};
use std::fmt;
use serde_json;

#[derive(Debug)]
struct out{
    outsize:usize,
    bytearr:Vec<u8>
}

impl fmt::Display for out {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} ", self.outsize, self.bytearr)
    }
}

fn main() {
    let matches = App::new("Data Processing Tool")
                    .version("1.0")
                    .author("Your Name")
                    .about("An example CLI tool for processing employee data")
                    .arg(Arg::with_name("input-file-path")
                    .short('i')
                        .long("input-file-path")
                        .value_name("FILE")
                        .help("input file tp be read")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("output-file-path")
                        .short('o')
                        .long("output-file-path")
                        .value_name("FILE")
                        .help("Output file to write into")
                        .takes_value(true)
                        .required(true))
                    .get_matches();

                let input_file = PathBuf::from(matches.value_of("input-file-path").unwrap());
                let output_file = PathBuf::from(matches.value_of("output-file-path").unwrap());

                let _data=read_data(&input_file);
                // println!("DATA: {:?}",_data);
                let _r=generate_output(_data,&output_file);
                let _o=read_output_data_and_write(&output_file);
                // println!("{:?}",o);

}


fn read_data(file_path: &PathBuf)->Vec<out>{
    use std::fs::File;
    use std::io::{BufReader, BufRead};



    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    // let mut o1=out::new();

    let mut out_msgnew = PersonList::new();
    // let mut arrays: Vec<Vec<u8>> = Vec::new();

    let mut arrays: Vec<out> = Vec::new();

    


    let mut lines = reader.lines();

        for line in lines {
            if let Ok(line) = line {
                let fields: Vec<&str> = line.split(',').collect();
                if fields.len() >= 3 {
                    let mut out_msg = Person::new();
                    out_msg.lname = fields[0].to_string();
                    out_msg.fname = fields[1].to_string();
                    out_msg.dob = fields[2].to_string();

                    let out_bytes1: Vec<u8> = out_msg.write_to_bytes().unwrap();
                    // println!("Message request in bytes:\nout_bytes {:?}", out_bytes1);
                    let s=out_bytes1.len();

                    out_msgnew.people.push(out_msg);
                    let mut o1=out{
                        outsize:s,
                        bytearr:out_bytes1
                    };
                    arrays.push(o1);

                    
                    // println!("ARRAYS:{:?}",arrays);

                    // Print or process out_msgnew here
                    
                    // // Decode example request
                    // let in_msg = Person::parse_from_bytes(&out_bytes).unwrap();
                }
            }
        }
        // let out_bytes: Vec<u8> = out_msgnew.write_to_bytes().unwrap();
        let size=arrays.len();
        arrays
        
                
    }





fn generate_output(data: Vec<out>, output_file: &PathBuf) -> Result<(), std::io::Error> {
    let mut ofile = File::create(output_file)?;
    // println!("GENOUT{:?}",data);
    for item in &data {
        writeln!(
            ofile,
            "{}",
            item 
        )?;
    }
    
    println!("Data written into {:?}", output_file);
    Ok(())
}


fn read_output_data_and_write(output_file: &Path) -> Result<(), std::io::Error> {
    let input_file = File::open(output_file)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create("../files/finalout.txt")?;

    for line in reader.lines() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.splitn(2, ' ').collect();
            if fields.len() == 2 {
                let size: usize = fields[0].parse().unwrap_or(0);
                let bytearr_str = fields[1].trim();

                if let Ok(bytearr) = serde_json::from_str::<Vec<u8>>(bytearr_str) {
                    if !bytearr.is_empty() {
                        if let Ok(person) = Person::parse_from_bytes(&bytearr) {
                            writeln!(
                                &mut output_file,
                                "{},{},{}",
                                person.lname, person.fname, person.dob
                            )?;
                        } else {
                            println!("Failed to parse Person from byte array");
                        }
                    } else {
                        println!("Empty byte array");
                    }
                } else {
                    println!("Failed to deserialize byte array JSON");
                }
            } else {
                println!("Invalid line format: {:?}", line);
            }
        } else {
            println!("Failed to read line");
        }
    }

    println!("Data written into new output file");
    Ok(())
}