//! Das Fenster (Port von `main.go` `windowOptions`) und der Tauri-Command `open_url`. Größe und Position merkt das
//! Plugin `window-state`, ein zweiter Start holt das Fenster nach vorn.

use std::path::PathBuf;
use std::sync::Arc;

use serde_json::{Value, json};
use tauri::webview::{Color, PageLoadEvent};
use tauri::{AppHandle, Emitter, Manager, RunEvent, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_window_state::StateFlags;

use super::{App, events::Emit, open_url as address, playtest_media};
use crate::platform;

/// Name des Fensters; `capabilities/default.json` gilt für ihn.
const WINDOW: &str = "main";
const TITLE: &str = "K3C Workbench";
/// Maße wie `assetkit/windowstate`.
const SIZE: (f64, f64) = (1280.0, 800.0);
const MIN_SIZE: (f64, f64) = (900.0, 600.0);
/// `--night-1`.
const BACKGROUND: Color = Color(0x0b, 0x10, 0x26, 0xff);
/// Fenster-Icon für Läufe ohne passende Icon-Ressource der EXE.
pub(super) const ICON_PNG: &[u8] = include_bytes!("../../icons/appicon.png");

/// Verwalteter Zustand im Tauri-Handle.
pub struct AppState(pub Arc<App>);

/// Command `open_url {url}`: nur geprüfte Web-Adressen.
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    platform::shell_open(address::allowed_url(&url)?)
}

/// Registriert die Commands und das Protokoll `k3c` der Player- und Marker-Dateien; generisch, damit der Test die
/// Mock-Laufzeit nutzen kann.
pub fn builder<R: Runtime>(b: tauri::Builder<R>) -> tauri::Builder<R> {
    b.invoke_handler(tauri::generate_handler![crate::rpc::rpc, open_url])
        .register_asynchronous_uri_scheme_protocol(playtest_media::PROTOCOL, |ctx, request, responder| {
            let app = ctx.app_handle().try_state::<AppState>().map(|s| Arc::clone(&s.0));
            tauri::async_runtime::spawn_blocking(move || match app {
                Some(app) => responder.respond(app.media_response(&request)),
                None => {
                    let mut response = tauri::http::Response::new(Vec::new());
                    *response.status_mut() = tauri::http::StatusCode::NOT_FOUND;
                    responder.respond(response);
                }
            });
        })
}

/// App, deren Ereignisse zusätzlich an alle Fenster gehen.
pub fn app_for<R: Runtime>(handle: &AppHandle<R>, root: PathBuf, port: u16) -> App {
    let handle = handle.clone();
    let forward: Emit = Arc::new(move |event: &str, data: Value| {
        // Die Senke kennt das eigene Log nicht; im Dev-Build steht der Fehler in der Konsole.
        if let Err(e) = handle.emit(event, data) {
            eprintln!("❌ ereignis {event}: {e}");
        }
    });
    App::new(root, port, Some(forward))
}

/// Holt das offene Fenster nach vorn (Go `secondInstance`).
fn raise<R: Runtime>(handle: &AppHandle<R>) {
    let Some(window) = handle.get_webview_window(WINDOW) else {
        return;
    };
    let _ = window.unminimize();
    let _ = window.show();
    // Windows holt ein Fenster sonst nicht aus dem Hintergrund.
    let _ = window.set_always_on_top(true);
    let _ = window.set_always_on_top(false);
    let _ = window.set_focus();
}

/// WebView2-Datenordner `%LOCALAPPDATA%\k3c-workbench`.
fn data_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .filter(|v| !v.is_empty())
        .map(|base| PathBuf::from(base).join(super::APP_NAME))
}

/// Baut das Fenster verdeckt (das Plugin setzt die gemerkte Position) und zeigt es, sobald die Seite geladen ist.
fn open_window<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<WebviewWindow<R>> {
    let mut window = WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::default())
        .title(TITLE)
        .inner_size(SIZE.0, SIZE.1)
        .min_inner_size(MIN_SIZE.0, MIN_SIZE.1)
        .background_color(BACKGROUND)
        .visible(false)
        .on_page_load(|window, payload| {
            if payload.event() == PageLoadEvent::Finished {
                let _ = window.show();
                // Blockiert bis zum sichtbaren Fenster, darum in einem eigenen Thread.
                std::thread::spawn(|| k3c_kit::winicon::set(ICON_PNG));
            }
        });
    if let Some(dir) = data_dir() {
        window = window.data_directory(dir);
    }
    window.build()
}

fn setup(app: &mut tauri::App, root: PathBuf, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let workbench = Arc::new(app_for(app.handle(), root, port));
    app.manage(AppState(Arc::clone(&workbench)));
    // Vor dem Fenster: Aufrufe der Oberfläche warten ohnehin, aber so steht der MCP-Server auch ohne WebView.
    tauri::async_runtime::block_on(workbench.startup());
    open_window(app)?;
    workbench.logger().info("🚀 fenster erzeugt", &[("ns", json!("main"))]);
    Ok(())
}

/// Läuft, bis das Fenster geschlossen wird; Schließen beendet Programm und MCP-Server.
pub fn run(root: PathBuf, port: u16) -> Result<(), String> {
    let flags = StateFlags::SIZE | StateFlags::POSITION;
    let app = builder(tauri::Builder::default())
        // Zuerst, damit ein zweiter Start nichts anderes anlegt.
        .plugin(tauri_plugin_single_instance::init(|handle, _args, _cwd| raise(handle)))
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(flags)
                .build(),
        )
        .setup(move |app| setup(app, root, port))
        .build(tauri::generate_context!())
        .map_err(|e| e.to_string())?;
    app.run(|handle, event| {
        if let RunEvent::Exit = event
            && let Some(state) = handle.try_state::<AppState>()
        {
            tauri::async_runtime::block_on(state.0.shutdown());
        }
    });
    Ok(())
}
