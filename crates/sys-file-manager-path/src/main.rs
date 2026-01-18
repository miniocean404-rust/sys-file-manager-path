#[cfg(target_os = "macos")]
use sys_file_manager_path::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use sys_file_manager_path::windows::index::get_explore_info;

fn main() -> anyhow::Result<()> {
    unsafe {
        #[cfg(target_os = "windows")]
        let info = get_explore_info()?;

        #[cfg(target_os = "macos")]
        let info = get_finder_info()?;

        println!("{:?}", info);
    }

    anyhow::Ok(())
}
