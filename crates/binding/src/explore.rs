use napi::bindgen_prelude::*;
use napi_derive::napi;

use sys_file_manager_path::dto::app_info::Platform as SDKPlatform;
use sys_file_manager_path::export::dir::get_os_explore_info;

#[napi(string_enum)]
#[derive(Debug, Default)]
#[allow(non_camel_case_types)]
pub enum Platform {
    #[default]
    unknown,
    windows,
    macos,
}

// 将 SDKPlatform 转换为 Platform
impl From<SDKPlatform> for Platform {
    fn from(platform: SDKPlatform) -> Self {
        match platform {
            SDKPlatform::Unknown => Platform::unknown,
            SDKPlatform::Windows => Platform::windows,
            SDKPlatform::MacOS => Platform::macos,
        }
    }
}

#[napi(object)]
pub struct AppInfo {
    // #[napi(ts_type = "MySpecialString")]
    // pub type_override: String,

    // #[napi(ts_type = "object")]
    // pub type_override_optional: Option<String>,
    /// 句柄
    pub hwnd_id: u32,
    /// 窗口标题
    pub title: String,
    /// MacOS bundleId
    pub bundle_id: String,
    /// 是否激活
    pub is_active: bool,
    /// 目录路径
    pub dir: String,
    /// 执行程序路径
    pub exec: String,

    /// 当前系统
    pub platform: Platform,
}

#[napi(js_name = "getOsExploreInfo")]
pub fn get_os_explore_info_binding() -> Result<AppInfo> {
    let info = unsafe { get_os_explore_info() }.map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("获取系统窗口信息失败: {}", err),
        )
    })?;

    Ok(AppInfo {
        hwnd_id: info.hwnd_id as u32,
        title: info.title,
        bundle_id: info.bundle_id,
        is_active: info.is_active,
        dir: info.dir,
        exec: info.exec,
        platform: Platform::from(info.platform),
    })
}
