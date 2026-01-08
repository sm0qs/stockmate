mod system;

#[cfg(target_os = "linux")]
mod linux_de;

pub use system::get_system_accent_color;
