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

#[cfg(not(test))]
pub fn init(service_functions_by_loader: usize) {
    services::install(service_functions_by_loader);

    let pd = services::platform_data();
    unsafe {
        match (*pd).env_id {
            services::ENV_ID_WINDOWS => {
                /* use OS APIs directly */
                os::windows::init();
            },
            services::ENV_ID_LINUX => {
                /* use syscalls directly */
                os::linux::init();
            },
            _ => {
                /* use loader services for allocation */
                os::unknown::init();
            }
        }
    }
}

#[cfg(test)]
pub fn init(_service_functions_by_loader: usize) {
}