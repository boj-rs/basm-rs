#[cfg(not(test))]
pub mod loader;
#[cfg(not(test))]
pub mod malloc;
#[cfg(not(test))]
pub mod os;
#[cfg(not(test))]
pub mod allocator;
#[cfg_attr(test, path = "services_std.rs")]
pub mod services;
pub mod io;

#[cfg(not(test))]
pub fn init(platform_data_by_loader: usize) {
    services::install(platform_data_by_loader);

    let pd = services::platform_data();
    unsafe {
        match pd.env_id {
            #[cfg(not(target_arch = "wasm32"))]
            #[cfg(not(feature = "short"))]
            services::ENV_ID_WINDOWS => {
                /* use OS APIs directly */
                os::windows::init();
            },
            #[cfg(not(target_arch = "wasm32"))]
            services::ENV_ID_LINUX => {
                /* use syscalls directly */
                os::linux::init();
            },
            #[cfg(target_arch = "wasm32")]
            services::ENV_ID_WASM => {
                /* wasm32-specific */
                os::wasm32::init();
            },
            _ => {
                /* use loader services for allocation */
                os::unknown::init();
            }
        }
    }
}
#[cfg(not(test))]
pub fn try_exit() {
    let pd = services::platform_data();
    if pd.env_id == services::ENV_ID_LINUX {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { os::linux::syscall::exit_group(services::get_exit_status() as usize); }
    }
}

#[cfg(test)]
pub fn init(_platform_data_by_loader: usize) {
}
#[cfg(test)]
pub fn try_exit() {
}