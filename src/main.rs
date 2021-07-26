use bindings::{
    Windows::Win32::UI::WindowsAndMessaging::*,
    Windows::Win32::UI::Controls::*,
    Windows::Win32::Foundation::*,
};

use std::convert::TryFrom;
use std::env;
use std::process;



fn main() -> windows::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Problem parsing arguments need path " );
        process::exit(1);
    }
    let path = &args[1];


    unsafe {
        run(path)
    }

    Ok(())
}

fn addCstringEnd(path: &str) -> String{
    let mut path = path.to_string();
    if !path.ends_with("\\\0") && path.ends_with("\0") {
        path.pop();
        path.push_str("\\\0");
    }else if path.ends_with("\\") {
        path.push_str("\0");
    }else if !path.ends_with("\\\0"){
        path.push_str("\\\0");
    }
    path
}


unsafe fn run(path: &str) {
    //let mut path:String = path.to_string();
    let path = addCstringEnd( path);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        assert_eq!(addCstringEnd("C:\\git"), "C:\\git\\\0".to_string());
        assert_eq!(addCstringEnd("C:\\git\\"), "C:\\git\\\0".to_string());
        assert_eq!(addCstringEnd("C:\\git\0"), "C:\\git\\\0".to_string());
        assert_eq!(addCstringEnd("C:\\git\\\0"), "C:\\git\\\0".to_string());
    }
}