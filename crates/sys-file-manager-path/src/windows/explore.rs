// RUST https://stackoverflow.com/questions/73311644/get-path-to-selected-files-in-active-explorer-window

#![cfg(target_os = "windows")]

use windows::core::{IUnknown, Interface, Param};
use windows::Win32::Foundation::{HWND, S_FALSE};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, IDispatch, CLSCTX_LOCAL_SERVER,
    COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};
use windows::Win32::System::Ole::IEnumVARIANT;
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::System::Variant::VT_DISPATCH;
use windows::Win32::UI::Shell::{
    IPersistIDList, IShellBrowser, IShellItem, IShellWindows, IUnknown_QueryService,
    SHCreateItemFromIDList, SID_STopLevelBrowser, ShellWindows, SIGDN_DESKTOPABSOLUTEPARSING,
};

#[derive(Debug)]
pub struct SubExploreInfo {
    pub shell_browser: IShellBrowser,
    pub hwnd: HWND,
}

pub fn get_sub_explore() -> anyhow::Result<Vec<SubExploreInfo>> {
    // CoInitialize 是一个 COM 初始化函数，用于初始化 COM 运行时，可以使用 CoInitialize 及 CoInitializeEx。
    let shell_windows: IShellWindows = unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?
    };

    dump_windows(&shell_windows)
}

fn dump_windows(shell_windows: &IShellWindows) -> anyhow::Result<Vec<SubExploreInfo>> {
    let unknowns = unsafe { shell_windows._NewEnum() }?;
    let enum_variant = unknowns.cast::<IEnumVARIANT>()?;

    let mut infos = vec![];

    loop {
        let mut fetched = 0;
        let mut rgvar: [VARIANT; 1] = [VARIANT::default(); 1];
        let hr = unsafe { enum_variant.Next(&mut rgvar, &mut fetched) };

        // 是否没有更多的子窗口
        if hr == S_FALSE || fetched == 0 {
            break;
        }

        // 不是一个 IDispatch 接口？
        if rgvar[0].vt() != VT_DISPATCH {
            continue;
        }

        // 使用 TryFrom 转换 VARIANT 到 IDispatch
        let dispatch: IDispatch = match (&rgvar[0]).try_into() {
            Ok(d) => d,
            Err(_) => continue,
        };

        infos.push(get_browser_info(&dispatch)?);

        // 将 UTF-16 转换为 UTF-8 以供显示
        // let location = String::from_utf16_lossy(&location);
    }

    Ok(infos)
}

// 获取信息
// 设置参数 hwnd: &mut HWND, 可以通过 &mut Default::default() 获取空窗口句柄指针
fn get_browser_info<P>(unk: P) -> anyhow::Result<SubExploreInfo>
where
    P: Param<IUnknown>,
{
    let shell_browser: IShellBrowser =
        unsafe { IUnknown_QueryService(unk, &SID_STopLevelBrowser) }?;
    let hwnd = unsafe { shell_browser.GetWindow() }?;

    Ok(SubExploreInfo {
        shell_browser,
        hwnd,
    })
}

// 从资源管理器视图获取路径, 通过 let location = String::from_utf16_lossy(&location); 获取路径值
pub fn get_path_from_explore_view(browser: &IShellBrowser) -> windows::core::Result<String> {
    let shell_view = unsafe { browser.QueryActiveShellView() }?;
    let persist_id_list: IPersistIDList = shell_view.cast()?;
    let id_list = unsafe { persist_id_list.GetIDList() }?;

    unsafe {
        let item = SHCreateItemFromIDList::<IShellItem>(id_list)?;
        let ptr = item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING)?;

        // 计算 UTF-16 字符串长度（不包含 NULL 终止符）
        let mut len = 0;
        while *ptr.0.add(len) != 0 {
            len += 1;
        }

        // 使用 slice 转换为 Vec<u16>
        let path = std::slice::from_raw_parts(ptr.0, len).to_vec();

        // 释放内存
        CoTaskMemFree(Some(ptr.0 as _));
        CoTaskMemFree(Some(id_list as _));

        Ok(String::from_utf16_lossy(&path))
    }
}
