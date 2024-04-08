pub mod v1 {
    pub mod osu;
}

pub const MIN_SLUG_LENGTH: usize = 2;
pub const MAX_SLUG_LENGTH: usize = 8;
pub const MIN_NAME_LENGTH: usize = 4;
pub const MAX_NAME_LENGTH: usize = 64;

pub fn if_false(t: &bool) -> bool {
    !t
}
