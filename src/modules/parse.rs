use std::cell::RefCell;

use anyhow::{bail, Result, ensure};
use super::types;
use super::types::{Module, ModuleContent, ModuleType, Up2Header};

// Takes an up2 file (as a slice of bytes) and returns a header struct
pub fn parse_header(buf: &[u8]) -> Up2Header {
    // Create a new header struct
    let mut header = Up2Header {
        header_id: 0,
        major_version: 0,
        minor_version: 0,
        build_number: 0,
        rev: 0,
    };

    // Copy the header bytes into the header struct
    header.header_id = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    header.major_version = buf[4];
    header.minor_version = buf[5];
    header.build_number = buf[6];
    header.rev = buf[7];

    // Return the header struct
    header
}

pub struct Up2Parser {
    reader: RefCell<Box<dyn std::io::Read>>,
    offset: usize,
    pub header: Up2Header,
}

impl Up2Parser {
    pub fn new(reader: Box<dyn std::io::Read>) -> Result<Self> {
        let mut parser = Up2Parser {
            reader: RefCell::new(reader),
            offset: 0,
            header: Up2Header { header_id: 0, major_version: 0, minor_version: 0, build_number: 0, rev: 0 }
        };
        parser.header = parser.load_header()?;
        Ok(parser)
    }

    fn load_header(&mut self) -> Result<Up2Header> {
        // Read the first 8 bytes of the file
        let bytes = &mut self.reader.borrow_mut();
        let mut buf = [0; 8];
        bytes.read_exact(&mut buf)?;
        let header = parse_header(&buf);
        // 977358163 = "SMA:"
        if header.header_id != 977358163 {
            bail!("Invalid header id, file does not seem to be an up2 file");
        }
        Ok(header)
    }

    fn parse_module_body(
        &self,
        raw_data: &[u8],
        module_type: &ModuleType,
    ) -> Result<ModuleContent> {
        let len = raw_data.len();
        match module_type {
            ModuleType::LevelStartMt => {
                ensure!(len == 4, "Invalid level start module found!");
                Ok(ModuleContent::LevelStart(types::LevelStartModule {
                    label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                }))
            }
            ModuleType::LevelEndMt => {
                ensure!(len == 4, "Invalid level end module found!");
                Ok(ModuleContent::LevelEnd(types::LevelEndModule {
                    label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                }))
            }
            ModuleType::PauseMt => {
                ensure!(len == 4, "Invalid pause module found!");
                Ok(ModuleContent::Pause(types::PauseModule {
                    delay: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                }))
            }
            ModuleType::LoopStartMt => {
                ensure!(len == 4, "Invalid loop start module found!");
                Ok(ModuleContent::LoopStart(types::LoopStartModule {
                    label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                }))
            }
            ModuleType::LoopEndMt => {
                ensure!(len == 8, "Invalid loop end module found!");
                Ok(ModuleContent::LoopEnd(types::LoopEndModule {
                    label: u32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
                    loops: u32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
                }))
            }
            ModuleType::FirmwareverMt => {
                ensure!(len == 4, "Invalid firmware version module found!");
                Ok(ModuleContent::Firmwarever(types::FirmwareverModule {
                    major_version: raw_data[0],
                    minor_version: raw_data[1],
                    build_number: raw_data[2],
                    rev: raw_data[3],
                }))
            }
            ModuleType::TextMt => Ok(ModuleContent::Text(types::TextModule {
                data: String::from_utf8(raw_data.to_vec())
                    .expect("Found text module, but failed to convert to utf8"),
            })),
            ModuleType::LoginMt => {
                ensure!(len == 56, "Invalid login module found!");
                Ok(ModuleContent::Login(types::LoginModule {
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
                }))
            }
            ModuleType::FwChkMt => {
                ensure!(len == 60, "Invalid firmware check module found!");
                Ok(ModuleContent::FwChk(types::FwChkModule {
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
                }))
            }
            ModuleType::CondChkMt => {
                ensure!(len == 52, "Invalid condition check module found!");
                Ok(ModuleContent::CondChk(types::CondChkModule {
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
                }))
            }
            ModuleType::FirmwareMt => Ok(ModuleContent::Firmware(types::FirmwareModule {
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
            })),
            ModuleType::LogoutMt => {
                ensure!(len == 28, "Invalid logout module found!");
                Ok(ModuleContent::Logout(types::LogoutModule {
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
                }))
            }
            ModuleType::UpFmt10Mt => Ok(ModuleContent::UpFmt10(types::UpFmt10Module {
                data: raw_data.to_vec(),
            })),
            ModuleType::UnknownMt => Ok(ModuleContent::Unknown(types::UnknownModule {
                data: raw_data.to_vec(),
            })),
        }
    }
}

impl Iterator for Up2Parser {
    type Item = Result<Module>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = &mut self.reader.borrow_mut();
        let mut buf = [0; 16];
        match bytes.read_exact(&mut buf) {
            Ok(_) => {
                let mut module = Module {
                    header: super::types::ModuleHeader {
                        adler: 0,
                        module_type: 0,
                        susyid: 0,
                        len: 0,
                    },
                    content: ModuleContent::Unknown(super::types::UnknownModule { data: vec![] }),
                };
            
                // Copy the module header bytes into the module struct
                module.header.adler = u32::from_le_bytes([
                    buf[0],
                    buf[1],
                    buf[2],
                    buf[3],
                ]);
                module.header.module_type = u32::from_le_bytes([
                    buf[4],
                    buf[5],
                    buf[6],
                    buf[7],
                ]);
                module.header.susyid = u32::from_le_bytes([
                    buf[8],
                    buf[9],
                    buf[10],
                    buf[11],
                ]);
                module.header.len = u32::from_le_bytes([
                    buf[12],
                    buf[13],
                    buf[14],
                    buf[15],
                ]);
                // Copy the module content bytes into the module struct
                let module_type: ModuleType = module.header.module_type.into();
                let module_len = module.header.len;
                let mut buf = vec![0; module_len as usize];
                bytes.read_exact(&mut buf).unwrap();
                match self.parse_module_body(&buf, &module_type) {
                    Ok(content) => module.content = content,
                    Err(e) => return Some(Err(e)),
                }
                self.offset += 16 + module_len as usize;
                Some(Ok(module))
            }
            Err(_) => None,
        }
    }
}