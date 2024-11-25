use std::env;

#[cfg(target_os = "macos")]
mod clipboard {
    use cocoa::appkit::NSStringPboardType;
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;
    use objc::runtime::Class;
    use objc::sel;
    use objc::{msg_send, sel_impl};

    pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
        unsafe {
            let pasteboard = Class::get("NSPasteboard").ok_or("NSPasteboard class not found")?;
            let general_pasteboard: id = msg_send![pasteboard, generalPasteboard];
            let _: () = msg_send![general_pasteboard, clearContents];

            let ns_string = NSString::alloc(nil).init_str(text);
            let _: bool =
                msg_send![general_pasteboard, setString:ns_string forType:NSStringPboardType];

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let path_str = current_dir.to_str().ok_or("Invalid path")?;

    clipboard::copy_to_clipboard(path_str)
        .map_err(|e| -> Box<dyn std::error::Error> { Box::<dyn std::error::Error>::from(e) })?;
    println!("Copied: {}", path_str);
    Ok(())
}

#[cfg(target_os = "macos")]
extern crate cocoa;
extern crate objc;
