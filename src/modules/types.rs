#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ModuleType {
    LevelStartMt = 0x0000,
    LevelEndMt = 0x0001,
    PauseMt = 0x0002,
    LoopStartMt = 0x0003,
    LoopEndMt = 0x0004,
    FirmwareverMt = 0x1000,
    TextMt = 0x1001,
    LoginMt = 0x2000,
    FwChkMt = 0x2001,
    CondChkMt = 0x2002,
    FirmwareMt = 0x2003,
    LogoutMt = 0x2004,
    UpFmt10Mt = 0x3000,
    UnknownMt = u32::MAX,
}

impl From<u32> for ModuleType {
    fn from(v: u32) -> Self {
        match v {
            0x0000 => ModuleType::LevelStartMt,
            0x0001 => ModuleType::LevelEndMt,
            0x0002 => ModuleType::PauseMt,
            0x0003 => ModuleType::LoopStartMt,
            0x0004 => ModuleType::LoopEndMt,
            0x1000 => ModuleType::FirmwareverMt,
            0x1001 => ModuleType::TextMt,
            0x2000 => ModuleType::LoginMt,
            0x2001 => ModuleType::FwChkMt,
            0x2002 => ModuleType::CondChkMt,
            0x2003 => ModuleType::FirmwareMt,
            0x2004 => ModuleType::LogoutMt,
            0x3000 => ModuleType::UpFmt10Mt,
            _ => ModuleType::UnknownMt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Up2Header {
    /// Should be 977358163 (0x3a414d53) for a valid header
    pub header_id: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub build_number: u8,
    pub rev: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub header: ModuleHeader,
    pub content: ModuleContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleHeader {
    pub adler: u32,
    pub module_type: u32,
    pub susyid: u32,
    pub len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleContent {
    LevelStart(LevelStartModule),
    LevelEnd(LevelEndModule),
    Pause(PauseModule),
    LoopStart(LoopStartModule),
    LoopEnd(LoopEndModule),
    Firmwarever(FirmwareverModule),
    Text(TextModule),
    Login(LoginModule),
    FwChk(FwChkModule),
    CondChk(CondChkModule),
    Firmware(FirmwareModule),
    Logout(LogoutModule),
    UpFmt10(UpFmt10Module),
    Unknown(UnknownModule),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelStartModule {
    pub label: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelEndModule {
    pub label: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauseModule {
    pub delay: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopStartModule {
    pub label: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopEndModule {
    pub label: u32,
    pub loops: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareverModule {
    pub major_version: u8,
    pub minor_version: u8,
    pub build_number: u8,
    pub rev: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextModule {
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginModule {
    pub ctrl: u16,
    pub dst_susy: u16,
    pub dst_ser: u32,
    pub dst_dev: u8,
    pub dst_fkt: u8,
    pub src_susy: u16,
    pub src_ser: u32,
    pub src_dev: u8,
    pub src_fkt: u8,
    pub cmd: u8,
    pub pcnt: u8,
    pub obj_num: u16,
    pub dat_len: u16,
    pub p0: u32,
    pub p1: u32,
    pub p2: u32,
    pub p3: u32,
    pub password: String,
    pub mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FwChkModule {
    pub ctrl: u16,
    pub dst_susy: u16,
    pub dst_ser: u32,
    pub dst_dev: u8,
    pub dst_fkt: u8,
    pub src_susy: u16,
    pub src_ser: u32,
    pub src_dev: u8,
    pub src_fkt: u8,
    pub cmd: u8,
    pub pcnt: u8,
    pub obj_num: u16,
    pub dat_len: u16,
    pub p0: u32,
    pub blk_first: u32,
    pub blk_last: u32,
    pub cond_cnt: u16,
    pub crc: u16,
    pub adler32: u32,
    pub md4: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondChkModule {
    pub ctrl: u16,
    pub dst_susy: u16,
    pub dst_ser: u32,
    pub dst_dev: u8,
    pub dst_fkt: u8,
    pub src_susy: u16,
    pub src_ser: u32,
    pub src_dev: u8,
    // These names are internal names from reverse-engineering the firmware
    // This could be a typo, but I'm not sure
    // it maybe should be src_fkt
    pub src_kkt: u8,
    pub cmd: u8,
    pub pcnt: u8,
    pub obj_num: u16,
    pub dat_len: u16,
    pub p0: u32,
    pub obj_nr: u16,
    pub rec_dw_first: u16,
    pub idx_first: u32,
    pub bitmask: u32,
    pub lo_bound: u32,
    pub hi_bound: u32,
    pub no_obj: u8,
    pub dat_valid: u8,
    pub res_1: u8,
    pub res_2: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareModule {
    pub ctrl: u16,
    pub dst_susy: u16,
    pub dst_ser: u32,
    pub dst_dev: u8,
    pub dst_fkt: u8,
    pub src_susy: u16,
    pub src_ser: u32,
    pub src_dev: u8,
    pub src_fkt: u8,
    pub cmd: u8,
    pub pcnt: u8,
    pub obj_num: u16,
    pub dat_len: u16,
    pub p0: u32,
    pub delay: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogoutModule {
    pub ctrl: u16,
    pub dst_susy: u16,
    pub dst_ser: u32,
    pub dst_dev: u8,
    pub dst_fkt: u8,
    pub src_susy: u16,
    pub src_ser: u32,
    pub src_dev: u8,
    pub src_fkt: u8,
    pub cmd: u8,
    pub pcnt: u8,
    pub obj_num: u16,
    pub dat_len: u16,
    pub p0: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpFmt10Module {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Up2File {
    pub header: Up2Header,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownModule {
    pub data: Vec<u8>,
}
