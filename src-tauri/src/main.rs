// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use tauri::{Manager, async_runtime::handle};

// use window_vibrancy::{Color, apply_tabbed};
// #[cfg(target_os = "windows")]
// // use raw_window_handle::HasRawWindowHandle::raw_window_handle;
// use window_vibrancy::{apply_mica,apply_blur, apply_acrylic,apply_vibrancy, NSVisualEffectMaterial};
// use windows_sys::Win32::Graphics::Dwm::{DwmSetWindowAttribute,DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND};
// use winapi::{
//     shared::{
//       minwindef::{BOOL, TRUE},
//       winerror::{HRESULT, S_OK}, windef::{HWND__, HWND},
//     },
//     um::dwmapi::{DWMWA_TRANSITIONS_FORCEDISABLED},
//   };
  
  
// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// fn main() {
//     tauri::Builder::default().setup(|app| {
//         let main_window = app.get_window("main").unwrap();
//         #[cfg(target_os = "windows")]
//         {   
//             // let rawhwnd= main_window.hwnd() as *const _ as HWND;
//             // apply_mica(&main_window, Some((false)));
//             // apply_blur(&main_window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
//             // apply_acrylic(&main_window, Some((0, 0, 0, 125))).expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");
//             // apply_tabbed(main_window.clone(),Some((false)));

// //   unsafe {
// //     let hwnd = main_window.hwnd().unwrap().0 as isize;
// //     let value: BOOL = TRUE;
    
// //     // let result: HRESULT = DwmSetWindowAttribute(
// //     //   hwnd,
// //     //   DWMWA_TRANSITIONS_FORCEDISABLED,
// //     //   &value as *const BOOL as *const _,
// //     //   std::mem::size_of::<BOOL>() as _,
// //     // );
// //     let result2:HRESULT = DwmSetWindowAttribute(
// //         hwnd,
// //         DWMWA_WINDOW_CORNER_PREFERENCE,
// //         &DWMWCP_ROUND as *const _ as _,
// //         4,
// //     );
// //     if result2 != S_OK {
// //       // handle the error
// //     }
// //   }
//             print!("Windows");
//             // let ins=main_window.inner_size().unwrap();
//             // main_window.minimize().unwrap();
//             // main_window.unminimize().unwrap();
            
//             // main_window.maximize().unwrap();
//             // main_window.unmaximize().unwrap();
//         }

//         Ok(())
//     })
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
#![warn(clippy::nursery, clippy::pedantic)]

mod util;
mod ns_panel;

use tauri::{
    Manager,
};
//     CustomMenuItem, Manager,SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
// };

#[allow(unused_imports)]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use util::{
    convert_all_app_icons_to_png, create_preferences_if_missing, get_icon, handle_input,
    launch_on_login, open_command,
};

// fn create_system_tray() -> SystemTray {
//     let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
//     let show = CustomMenuItem::new("Show".to_string(), "Show");
//     let hide = CustomMenuItem::new("Hide".to_string(), "Hide");
//     let preferences = CustomMenuItem::new("Preferences".to_string(), "Preferences");
//     let tray_menu = SystemTrayMenu::new()
//         .add_item(show)
//         .add_item(hide)
//         .add_item(preferences)
//         .add_native_item(SystemTrayMenuItem::Separator)
//         .add_item(quit);
//     SystemTray::new().with_menu(tray_menu)
// }

fn main() {
    convert_all_app_icons_to_png();
    create_preferences_if_missing();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_command,
            get_icon,
            handle_input,
            launch_on_login,
            ns_panel::init_ns_panel,
            // ns_panel::show_app,
            // ns_panel::hide_app
        ])
        .setup(|app| {
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            // window.hide().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
