use skyline::hooks::{getRegionAddress, Region};

// Default 11.0.1 offsets
pub static mut LADDER_01_POSITION_OFFSET: usize = 0x4334cc0;
pub static mut LADDER_02_POSITION_OFFSET: usize = 0x4339670;
pub static mut LADDER_03_POSITION_OFFSET: usize = 0x43372a0;
pub static mut LADDER_04_POSITION_OFFSET: usize = 0x433a490;
pub static mut LADDER_05_POSITION_OFFSET: usize = 0x43369d0;
pub static mut LADDER_06_POSITION_OFFSET: usize = 0x4338b30;
pub static mut LADDER_07_POSITION_OFFSET: usize = 0x4339c00;
pub static mut LADDER_08_POSITION_OFFSET: usize = 0x43385a0;
pub static mut LADDER_09_POSITION_OFFSET: usize = 0x4336fd0;
pub static mut LADDER_10_POSITION_OFFSET: usize = 0x4334fd0;
pub static mut OTHER_MODEL_POSITION_OFFSET: usize = 0x441c860;

pub static mut LADDER_01_08_PARAMETER_OFFSET: usize = 0x4335c10;
pub static mut LADDER_02_PARAMETER_OFFSET: usize = 0x43358f0;
pub static mut LADDER_03_04_PARAMETER_OFFSET: usize = 0x4336160;
pub static mut LADDER_05_07_09_PARAMETER_OFFSET: usize = 0x4335c20;
pub static mut LADDER_06_PARAMETER_OFFSET: usize = 0x4337d44;
pub static mut LADDER_PARAMETER_INLINE_HOOK_OFFSET: usize = 0x25d219c;

pub static mut SCORE_INCREMENT_VALUE_INLINE_HOOK_OFFSET: usize = 0x25d2780;

pub static mut DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET: usize = 0x25d0f68;

pub static mut JACK_SE_HASH40_INLINE_HOOK_OFFSET: usize = 0x2398810;
pub static mut JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_OFFSET: usize = 0x25d4b98;
pub static mut JACK_HITBOX_JOINT_INLINE_HOOK_OFFSET: usize = 0x25d5018;
pub static mut JACK_HITBOX_RELATIVE_Y_POSITION_OFFSET: usize = 0x4335250;

static LADDER_01_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xfd, 0xc2, 
    0x00, 0x00, 0x80, 0x41
];

static LADDER_02_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xe9, 0xc2, 
    0x00, 0x00, 0x60, 0x42
];

static LADDER_03_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x62, 0xc2, 
    0x00, 0x00, 0x20, 0x42
];

static LADDER_04_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x12, 0xc2, 
    0x00, 0x00, 0x20, 0x42
];

static LADDER_05_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x06, 0x42, 
    0x00, 0x00, 0xa0, 0x42
];

static LADDER_06_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xf7, 0x42, 
    0x00, 0x00, 0x20, 0x42
];

static LADDER_07_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xba, 0x42, 
    0x00, 0x00, 0x80, 0x42
];

static LADDER_08_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xf7, 0x42, 
    0x00, 0x00, 0xc0, 0x42
];

static LADDER_09_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x7e, 0x42, 
    0x00, 0x00, 0xf0, 0x42
];

static LADDER_10_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xbc, 0x41, 
    0x00, 0x00, 0x18, 0x43
];

static OTHER_MODEL_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xb7, 0xc2, 
    0x00, 0x00, 0x18, 0x43
];

static LADDER_01_08_PARAMETER_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x20, 0x41, 
    0x00, 0x00, 0x20, 0x42
];

static LADDER_02_PARAMETER_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x20, 0x41, 
    0x00, 0x00, 0x40, 0x42
];

static LADDER_03_04_PARAMETER_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x20, 0x41, 
    0x00, 0x00, 0x80, 0x42
];

static LADDER_05_07_09_PARAMETER_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x20, 0x41, 
    0x00, 0x00, 0x00, 0x42
];

static LADDER_06_PARAMETER_SEARCH_CODE: &[u8] = &[
    0xcd, 0xcc, 0x7c, 0x41
];

static LADDER_PARAMETER_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0xe0, 0x03, 0x1c, 0x32, 
    0xe0, 0x07, 0x00, 0xad
];

static SCORE_INCREMENT_VALUE_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0x1f, 0x01, 0x14, 0x6b, 
    0x08, 0x31, 0x94, 0x1a, 
    0x3f, 0x01, 0x08, 0x6b
];

static DONKEYKONG_SE_POUND_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0xe1, 0x03, 0x00, 0x91, 
    0xf7, 0x03, 0x00, 0xb9, 
    0xff, 0x0f, 0x00, 0xb9
];

static JACK_SE_HASH40_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0x08, 0xf6, 0x8c, 0xd2
];

static JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0xfa, 0xa3, 0x09, 0xa9, 
    0xe8, 0x13, 0x00, 0x32
];

static JACK_HITBOX_JOINT_INLINE_HOOK_SEARCH_CODE: &[u8] = &[
    0x01, 0x51, 0x96, 0xd2
];

static JACK_HITBOX_RELATIVE_Y_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x00, 0x41, 
    0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 
    0xa9, 0x84, 0x5b, 0xbf
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

macro_rules! find_offsets_rodata {
    (
        $(
            ($out_variable:expr, $search_pattern:expr)
        ),*
        $(,)?
    ) => {
        $(
            unsafe {
                let text_ptr = getRegionAddress(Region::Rodata) as *const u8;
                let rodata_offset = (getRegionAddress(Region::Rodata) as usize) - (getRegionAddress(Region::Text) as usize);
                let text_size = (getRegionAddress(Region::Data) as usize) - (text_ptr as usize);
                let text = std::slice::from_raw_parts(text_ptr, text_size);

                if let Some(offset) = find_subsequence(text, $search_pattern) {
                    $out_variable = offset + rodata_offset
                } else {
                    println!("Error: no offset found for '{}'. Defaulting to 11.0.1 offset. This most likely won't work.", stringify!($out_variable));
                }
            }
        )*
    };
}

macro_rules! find_offsets_text {
    (
        $(
            ($out_variable:expr, $search_pattern:expr)
        ),*
        $(,)?
    ) => {
        $(
            unsafe {
                let text_ptr = getRegionAddress(Region::Text) as *const u8;
                let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
                let text = std::slice::from_raw_parts(text_ptr, text_size);

                if let Some(offset) = find_subsequence(text, $search_pattern) {
                    $out_variable = offset
                } else {
                    println!("Error: no offset found for '{}'. Defaulting to 11.0.1 offset. This most likely won't work.", stringify!($out_variable));
                }
            }
        )*
    };
}

pub fn search_offsets() {
    find_offsets_rodata!(
        (LADDER_01_POSITION_OFFSET, LADDER_01_POSITION_SEARCH_CODE),
        (LADDER_02_POSITION_OFFSET, LADDER_02_POSITION_SEARCH_CODE),
        (LADDER_03_POSITION_OFFSET, LADDER_03_POSITION_SEARCH_CODE),
        (LADDER_04_POSITION_OFFSET, LADDER_04_POSITION_SEARCH_CODE),
        (LADDER_05_POSITION_OFFSET, LADDER_05_POSITION_SEARCH_CODE),
        (LADDER_06_POSITION_OFFSET, LADDER_06_POSITION_SEARCH_CODE),
        (LADDER_07_POSITION_OFFSET, LADDER_07_POSITION_SEARCH_CODE),
        (LADDER_08_POSITION_OFFSET, LADDER_08_POSITION_SEARCH_CODE),
        (LADDER_09_POSITION_OFFSET, LADDER_09_POSITION_SEARCH_CODE),
        (LADDER_10_POSITION_OFFSET, LADDER_10_POSITION_SEARCH_CODE),
        (OTHER_MODEL_POSITION_OFFSET, OTHER_MODEL_POSITION_SEARCH_CODE),
        (LADDER_01_08_PARAMETER_OFFSET, LADDER_01_08_PARAMETER_SEARCH_CODE),
        (LADDER_02_PARAMETER_OFFSET, LADDER_02_PARAMETER_SEARCH_CODE),
        (LADDER_03_04_PARAMETER_OFFSET, LADDER_03_04_PARAMETER_SEARCH_CODE),
        (LADDER_05_07_09_PARAMETER_OFFSET, LADDER_05_07_09_PARAMETER_SEARCH_CODE),
        (LADDER_06_PARAMETER_OFFSET, LADDER_06_PARAMETER_SEARCH_CODE),
        (JACK_HITBOX_RELATIVE_Y_POSITION_OFFSET, JACK_HITBOX_RELATIVE_Y_POSITION_SEARCH_CODE)
    );
    find_offsets_text! (
        (LADDER_PARAMETER_INLINE_HOOK_OFFSET, LADDER_PARAMETER_INLINE_HOOK_SEARCH_CODE),
        (SCORE_INCREMENT_VALUE_INLINE_HOOK_OFFSET, SCORE_INCREMENT_VALUE_INLINE_HOOK_SEARCH_CODE),
        (DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET, DONKEYKONG_SE_POUND_INLINE_HOOK_SEARCH_CODE),
        (JACK_SE_HASH40_INLINE_HOOK_OFFSET, JACK_SE_HASH40_INLINE_HOOK_SEARCH_CODE),
        (JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_OFFSET, JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_SEARCH_CODE),
        (JACK_HITBOX_JOINT_INLINE_HOOK_OFFSET, JACK_HITBOX_JOINT_INLINE_HOOK_SEARCH_CODE)
    );
}