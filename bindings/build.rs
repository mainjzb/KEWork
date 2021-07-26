fn main() {
    windows::build!(
        Windows::Foundation::*,
        Windows::Foundation::Collections::*,
        Windows::Win32::UI::WindowsAndMessaging::*,
        Windows::Win32::UI::Controls::*,
        Windows::Win32::Foundation::*,
    );
}
