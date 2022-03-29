#[repr(C)]
pub struct LadderParam {
    pub pos: smash::cpp::simd::Vector3,
    pub area: smash::cpp::simd::Vector2,
}

#[repr(C)]
pub struct SequenceParam {
    pub sound_id: u32,
    unk1: i32,
    pub sound_frame: f32,
    unk2: i32
}
