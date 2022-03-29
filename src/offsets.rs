use skyline::hooks::{getRegionAddress, Region};

// Default 13.0.1 offsets
pub static mut LADDER_SETUP_OFFSET: usize = 0x2659CF0;
pub static mut LADDER_10_INSTRUCTION_PATCH_OFFSET: usize = 0x265517C;
pub static mut SCORE_INCREMENT_VALUE_OFFSET: usize = 0x26552C0;
pub static mut SOUND_EFFECT_SEQUENCE_SETUP_OFFSET: usize = 0x265AD00;
pub static mut JACK_ATTACK_DATA_02_OFFSET: usize = 0x2657AE8;
pub static mut JACK_ATTACK_JOINT_HASH_OFFSET: usize = 0x2657B70;
pub static mut JACK_ATTACK_POSITION_OFFSET: usize = 0x4463CE0;
pub static mut JACK_ATTACK_SWAP_FRAME_OFFSET: usize = 0x2657EAC;
pub static mut JACK_SE_FALL_FRAME_OFFSET: usize = 0x2658934;
pub static mut OTHER_MODEL_POSITION_OFFSET: usize = 0x454EFD0;

static LADDER_SETUP_SEARCH_CODE: &[u8] = &[
    0xFF, 0x43, 0x02, 0xD1,
    0xF7, 0x2B, 0x00, 0xF9,
    0xF6, 0x57, 0x06, 0xA9,
    0xF4, 0x4F, 0x07, 0xA9,
    0xFD, 0x7B, 0x08, 0xA9,
    0xFD, 0x03, 0x02, 0x91,
    0xE8, 0x03, 0x00, 0xB2,
];

static LADDER_10_INSTRUCTION_PATCH_SEARCH_CODE: &[u8] = &[
    0x74, 0xBE, 0x44, 0xF9,
    0x34, 0x04, 0x00, 0xB4,
];

static SCORE_INCREMENT_VALUE_SEARCH_CODE: &[u8] = &[
    0x1F, 0x01, 0x14, 0x6B,
    0x08, 0x31, 0x94, 0x1A,
    0x3F, 0x01, 0x08, 0x6B,
];

static SOUND_EFFECT_SEQUENCE_SETUP_SEARCH_CODE: &[u8] = &[
    0xFF, 0x83, 0x01, 0xD1,
    0xFA, 0x67, 0x01, 0xA9,
    0xF8, 0x5F, 0x02, 0xA9,
    0xF6, 0x57, 0x03, 0xA9,
    0xF4, 0x4F, 0x04, 0xA9,
    0xFD, 0x7B, 0x05, 0xA9,
    0xFD, 0x43, 0x01, 0x91,
    0x09, 0x20, 0x41, 0xA9,
    0xF5, 0x03, 0x01, 0xAA,
    0xF3, 0x03, 0x00, 0xAA,
    0x3F, 0x01, 0x08, 0xEB,
    0xA0, 0x01, 0x00, 0x54,
];

static JACK_ATTACK_DATA_02_SEARCH_CODE: &[u8] = &[
    0xE1, 0x03, 0x1F, 0x2A,
    0xE2, 0x03, 0x1F, 0x2A,
    0x00, 0x01, 0x3F, 0xD6,
    0x74, 0x72, 0x40, 0xF9,
];

static JACK_ATTACK_JOINT_HASH_SEARCH_CODE: &[u8] = &[
    0x01, 0x51, 0x96, 0xD2,
    0x01, 0x24, 0xA0, 0xF2,
    0x01, 0x02, 0xC0, 0xF2,
];

static JACK_ATTACK_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x41,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

static JACK_ATTACK_SWAP_FRAME_SEARCH_CODE: &[u8] = &[
    0x00, 0x20, 0x21, 0x1E,
    0x25, 0x09, 0x00, 0x54,
    0x60, 0x6E, 0x40, 0xF9,
];

static JACK_SE_FALL_FRAME_SEARCH_CODE: &[u8] = &[
    0x00, 0x20, 0x21, 0x1E,
    0xED, 0x07, 0x00, 0x54,
    0x68, 0xA2, 0x40, 0xB9,
];

static OTHER_MODEL_POSITION_SEARCH_CODE: &[u8] = &[
    0x00, 0x00, 0xB7, 0xC2,
    0x00, 0x00, 0x18, 0x43,
    0x00, 0x00, 0x10, 0xC1,
    0x00, 0x00, 0x00, 0x00,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

macro_rules! find_offsets {
    (
        $(
            ($out_variable:expr, $search_pattern:expr)
        ),*
        $(,)?
    ) => {
        $(
            unsafe {
                let text_ptr = getRegionAddress(Region::Text) as *const u8;
                let data_ptr = getRegionAddress(Region::Data) as usize;
                let search_size = data_ptr - (text_ptr as usize);
                let search_slice = std::slice::from_raw_parts(text_ptr, search_size);

                if let Some(offset) = find_subsequence(search_slice, $search_pattern) {
                    $out_variable = offset;
                } else {
                    println!("[stage_25m] Error: no offset found for '{}'. Defaulting to 13.0.1 offset. This most likely won't work.", stringify!($out_variable));
                }
            }
        )*
    };
}

pub fn search_offsets() {
    find_offsets! (
        (LADDER_SETUP_OFFSET, LADDER_SETUP_SEARCH_CODE),
        (LADDER_10_INSTRUCTION_PATCH_OFFSET, LADDER_10_INSTRUCTION_PATCH_SEARCH_CODE),
        (SCORE_INCREMENT_VALUE_OFFSET, SCORE_INCREMENT_VALUE_SEARCH_CODE),
        (SOUND_EFFECT_SEQUENCE_SETUP_OFFSET, SOUND_EFFECT_SEQUENCE_SETUP_SEARCH_CODE),
        (JACK_ATTACK_DATA_02_OFFSET, JACK_ATTACK_DATA_02_SEARCH_CODE),
        (JACK_ATTACK_JOINT_HASH_OFFSET, JACK_ATTACK_JOINT_HASH_SEARCH_CODE),
        (JACK_ATTACK_POSITION_OFFSET, JACK_ATTACK_POSITION_SEARCH_CODE),
        (JACK_ATTACK_SWAP_FRAME_OFFSET, JACK_ATTACK_SWAP_FRAME_SEARCH_CODE),
        (JACK_SE_FALL_FRAME_OFFSET, JACK_SE_FALL_FRAME_SEARCH_CODE),
        (OTHER_MODEL_POSITION_OFFSET, OTHER_MODEL_POSITION_SEARCH_CODE),
    );
}
