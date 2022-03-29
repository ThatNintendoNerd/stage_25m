#![feature(asm)]

mod offsets;
mod structs;

use skyline::{hook, hooks::InlineCtx, patching::patch_data};
use smash::{app::AttackData, phx::Hash40};
use structs::{LadderParam, SequenceParam};
use offsets::{
    LADDER_SETUP_OFFSET,
    LADDER_10_INSTRUCTION_PATCH_OFFSET,
    SCORE_INCREMENT_VALUE_OFFSET,
    SOUND_EFFECT_SEQUENCE_SETUP_OFFSET,
    JACK_ATTACK_DATA_02_OFFSET,
    JACK_ATTACK_JOINT_HASH_OFFSET,
    JACK_ATTACK_POSITION_OFFSET,
    JACK_ATTACK_SWAP_FRAME_OFFSET,
    JACK_SE_FALL_FRAME_OFFSET,
    OTHER_MODEL_POSITION_OFFSET,
};

#[hook(offset = LADDER_SETUP_OFFSET)]
unsafe fn ladder_setup_hook(param_1: &u64, param_2: &u64, subdirectory_name: &i8, ladder_param: &mut LadderParam, param_5: &u64) {
    match std::ffi::CStr::from_ptr(subdirectory_name).to_str().unwrap() {
        "stc_hasigo_01_set" => {
            (*ladder_param).pos.x  = 95.0;
            (*ladder_param).pos.y  = 4.0;
            (*ladder_param).area.y = 24.0;
        },
        "stc_hasigo_02_set" => {
            (*ladder_param).pos.x  = -15.0;
            (*ladder_param).pos.y  = 32.0;
            (*ladder_param).area.y = 29.0;
        },
        "stc_hasigo_03_set" => {
            (*ladder_param).pos.x  = -95.0;
            (*ladder_param).pos.y  = 35.0;
            (*ladder_param).area.y = 23.0;
        },
        "stc_hasigo_04_set" => {
            (*ladder_param).pos.x  = 5.0;
            (*ladder_param).pos.y  = 62.0;
            (*ladder_param).area.y = 29.0;
        },
        "stc_hasigo_05_set" => {
            (*ladder_param).pos.x  = 95.0;
            (*ladder_param).pos.y  = 65.0;
            (*ladder_param).area.y = 23.0;
        },
        "stc_hasigo_06_set" => {
            (*ladder_param).pos.x  = -45.0;
            (*ladder_param).pos.y  = 93.0;
            (*ladder_param).area.y = 27.0;
        },
        "stc_hasigo_07_set" => {
            (*ladder_param).pos.x  = -95.0;
            (*ladder_param).pos.y  = 95.0;
            (*ladder_param).area.y = 23.0;
        },
        "stc_hasigo_08_set" => {
            (*ladder_param).pos.x  = 95.0;
            (*ladder_param).pos.y  = 125.0;
            (*ladder_param).area.y = 23.0;
        },
        "stc_hasigo_09_set" => {
            (*ladder_param).pos.x  = 25.0;
            (*ladder_param).pos.y  = 151.0;
            (*ladder_param).area.y = 25.0;
        },
        _ => {}
    }

    original!()(param_1, param_2, subdirectory_name, ladder_param, param_5);
}

#[hook(offset = SCORE_INCREMENT_VALUE_OFFSET, inline)]
unsafe fn score_increment_value_parasol_hook(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut();

    *w8 -= 800;
    *w8 += 500;
}

#[hook(offset = SCORE_INCREMENT_VALUE_OFFSET + 0x38, inline)]
unsafe fn score_increment_value_bag_hook(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut();

    *w8 -= 800;
    *w8 += 500;
}

#[hook(offset = SOUND_EFFECT_SEQUENCE_SETUP_OFFSET)]
fn sound_effect_sequence_setup_hook(param_1: &u64, sequence_param: &mut SequenceParam) {
    if (*sequence_param).sound_id == 0x01 {
        match (*sequence_param).sound_frame {
            frame if frame == 1.0  => (*sequence_param).sound_frame = 114.0,
            frame if frame == 30.0 => (*sequence_param).sound_frame = 146.0,
            frame if frame == 60.0 => (*sequence_param).sound_frame = 290.0,
            frame if frame == 75.0 => (*sequence_param).sound_frame = 322.0,
            _ => {}
        }

        original!()(param_1, sequence_param);
    }
}

#[hook(offset = JACK_ATTACK_DATA_02_OFFSET, inline)]
unsafe fn jack_attack_data_02_hook(ctx: &mut InlineCtx) {
    let attack_data = *ctx.registers[3].x.as_ref() as *mut AttackData;

    (*attack_data).sound_level = 0x01; // ATTACK_SOUND_LEVEL_M
}

#[hook(offset = JACK_ATTACK_JOINT_HASH_OFFSET + 0x0C, inline)]
unsafe fn jack_attack_joint_hash_hook(ctx: &mut InlineCtx) {
    let joint_hash = ctx.registers[1].x.as_mut();

    *joint_hash = Hash40::new("barrel").as_u64();
}

#[hook(offset = JACK_ATTACK_SWAP_FRAME_OFFSET, inline)]
unsafe fn jack_attack_swap_frame_hook(_ctx: &mut InlineCtx) {
    asm!("fmov s1, w8", in("w8") 44.0_f32);
}

#[hook(offset = JACK_SE_FALL_FRAME_OFFSET, inline)]
unsafe fn jack_se_fall_frame_hook(_ctx: &mut InlineCtx) {
    asm!("fmov s1, w8", in("w8") f32::NAN);
}

#[allow(unused_must_use)]
#[skyline::main(name = "stage_25m")]
fn main() {
    offsets::search_offsets();
    skyline::install_hooks!(
        ladder_setup_hook,
        score_increment_value_parasol_hook,
        score_increment_value_bag_hook,
        sound_effect_sequence_setup_hook,
        jack_attack_data_02_hook,
        jack_attack_joint_hash_hook,
        jack_attack_swap_frame_hook,
        jack_se_fall_frame_hook,
    );
    unsafe {
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x00, &-90.0_f32);         // DONKEYKONG_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x10, &-115.0_f32);        // CURRENTSCORE1_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x20, &-105.0_f32);        // CURRENTSCORE2_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x30, &-95.0_f32);         // CURRENTSCORE3_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x40, &-85.0_f32);         // CURRENTSCORE4_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x50, &-75.0_f32);         // CURRENTSCORE5_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x60, &-65.0_f32);         // CURRENTSCORE6_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x70, &5.0_f32);           // HIGHSCORE1_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x80, &15.0_f32);          // HIGHSCORE2_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0x90, &25.0_f32);          // HIGHSCORE3_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xA0, &35.0_f32);          // HIGHSCORE4_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xB0, &45.0_f32);          // HIGHSCORE5_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xC0, &55.0_f32);          // HIGHSCORE6_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xD0, &78.0_f32);          // PARASOL_SEARCH_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xD4, &42.0_f32);          // PARASOL_SEARCH_POS_Y
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xE0, &-114.25_f32);       // BAG_SEARCH_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xE4, &129.0_f32);         // BAG_SEARCH_POS_Y
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xF0, &f32::NAN);          // HAT_SEARCH_POS_X
        patch_data(OTHER_MODEL_POSITION_OFFSET + 0xF4, &f32::NAN);          // HAT_SEARCH_POS_Y
        patch_data(JACK_ATTACK_POSITION_OFFSET + 0x04, &5.0_f32);           // JACK_ATTACK_OFFSET_Y
        patch_data(LADDER_10_INSTRUCTION_PATCH_OFFSET + 0x04, &0x14000021); // 'B #0x84'
    }
}
