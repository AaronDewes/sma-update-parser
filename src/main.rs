use sma_update_parser::modules::parse::parse_up2;
use sma_update_parser::modules::types::ModuleContent;

// A CLI util to parse a SMA update file
use std::fs::File;
use std::io::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parses an update file and dumps some information about it
    Parse {
        /// The path to the update file
        path: String,
        /// The path to dump the raw firmware to (optional)
       dump: Option<String>,
    },
}


fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse { path, dump } => {
            // Open the file
            let mut file = File::open(path).expect("Unable to open file");

            // Read the file into a buffer
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("Unable to read file");

            // Parse the header
            let file = parse_up2(&buffer);

            let header = file.header;

            // Print the header
            println!("Header ID: 0x{:x}", header.header_id);
            println!("Major Version: {}", header.major_version);
            println!("Minor Version: {}", header.minor_version);
            println!("Build Number: {}", header.build_number);
            println!("Revision: {}", header.rev);

            for module in file.modules {
                match module.content {
                    ModuleContent::Firmwarever(firmwarever) => {
                        println!("Firmware Version: {:#?}", firmwarever);
                    }
                    ModuleContent::LevelStart(level_start) => {
                        println!("Level Start: {:#?}", level_start.label);
                    },
                    ModuleContent::LevelEnd(level_end) => {
                        println!("Level End: {:#?}", level_end.label);
                    },
                    ModuleContent::Pause(pause) => {
                        println!("Pause: {:#?}", pause.delay);
                    },
                    ModuleContent::LoopStart(loop_start) => {
                        println!("Loop Start: {:#?}", loop_start.label);
                    },
                    ModuleContent::LoopEnd(loop_end) => {
                        println!("Loop End: {:#?}", loop_end.label);
                    },
                    ModuleContent::Text(text) => {
                        println!("Text: {:#?}", text.data);
                    },
                    ModuleContent::Login(login) => {
                        println!("Login: {:#?}", login);
                    },
                    ModuleContent::FwChk(fw_chk) => {
                        println!("FwChk: {:#?}", fw_chk);
                    },
                    ModuleContent::CondChk(cond_chk) => {
                        println!("CondChk: {:#?}", cond_chk);
                    },
                    ModuleContent::Firmware(firmware) => {
                        // the firmware for some devices consists of blocks that contain a 4 bytes uint that seems to be the block count, then 128 bytes of data
                        let mut blocks = Vec::new();
                        for (i, byte) in firmware.data.chunks(132).enumerate() {
                            let p0 = u32::from_le_bytes([byte[0], byte[1], byte[2], byte[3]]);
                            assert_eq!(i as u32, p0, "Block count does not match p0 present in block");
                            // Remove the p0 bytes
                            let block = byte[4..].to_vec();
                            blocks.push(block);
                        }
                        println!("Firmware: {} blocks", blocks.len());
                        // If dump_blocks is set, join the blocks and dump them to the specified path
                        if let Some(ref dump_path) = dump {
                            let mut file = File::create(dump_path).expect("Unable to create file");
                            file.write_all(&blocks.concat()).expect("Unable to write file");
                        }
                    },
                    ModuleContent::Logout(logout) => {
                        println!("Logout: {:#?}", logout);
                    },
                    ModuleContent::UpFmt10(up_fmt10) => {
                        println!("UpFmt10: {:#?}", up_fmt10);
                    },
                    ModuleContent::Unknown(unknown) => {
                        eprintln!("Unknown module: {:#?}", unknown);
                    },
                }
            }
        }
    }
}
