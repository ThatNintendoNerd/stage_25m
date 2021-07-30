mod offsets;

use skyline::{hook, hooks::InlineCtx, install_hooks, patching::patch_data};
use smash::{hash40, lib::lua_const::ATTACK_SOUND_LEVEL_M};
use offsets::*;

#[hook(offset=LADDER_PARAMETER_INLINE_HOOK_OFFSET+0x1a0, inline)]
unsafe fn modify_ladder_04_height(ctx: &mut InlineCtx) {
    let sp = (ctx as *mut _ as *mut u8).offset(0x100) as *mut f32;
    *sp.offset(5) = 29.0;
}

#[hook(offset=LADDER_PARAMETER_INLINE_HOOK_OFFSET+0x338, inline)]
unsafe fn modify_ladder_07_height(ctx: &mut InlineCtx) {
    let sp = (ctx as *mut _ as *mut u8).offset(0x100) as *mut f32;
    *sp.offset(5) = 23.0;
}

#[hook(offset=LADDER_PARAMETER_INLINE_HOOK_OFFSET+0x3c0, inline)]
unsafe fn modify_ladder_08_height(ctx: &mut InlineCtx) {
    let sp = (ctx as *mut _ as *mut u8).offset(0x100) as *mut f32;
    *sp.offset(5) = 23.0;
}

#[hook(offset=LADDER_PARAMETER_INLINE_HOOK_OFFSET+0x448, inline)]
unsafe fn modify_ladder_09_height(ctx: &mut InlineCtx) {
    let sp = (ctx as *mut _ as *mut u8).offset(0x100) as *mut f32;
    *sp.offset(5) = 25.0;
}

#[hook(offset=SCORE_INCREMENT_VALUE_INLINE_HOOK_OFFSET, inline)]
unsafe fn modify_score_increment_value_parasol(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut();
    *w8 -= 800; // Subtract the immediate so we can add our own value. Equaling freezes the counter at the equaled value once the score changes.
    *w8 += 500; // New value to increment the score counter with.
}

#[hook(offset=SCORE_INCREMENT_VALUE_INLINE_HOOK_OFFSET+0x38, inline)]
unsafe fn modify_score_increment_value_bag(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut();
    *w8 -= 800;
    *w8 += 500;
}

#[hook(offset=DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET, inline)]
unsafe fn modify_donkeykong_se_pound_1_frame(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut f32;
    *x8.offset(1) = 114.0;
}

#[hook(offset=DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET+0x18, inline)]
unsafe fn modify_donkeykong_se_pound_2_frame(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut f32;
    *x8.offset(1) = 146.0;
}

#[hook(offset=DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET+0x34, inline)]
unsafe fn modify_donkeykong_se_pound_3_frame(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut f32;
    *x8.offset(1) = 290.0;
}

#[hook(offset=DONKEYKONG_SE_POUND_INLINE_HOOK_OFFSET+0x50, inline)]
unsafe fn modify_donkeykong_se_pound_4_frame(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut f32;
    *x8.offset(1) = 322.0;
}

#[hook(offset=JACK_SE_HASH40_INLINE_HOOK_OFFSET+0xc, inline)]
unsafe fn modify_jack_se_jump_hash(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut();
    *x8 = 0x0; // Zero the contents of the hash40 so nothing plays in-game.
}

#[hook(offset=JACK_SE_HASH40_INLINE_HOOK_OFFSET+0x54, inline)]
unsafe fn modify_jack_se_fall_hash(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut();
    *x8 = 0x0;
}

#[hook(offset=JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_OFFSET, inline)]
unsafe fn modify_jack_sound_level_1(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut u8;
    *x8 = *ATTACK_SOUND_LEVEL_M as u8;
}

#[hook(offset=JACK_HITBOX_SOUND_LEVEL_INLINE_HOOK_OFFSET+0x2f4, inline)]
unsafe fn modify_jack_sound_level_2(ctx: &mut InlineCtx) {
    let x8 = ctx.registers[8].x.as_mut() as *mut _ as *mut u8;
    *x8 = *ATTACK_SOUND_LEVEL_M as u8;
}

#[hook(offset=JACK_HITBOX_JOINT_INLINE_HOOK_OFFSET+0xc, inline)]
unsafe fn modify_jack_joint_hash(ctx: &mut InlineCtx) {
    let x1 = ctx.registers[1].x.as_mut();
    *x1 = hash40("attack"); // Just so I don't have to keep the joint name "Jack_O_OBJ_O_HIR" when animating the barrel.
}

#[allow(unused_must_use)]
#[skyline::main(name = "stage_25m")]
fn main() {
    search_offsets();
    install_hooks!(
        modify_ladder_04_height, 
        modify_ladder_07_height, 
        modify_ladder_08_height, 
        modify_ladder_09_height, 
        modify_score_increment_value_parasol, 
        modify_score_increment_value_bag, 
        modify_donkeykong_se_pound_1_frame, 
        modify_donkeykong_se_pound_2_frame, 
        modify_donkeykong_se_pound_3_frame, 
        modify_donkeykong_se_pound_4_frame, 
        modify_jack_se_jump_hash, 
        modify_jack_se_fall_hash, 
        modify_jack_sound_level_1, 
        modify_jack_sound_level_2, 
        modify_jack_joint_hash
    );
    unsafe {
        patch_data(LADDER_01_POSITION_OFFSET, &95.0_f32);
        patch_data(LADDER_01_POSITION_OFFSET+0x4, &4.0_f32);
        patch_data(LADDER_01_08_PARAMETER_OFFSET+0x4, &24.0_f32);
        patch_data(LADDER_02_POSITION_OFFSET, &-15.0_f32);
        patch_data(LADDER_02_POSITION_OFFSET+0x4, &32.0_f32);
        patch_data(LADDER_02_PARAMETER_OFFSET+0x4, &29.0_f32);
        patch_data(LADDER_03_POSITION_OFFSET, &-95.0_f32);
        patch_data(LADDER_03_POSITION_OFFSET+0x4, &35.0_f32);
        patch_data(LADDER_03_04_PARAMETER_OFFSET+0x4, &23.0_f32);
        patch_data(LADDER_04_POSITION_OFFSET, &5.0_f32);
        patch_data(LADDER_04_POSITION_OFFSET+0x4, &62.0_f32);
        /* 4th ladder parameter offset is shared, so modify the stack pointer. */
        patch_data(LADDER_05_POSITION_OFFSET, &95.0_f32);
        patch_data(LADDER_05_POSITION_OFFSET+0x4, &65.0_f32);
        patch_data(LADDER_05_07_09_PARAMETER_OFFSET+0x4, &23.0_f32);
        patch_data(LADDER_06_POSITION_OFFSET, &-45.0_f32);
        patch_data(LADDER_06_POSITION_OFFSET+0x4, &93.0_f32);
        patch_data(LADDER_06_PARAMETER_OFFSET, &27.0_f32);
        patch_data(LADDER_07_POSITION_OFFSET, &-95.0_f32);
        patch_data(LADDER_07_POSITION_OFFSET+0x4, &95.0_f32);
        /* 7th ladder parameter offset is shared, so modify the stack pointer. */
        patch_data(LADDER_08_POSITION_OFFSET, &95.0_f32);
        patch_data(LADDER_08_POSITION_OFFSET+0x4, &125.0_f32);
        /* 8th ladder parameter offset is shared, so modify the stack pointer. */
        patch_data(LADDER_09_POSITION_OFFSET, &25.0_f32);
        patch_data(LADDER_09_POSITION_OFFSET+0x4, &151.0_f32);
        /* 9th ladder parameter offset is shared, so modify the stack pointer. */
        patch_data(LADDER_10_POSITION_OFFSET, &f32::INFINITY);      // A 10th ladder is undesired, so set coordinates to infinity.
        patch_data(LADDER_10_POSITION_OFFSET+0x4, &f32::INFINITY);
        /* No need for a 10th ladder parameter modification if positions are invalidated. */
        patch_data(OTHER_MODEL_POSITION_OFFSET, &-90.0_f32);        // DONKEYKONG_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x10, &-115.0_f32);  // CURRENTSCORE1_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x20, &-105.0_f32);  // CURRENTSCORE2_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x30, &-95.0_f32);   // CURRENTSCORE3_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x40, &-85.0_f32);   // CURRENTSCORE4_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x50, &-75.0_f32);   // CURRENTSCORE5_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x60, &-65.0_f32);   // CURRENTSCORE6_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x70, &5.0_f32);     // HIGHSCORE1_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x80, &15.0_f32);    // HIGHSCORE2_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0x90, &25.0_f32);    // HIGHSCORE3_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xa0, &35.0_f32);    // HIGHSCORE4_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xb0, &45.0_f32);    // HIGHSCORE5_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xc0, &55.0_f32);    // HIGHSCORE6_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xd0, &78.0_f32);    // PARASOL_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xd4, &42.0_f32);    // PARASOL_Y_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xe0, &-114.25_f32); // BAG_X_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xe4, &129.0_f32);   // BAG_Y_POSITION
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xf0, &f32::NAN);    // HAT_X_POSITION (A third item is undesired, so set coordinates to NaN.)
        patch_data(OTHER_MODEL_POSITION_OFFSET+0xf4, &f32::NAN);    // HAT_Y_POSITION
        patch_data(JACK_HITBOX_RELATIVE_Y_POSITION_OFFSET, &5.0_f32);
    }
}