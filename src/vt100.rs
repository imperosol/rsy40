/// Sur Windows, les codes d'échappement sont pas forcément activés,
/// donc faut les remettre
#[cfg(windows)]
pub fn init() {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{DISABLE_NEWLINE_AUTO_RETURN, ENABLE_VIRTUAL_TERMINAL_PROCESSING};

    let mut state: DWORD = 0;
    unsafe {
        let console = GetStdHandle(STD_OUTPUT_HANDLE);
        if GetConsoleMode(console, &mut state) != 0 {
            state |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            state &= !DISABLE_NEWLINE_AUTO_RETURN;
            SetConsoleMode(console, state);
        }
    }
}

#[cfg(not(windows))]
pub fn init() {}