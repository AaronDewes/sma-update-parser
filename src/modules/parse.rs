use super::types;
use super::types::{Module, ModuleContent, ModuleType, Up2File, Up2Header};

// Takes an up2 file (as a slice of bytes) and returns a header struct
pub fn parse_header(up2_file: &[u8]) -> Up2Header {
    // Create a new header struct
    let mut header = Up2Header {
        header_id: 0,
        major_version: 0,
        minor_version: 0,
        build_number: 0,
        rev: 0,
    };

    // Copy the header bytes into the header struct
    header.header_id = u32::from_le_bytes([up2_file[0], up2_file[1], up2_file[2], up2_file[3]]);
    header.major_version = up2_file[4];
    header.minor_version = up2_file[5];
    header.build_number = up2_file[6];
    header.rev = up2_file[7];

    // Return the header struct
    header
}

pub fn parse_module_body(
    up2_file: &[u8],
    module_type: &ModuleType,
    offset: usize,
    len: u32,
) -> ModuleContent {
    let raw_data = &up2_file[offset..offset + len as usize];
    match module_type {
        ModuleType::LevelStartMt => {
            assert_eq!(len, 4, "Invalid level start module found!");
            ModuleContent::LevelStart(types::LevelStartModule {
                label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
            })
        }
        ModuleType::LevelEndMt => {
            assert_eq!(len, 4, "Invalid level end module found!");
            ModuleContent::LevelEnd(types::LevelEndModule {
                label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
            })
        }
        ModuleType::PauseMt => {
            assert_eq!(len, 4, "Invalid pause module found!");
            ModuleContent::Pause(types::PauseModule {
                delay: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
            })
        }
        ModuleType::LoopStartMt => {
            assert_eq!(len, 4, "Invalid loop start module found!");
            ModuleContent::LoopStart(types::LoopStartModule {
                label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
            })
        }
        ModuleType::LoopEndMt => {
            assert_eq!(len, 8, "Invalid loop end module found!");
            ModuleContent::LoopEnd(types::LoopEndModule {
                label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                loops: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
            })
        }
        ModuleType::FirmwareverMt => {
            assert_eq!(len, 4, "Invalid firmware version module found!");
            ModuleContent::Firmwarever(types::FirmwareverModule {
                major_version: raw_data[0],
                minor_version: raw_data[1],
                build_number: raw_data[2],
                rev: raw_data[3],
            })
        }
        ModuleType::TextMt => ModuleContent::Text(types::TextModule {
            data: String::from_utf8(raw_data.to_vec())
                .expect("Found text module, but failed to convert to utf8"),
        }),
        ModuleType::LoginMt => {
            assert_eq!(len, 56, "Invalid login module found!");
            ModuleContent::Login(types::LoginModule {
                ctrl: u16::from_le_bytes([raw_data[0], raw_data[1]]),
                dst_susy: u16::from_le_bytes([raw_data[2], raw_data[3]]),
                dst_ser: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
                dst_dev: raw_data[8],
                dst_fkt: raw_data[9],
                src_susy: u16::from_le_bytes([raw_data[10], raw_data[11]]),
                src_ser: u32::from_le_bytes([
                    raw_data[12],
                    raw_data[13],
                    raw_data[14],
                    raw_data[15],
                ]),
                src_dev: raw_data[16],
                src_fkt: raw_data[17],
                cmd: raw_data[18],
                pcnt: raw_data[19],
                obj_num: u16::from_le_bytes([raw_data[20], raw_data[21]]),
                dat_len: u16::from_le_bytes([raw_data[22], raw_data[23]]),
                p0: u32::from_le_bytes([raw_data[24], raw_data[25], raw_data[26], raw_data[27]]),
                p1: u32::from_le_bytes([raw_data[28], raw_data[29], raw_data[30], raw_data[31]]),
                p2: u32::from_le_bytes([raw_data[32], raw_data[33], raw_data[34], raw_data[35]]),
                p3: u32::from_le_bytes([raw_data[36], raw_data[37], raw_data[38], raw_data[39]]),
                password: String::from_utf8(raw_data[40..52].to_vec())
                    .expect("Found login module, but failed to convert password to utf8"),
                mode: u32::from_le_bytes([raw_data[52], raw_data[53], raw_data[54], raw_data[55]]),
            })
        }
        ModuleType::FwChkMt => {
            assert_eq!(len, 60, "Invalid firmware check module found!");
            ModuleContent::FwChk(types::FwChkModule {
                ctrl: u16::from_le_bytes([raw_data[0], raw_data[1]]),
                dst_susy: u16::from_le_bytes([raw_data[2], raw_data[3]]),
                dst_ser: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
                dst_dev: raw_data[8],
                dst_fkt: raw_data[9],
                src_susy: u16::from_le_bytes([raw_data[10], raw_data[11]]),
                src_ser: u32::from_le_bytes([
                    raw_data[12],
                    raw_data[13],
                    raw_data[14],
                    raw_data[15],
                ]),
                src_dev: raw_data[16],
                src_fkt: raw_data[17],
                cmd: raw_data[18],
                pcnt: raw_data[19],
                obj_num: u16::from_le_bytes([raw_data[20], raw_data[21]]),
                dat_len: u16::from_le_bytes([raw_data[22], raw_data[23]]),
                p0: u32::from_le_bytes([raw_data[24], raw_data[25], raw_data[26], raw_data[27]]),
                blk_first: u32::from_le_bytes([
                    raw_data[28],
                    raw_data[29],
                    raw_data[30],
                    raw_data[31],
                ]),
                blk_last: u32::from_le_bytes([
                    raw_data[32],
                    raw_data[33],
                    raw_data[34],
                    raw_data[35],
                ]),
                cond_cnt: u16::from_le_bytes([raw_data[36], raw_data[37]]),
                crc: u16::from_le_bytes([raw_data[38], raw_data[39]]),
                adler32: u32::from_le_bytes([
                    raw_data[40],
                    raw_data[41],
                    raw_data[42],
                    raw_data[43],
                ]),
                md4: raw_data[44..60].try_into().unwrap(),
            })
        }
        ModuleType::CondChkMt => {
            assert_eq!(len, 52, "Invalid condition check module found!");
            ModuleContent::CondChk(types::CondChkModule {
                ctrl: u16::from_le_bytes([raw_data[0], raw_data[1]]),
                dst_susy: u16::from_le_bytes([raw_data[2], raw_data[3]]),
                dst_ser: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
                dst_dev: raw_data[8],
                dst_fkt: raw_data[9],
                src_susy: u16::from_le_bytes([raw_data[10], raw_data[11]]),
                src_ser: u32::from_le_bytes([
                    raw_data[12],
                    raw_data[13],
                    raw_data[14],
                    raw_data[15],
                ]),
                src_dev: raw_data[16],
                src_kkt: raw_data[17],
                cmd: raw_data[18],
                pcnt: raw_data[19],
                obj_num: u16::from_le_bytes([raw_data[20], raw_data[21]]),
                dat_len: u16::from_le_bytes([raw_data[22], raw_data[23]]),
                p0: u32::from_le_bytes([raw_data[24], raw_data[25], raw_data[26], raw_data[27]]),
                obj_nr: u16::from_le_bytes([raw_data[28], raw_data[29]]),
                rec_dw_first: u16::from_le_bytes([raw_data[30], raw_data[31]]),
                idx_first: u32::from_le_bytes([
                    raw_data[32],
                    raw_data[33],
                    raw_data[34],
                    raw_data[35],
                ]),
                bitmask: u32::from_le_bytes([
                    raw_data[36],
                    raw_data[37],
                    raw_data[38],
                    raw_data[39],
                ]),
                lo_bound: u32::from_le_bytes([
                    raw_data[40],
                    raw_data[41],
                    raw_data[42],
                    raw_data[43],
                ]),
                hi_bound: u32::from_le_bytes([
                    raw_data[44],
                    raw_data[45],
                    raw_data[46],
                    raw_data[47],
                ]),
                no_obj: raw_data[48],
                dat_valid: raw_data[49],
                res_1: raw_data[50],
                res_2: raw_data[51],
            })
        }
        ModuleType::FirmwareMt => ModuleContent::Firmware(types::FirmwareModule {
            ctrl: u16::from_le_bytes([raw_data[0], raw_data[1]]),
            dst_susy: u16::from_le_bytes([raw_data[2], raw_data[3]]),
            dst_ser: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
            dst_dev: raw_data[8],
            dst_fkt: raw_data[9],
            src_susy: u16::from_le_bytes([raw_data[10], raw_data[11]]),
            src_ser: u32::from_le_bytes([raw_data[12], raw_data[13], raw_data[14], raw_data[15]]),
            src_dev: raw_data[16],
            src_fkt: raw_data[17],
            cmd: raw_data[18],
            pcnt: raw_data[19],
            obj_num: u16::from_le_bytes([raw_data[20], raw_data[21]]),
            dat_len: u16::from_le_bytes([raw_data[22], raw_data[23]]),
            p0: u32::from_le_bytes([raw_data[24], raw_data[25], raw_data[26], raw_data[27]]),
            delay: u32::from_le_bytes([raw_data[28], raw_data[29], raw_data[30], raw_data[31]]),
            data: raw_data[32..].to_vec(),
        }),
        ModuleType::LogoutMt => {
            assert_eq!(len, 28, "Invalid logout module found!");
            ModuleContent::Logout(types::LogoutModule {
                ctrl: u16::from_le_bytes([raw_data[0], raw_data[1]]),
                dst_susy: u16::from_le_bytes([raw_data[2], raw_data[3]]),
                dst_ser: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
                dst_dev: raw_data[8],
                dst_fkt: raw_data[9],
                src_susy: u16::from_le_bytes([raw_data[10], raw_data[11]]),
                src_ser: u32::from_le_bytes([
                    raw_data[12],
                    raw_data[13],
                    raw_data[14],
                    raw_data[15],
                ]),
                src_dev: raw_data[16],
                src_fkt: raw_data[17],
                cmd: raw_data[18],
                pcnt: raw_data[19],
                obj_num: u16::from_le_bytes([raw_data[20], raw_data[21]]),
                dat_len: u16::from_le_bytes([raw_data[22], raw_data[23]]),
                p0: u32::from_le_bytes([raw_data[24], raw_data[25], raw_data[26], raw_data[27]]),
            })
        }
        ModuleType::UpFmt10Mt => ModuleContent::UpFmt10(types::UpFmt10Module {
            data: raw_data.to_vec(),
        }),
        ModuleType::UnknownMt => ModuleContent::Unknown(types::UnknownModule {
            data: raw_data.to_vec(),
        }),
    }
}

pub fn parse_module(up2_file: &[u8], offset: usize) -> Module {
    // Create a new module struct
    let mut module = Module {
        header: super::types::ModuleHeader {
            adler: 0,
            module_type: 0,
            susyid: 0,
            len: 0,
        },
        content: super::types::ModuleContent::Unknown(super::types::UnknownModule { data: vec![] }),
    };

    // Copy the module header bytes into the module struct
    module.header.adler = u32::from_le_bytes([
        up2_file[offset as usize],
        up2_file[offset as usize + 1],
        up2_file[offset as usize + 2],
        up2_file[offset as usize + 3],
    ]);
    module.header.module_type = u32::from_le_bytes([
        up2_file[offset as usize + 4],
        up2_file[offset as usize + 5],
        up2_file[offset as usize + 6],
        up2_file[offset as usize + 7],
    ]);
    module.header.susyid = u32::from_le_bytes([
        up2_file[offset as usize + 8],
        up2_file[offset as usize + 9],
        up2_file[offset as usize + 10],
        up2_file[offset as usize + 11],
    ]);
    module.header.len = u32::from_le_bytes([
        up2_file[offset as usize + 12],
        up2_file[offset as usize + 13],
        up2_file[offset as usize + 14],
        up2_file[offset as usize + 15],
    ]);
    // Copy the module content bytes into the module struct
    let module_type: ModuleType = module.header.module_type.into();
    //println!("Found {:#?} with length {}", module_type, module.header.len);

    module.content = parse_module_body(up2_file, &module_type, offset + 16, module.header.len);

    module
}

// Parse an up2 file and return a vector of modules
pub fn parse_up2(up2_file: &[u8]) -> Up2File {
    let header = parse_header(&up2_file[0..8]);
    if header.header_id != 977358163 {
        panic!("Invalid header id, file does not seem to be an up2 file");
    }
    let mut offset = 8;
    let mut modules = vec![];
    while offset + 16 < up2_file.len() {
        let module = parse_module(&up2_file, offset);
        offset += 16 + module.header.len as usize;
        modules.push(module);
    }
    Up2File { header, modules }
}
