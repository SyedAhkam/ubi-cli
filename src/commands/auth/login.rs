use clap::Parser;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use ubi_cli::{APP_ID, CONFIG_DIR_NAME, CREDS_FILE_NAME, GENOME_ID};
use wry::WebViewBuilder;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Parser)]
pub struct Login {}

const REDIRECT_URL: &str = "https://connect.ubisoft.com/ready";

fn ipc_handler(message: String, should_close: Arc<AtomicBool>) {
    if message == "FAILED" {
        panic!("failed to retrieve login data");
    }

    // Construct creds file path
    let config_path = dirs::config_local_dir()
        .expect("failed to find config dir")
        .join(CONFIG_DIR_NAME);

    let creds_file_path = config_path.join(CREDS_FILE_NAME);

    println!("found creds! writing to {:?} ..", creds_file_path);

    // Make sure config path exists
    std::fs::create_dir_all(config_path).unwrap();

    // Save data
    std::fs::write(creds_file_path, message).expect("failed to write creds file");

    println!("closing window..");
    should_close.store(true, Ordering::Relaxed); // weird hack to closing window
}

pub fn handle(_args: Login) {
    // Create a Tao window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ubi-cli - login")
        .build(&event_loop)
        .unwrap();

    let should_close_window = Arc::new(AtomicBool::new(false));

    // If not linux, get normal webview builder
    #[cfg(not(target_os = "linux"))]
    let builder = WebViewBuilder::new(&window);

    // If linux, do some hacks to support wayland
    #[cfg(target_os = "linux")]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    // Init new webview with auth url
    let should_close_window_cloned = should_close_window.clone();
    let _webview = builder
        .with_url(format!(
            "https://connect.ubisoft.com/login?appId={}&genomeId={}&lang=en-US&nextUrl={}",
            APP_ID, GENOME_ID, REDIRECT_URL
        ))
        // Trigger JS script on each page load
        // and try to fetch PRODloginData from local storage after login
        // if found; transmit back using IPC
        .with_initialization_script(
            r#"
                if (window.location.pathname === "/ready") {
                  const loginData = localStorage.getItem("PRODloginData");

                  if (!loginData) window.ipc.postMessage("FAILED");

                  window.ipc.postMessage(loginData);
                }
            "#,
        )
        .with_ipc_handler(move |message| ipc_handler(message, should_close_window_cloned.clone()))
        .build()
        .unwrap();

    // Run the window event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if should_close_window.load(Ordering::Relaxed) {
            *control_flow = ControlFlow::Exit;
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
