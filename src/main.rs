use gdk4::ContentProvider;
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Button, Image, Label, Orientation, Box, Align, ScrolledWindow
};
use gdk_pixbuf::Pixbuf;
use std::path::PathBuf;
use chrono::Local;
use std::fs;
use tokio::runtime::Runtime;
use gdk4::prelude::{DisplayExt, MonitorExt};
use ashpd::desktop::screenshot::Screenshot;


fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.pigeonshot")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pigeon Shot")
        .default_width(500)
        .default_height(350)
        .build();

    let main_box = Box::new(Orientation::Vertical, 0);

    let content_box = Box::new(Orientation::Vertical, 16);
    content_box.set_margin_top(32);
    content_box.set_margin_bottom(32);
    content_box.set_margin_start(32);
    content_box.set_margin_end(32);
    content_box.set_halign(Align::Center);
    content_box.set_valign(Align::Center);

    let title_label = Label::new(Some("Pigeon Shot"));
    title_label.add_css_class("title-1");

    let info_label = Label::new(Some("A simple screenshot tool\nPress Print key or click below"));
    info_label.set_wrap(true);
    info_label.set_justify(gtk4::Justification::Center);
    info_label.add_css_class("dim-label");

    let screenshot_btn = Button::with_label("📷 Take Screenshot");
    screenshot_btn.add_css_class("suggested-action");
    screenshot_btn.set_size_request(200, 50);

    let weak_window = window.downgrade();
    screenshot_btn.connect_clicked(move |_| {
        if let Some(window) = weak_window.upgrade() {
            glib::MainContext::default().spawn_local(async move {
                let result = take_screenshot().await;
                match result {
                    Ok(pixbuf) => show_preview_window(&window, &pixbuf),
                    Err(e) => eprintln!("Screenshot failed: {}", e),
                }
            });
        }
    });

    content_box.append(&title_label);
    content_box.append(&info_label);
    content_box.append(&screenshot_btn);

    main_box.append(&content_box);
    window.set_child(Some(&main_box));
    window.present();

    setup_hotkey();
}

async fn take_screenshot() -> Result<Pixbuf, std::boxed::Box<dyn std::error::Error>> {
    if !gtk4::is_initialized() {
        gtk4::init()?;
    }

    // Get current monitor where cursor is using GDK
    let display = gdk4::Display::default().ok_or("No display")?;
    let _seat = display.default_seat().ok_or("No seat")?;
    let monitors = display.monitors();
    let monitor = monitors.item(0).and_downcast::<gdk4::Monitor>().ok_or("No monitor")?;
    let connector = monitor.connector().ok_or("No connector")?;

    // Use libwayshot to capture the specific output
    let wayshot = libwayshot::WayshotConnection::new()?;
    let outputs = wayshot.get_all_outputs();
    let target_output = outputs.iter().find(|o| o.name == connector).ok_or("Output not found")?;

    match wayshot.screenshot(target_output.into(), false) {
        Ok(image_buffer) => {
            let rgba = image_buffer.to_rgba8();
            let width = rgba.width() as i32;
            let height = rgba.height() as i32;
            let pixels = rgba.into_raw();
            let bytes = glib::Bytes::from(&pixels);
            let pixbuf = Pixbuf::from_bytes(
                &bytes,
                gdk_pixbuf::Colorspace::Rgb,
                true, // has_alpha
                8,    // bits_per_sample
                width,
                height,
                width * 4, // rowstride for RGBA
            );
            Ok(pixbuf)
        }
        Err(err) => {
            eprintln!("libwayshot capture failed: {}. Trying portal fallback.", err);
            screenshot_via_portal().await
        }
    }
}

async fn screenshot_via_portal() -> Result<Pixbuf, std::boxed::Box<dyn std::error::Error>> {
    // Enter a Tokio runtime so async ashpd/zbus calls can resolve correctly
    let runtime = Runtime::new()?;
    let _guard = runtime.enter();

    // Use XDG Desktop Portal via ashpd for screenshot capture
    let response = Screenshot::request()
        .interactive(false)
        .modal(false)
        .send()
        .await?
        .response()?;
    
    // Get the file URI from the response
    let uri = response.uri().to_string();
    
    // Parse file:// URI to get filesystem path
    let path = if let Some(stripped) = uri.strip_prefix("file://") {
        PathBuf::from(stripped)
    } else {
        PathBuf::from(uri)
    };
    
    // Load the screenshot as a Pixbuf
    let pixbuf = Pixbuf::from_file(path)?;
    Ok(pixbuf)
}

fn show_preview_window(parent: &ApplicationWindow, pixbuf: &Pixbuf) {
    let preview_win = ApplicationWindow::builder()
        .transient_for(parent)
        .title("Screenshot Preview")
        .default_width(900)
        .default_height(700)
        .resizable(true)
        .build();

    let main_box = Box::new(Orientation::Vertical, 12);

    let scroll = ScrolledWindow::new();
    scroll.set_hexpand(true);
    scroll.set_vexpand(true);

    let image = Image::from_pixbuf(Some(pixbuf));
    scroll.set_child(Some(&image));

    let button_box = Box::new(Orientation::Horizontal, 12);
    button_box.set_margin_top(12);
    button_box.set_margin_bottom(12);
    button_box.set_margin_start(12);
    button_box.set_margin_end(12);
    button_box.set_halign(Align::Center);

    let save_btn = Button::with_label("💾 Save to Pictures");
    let copy_btn = Button::with_label("📋 Copy to Clipboard");
    let close_btn = Button::with_label("Close");

    let pixbuf_clone = pixbuf.clone();
    save_btn.connect_clicked(move |_| {
        save_screenshot(&pixbuf_clone);
    });

    let pixbuf_copy = pixbuf.clone();
    copy_btn.connect_clicked(move |_| {
        copy_to_clipboard(&pixbuf_copy);
    });

    close_btn.connect_clicked(glib::clone!(
        #[weak]
        preview_win,
        move |_| {
            preview_win.close();
        }
    ));

    button_box.append(&save_btn);
    button_box.append(&copy_btn);
    button_box.append(&close_btn);

    main_box.append(&scroll);
    main_box.append(&button_box);

    preview_win.set_child(Some(&main_box));
    preview_win.present();
}

fn save_screenshot(pixbuf: &Pixbuf) {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("screenshot_{}.png", timestamp);

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = PathBuf::from(home).join("Pictures/Screenshots").join(&filename);

    fs::create_dir_all(path.parent().unwrap()).ok();

    match pixbuf.savev(&path, "png", &[]) {
        Ok(_) => println!("✓ Saved to: {}", path.display()),
        Err(e) => eprintln!("✗ Save failed: {}", e),
    }
}

fn copy_to_clipboard(pixbuf: &Pixbuf) {
    // Get display and clipboard
    let Some(display) = gdk4::Display::default() else {
        eprintln!("✗ No display available");
        return;
    };

    let clipboard = display.clipboard();

    // Create a temporary file to hold the PNG data
    let temp_path = std::env::temp_dir().join("pigeon_shot_clipboard.png");
    
    match pixbuf.savev(&temp_path, "png", &[]) {
        Ok(_) => {
            // Read the PNG data
            match std::fs::read(&temp_path) {
                Ok(bytes) => {
                    // Create a content provider for the PNG data
                    let gb = glib::Bytes::from(&bytes);
                    let provider: ContentProvider = gdk4::ContentProvider::for_bytes("image/png", &gb);

                    if let Err(err) = clipboard.set_content(Some(&provider)) {
                        eprintln!("✗ Failed to create Screenshot: {:#?}", err);
                    } else {
                        println!("✓ Screenshot copied to clipboard");
                    }
                    // Clean up
                    let _ = std::fs::remove_file(&temp_path);
                }
                Err(e) => eprintln!("✗ Failed to read PNG data: {}", e),
            }
        }
        Err(e) => eprintln!("✗ Failed to save PNG: {}", e),
    }
}

fn setup_hotkey() {
    std::thread::spawn(|| {
        println!("⚙ Setting up Print key hotkey...");
        // Configure with xbindkeys or similar if needed
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_take_screenshot() {
        let result = take_screenshot().await;
        match result {
            Ok(pixbuf) => {
                assert!(pixbuf.width() > 0);
                assert!(pixbuf.height() > 0);
                println!("Screenshot test passed: {}x{}", pixbuf.width(), pixbuf.height());
            }
            Err(e) => {
                assert!(false, "Screenshot test failed (expected in headless): {:#?}", e);
            }
        }
    }
}
