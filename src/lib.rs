mod rpc;
mod types;

use std::sync::{Arc, Mutex};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::webview::PageLoadEvent;
use tauri::{Builder, Listener, Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }));
    }

    builder.manage(Arc::new(Mutex::new(rpc::Client::new())))
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_webview_window("main").unwrap().open_devtools();
            }

            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Deezer")
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                      button: MouseButton::Left,
                      button_state: MouseButtonState::Up,
                      ..
                    } => {
                        println!("left click pressed and released");
                        // in this example, let's show and focus the main window when the tray is clicked
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                      println!("unhandled event {event:?}");
                    }
                  })
                .menu(&menu).show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                      println!("Quitting the app via menu item");
                        app.state::<Arc<Mutex<rpc::Client>>>()
                            .clone().lock().unwrap().close().unwrap();
                      app.exit(0);
                    }
                    "show" => {
                        println!("Showing the app via menu item");
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                      println!("Menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            app_handle.clone().listen("update_activity", move |event| {
                let song = serde_json::from_str(event.payload()).unwrap_or_default();
                println!("{:?}", song);
                app_handle
                    .state::<Arc<Mutex<rpc::Client>>>()
                    .clone()
                    .lock()
                    .unwrap()
                    .update_activity(song)
                    .unwrap();
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_page_load(|webview, payload| match payload.event() {
            PageLoadEvent::Finished => {
                println!("Page loaded!");
                webview
                    .eval(
                    "const interval = setInterval(() => {
                        if (document.querySelector('[data-testid=\"item_title\"]')) {
                            clearInterval(interval);
                            document.querySelector('[data-testid=\"miniplayer_container\"] .slider').addEventListener('click', () => window.__TAURI__.event.emit('update_activity',
                                Object.assign(dzPlayer.getCurrentSong(), {
                                    IS_PLAYING: dzPlayer.isPlaying(),
                                    SNG_TIME: Math.round(dzPlayer.getDuration()),
                                    REM_TIME: Math.round(dzPlayer.getRemainingTime())
                                }))
                            );
                            const playerObserver = new MutationObserver(() => window.__TAURI__.event.emit('update_activity',
                                Object.assign(dzPlayer.getCurrentSong(), {
                                    IS_PLAYING: dzPlayer.isPlaying(),
                                    SNG_TIME: Math.round(dzPlayer.getDuration()),
                                    REM_TIME: Math.round(dzPlayer.getRemainingTime())
                                }))
                            );
                            playerObserver.observe(document.querySelector('.marquee-content > [data-testid=\"item_title\"]'), { childList: true, subtree: true, characterData: true });
                            playerObserver.observe(document.querySelector('.chakra-button__group > button[data-testid^=\"play_button_\"]'), { attributes: true, childList: false, subtree: false });
                        }
                    }, 50);
                    const originalFetch = window.fetch;
                    window.fetch = (url, options) => {
                        if (url.startsWith('https://www.deezer.com/ajax/gw-light.php?method=deezer.adConfig')) {
                            return Promise.reject(new Error('Request Blocked'));
                        }
                        return originalFetch(url, options);
                    }"
                    )
                    .unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .unwrap();
}
