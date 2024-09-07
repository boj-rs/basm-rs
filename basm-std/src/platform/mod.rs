#[cfg(not(test))]
pub mod allocator;
#[cfg(all(not(test), feature = "codegen"))]
pub mod codegen;
pub mod io;
#[cfg(not(test))]
pub mod loader;
#[cfg(not(test))]
pub mod malloc;
#[cfg(not(test))]
pub mod os;
#[cfg_attr(test, path = "services_std.rs")]
pub mod services;

#[cfg(not(test))]
pub fn init(platform_data_by_loader: usize) {
    services::install(platform_data_by_loader);

    let pd = services::platform_data();
    unsafe {
        match pd.env_id {
            #[cfg(not(any(target_arch = "wasm32", target_arch = "aarch64")))]
            #[cfg(not(feature = "short"))]
            services::ENV_ID_WINDOWS => {
                /* use OS APIs directly */
                os::windows::init();
            }
            #[cfg(not(any(target_arch = "wasm32")))]
            services::ENV_ID_LINUX => {
                /* use syscalls directly */
                os::linux::init();
            }
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            services::ENV_ID_MACOS => {
                os::macos::init();
            }
            #[cfg(target_arch = "wasm32")]
            services::ENV_ID_WASM => {
                /* wasm32-specific */
                os::wasm32::init();
            }
            _ => {
                /* use loader services for allocation */
                #[cfg(not(feature = "short"))]
                os::unknown::init();
                #[cfg(feature = "short")]
                unreachable!();
            }
        }
    }
}
#[cfg(not(test))]
pub fn try_exit() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let pd = services::platform_data();
        if (pd.env_flags & services::ENV_FLAGS_NO_EXIT) != 0 {
            return;
        }
        if pd.env_id == services::ENV_ID_LINUX {
            unsafe {
                os::linux::syscall::exit_group(services::get_exit_status() as usize);
            }
        }
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if pd.env_id == services::ENV_ID_MACOS {
            unsafe {
                os::macos::syscall::exit_group(services::get_exit_status() as usize);
            }
        }
    }
}
#[cfg(not(test))]
pub fn is_local_env() -> bool {
    let pd = services::platform_data();
    (pd.env_flags & services::ENV_FLAGS_NATIVE) != 0
}

#[cfg(test)]
pub fn init(_platform_data_by_loader: usize) {}
#[cfg(test)]
pub fn try_exit() {}
#[cfg(test)]
pub fn is_local_env() -> bool {
    true
}
