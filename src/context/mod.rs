pub mod jamerr;
pub mod res_wrapper;

mod res_code {
    pub const SUCCESS: u8 = 0;
    pub const PARAM_ERROR: u8 = 1;
    pub const AUTH_ERROR: u8 = 2;
    pub const BIZ_ERROR: u8 = 3;
    pub const OTHER: u8 = 4;
}
