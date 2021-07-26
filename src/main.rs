use bindings::{
    Windows::Win32::UI::WindowsAndMessaging::*,
    Windows::Win32::UI::Controls::*,
    Windows::Win32::Foundation::*,
};

use std::convert::TryFrom;
use std::env;
use std::panic::panic_any;


fn main() -> windows::Result<()> {
    // let uri = Uri::CreateUri("https://kennykerr.ca/feed")?;
    // let client = SyndicationClient::new()?;
    // let feed = client.RetrieveFeedAsync(uri)?.get()?;
    //
    // for item in feed.Items()? {
    //     print!("{}", item.Title()?.Text()?);
    // }

    let args: Vec<String> = env::args().collect();
    let path = &args[1];


    unsafe {
        run(path)
    }

    Ok(())
}


unsafe fn run(path: & String) {
    let mut path = path.clone();
    if !path.ends_with("\\\0") && path.ends_with("\0") {
        path.pop();
        path.push_str("\\\0");
    }else if path.ends_with("\\") {
        path.push_str("\0");
    }else if !path.ends_with("\\\0"){
        path.push_str("\\\0");
    }

    let h = FindWindowA("#32770", None);
    let h = FindWindowExA(h, None, "WorkerW", None);
    let h = FindWindowExA(h, None, "ReBarWindow32", None);
    let h = FindWindowExA(h, None, "Address Band Root", None);
    let h = FindWindowExA(h, None, "msctls_progress32", None);
    let h = FindWindowExA(h, None, "ComboBoxEx32", None);
    let h = FindWindowExA(h, None, "ComboBox", None);
    let h = FindWindowExA(h, None, "Edit", None);

    let path = path.as_str().as_ptr();

    SendMessageA(h, EM_SETSEL, WPARAM(0), LPARAM(255));
    SendMessageA(h, EM_REPLACESEL, WPARAM(1), LPARAM(path as isize));
    SendMessageA(h, WM_KEYDOWN, WPARAM(usize::try_from(VK_RETURN).unwrap()), LPARAM(0));
}
