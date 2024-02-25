use clap::Parser;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Parser)]
pub struct Login {}

// These variables are taken straight from the official ubisoft site
const APP_ID: &str = "314d4fef-e568-454a-ae06-43e3bece12a6";
const GENOME_ID: &str = "85c31714-0941-4876-a18d-2c7e9dce8d40";

const REDIRECT_URL: &str = "https://connect.ubisoft.com/ready";

fn ipc_handler(message: String) {
    if message == "FAILED" {
        panic!("failed to retrieve token");
    }

    println!("{}", message);
}

pub fn handle(_args: Login) {
    // Create a Tao window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ubi-cli - login")
        .build(&event_loop)
        .unwrap();

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
        .with_ipc_handler(ipc_handler)
        .build()
        .unwrap();

    // Run the window event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
