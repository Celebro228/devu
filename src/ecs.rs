pub use linkme::distributed_slice as system;


#[system]
pub static START: [fn()];
#[system]
pub static UPDATE: [fn()];