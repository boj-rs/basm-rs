pub mod loader;
pub mod malloc;
pub mod os;
pub mod allocator;
pub mod services;

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