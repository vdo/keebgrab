use std::{
    os::raw::{c_int, c_uchar},
    ptr::{null_mut,null}
};

use std::mem::{transmute,zeroed};
use x11::{xinput2, xlib};

fn keycode<'k>(code: i32) -> &'k str {
    match code {
      0x0009 => "ESC",
      0x000A => "1",
      0x000B => "2",
      0x000C => "3",
      0x000D => "4",
      0x000E => "5",
      0x000F => "6",
      0x0010 => "7",
      0x0011 => "8",
      0x0012 => "9",
      0x0013 => "0",
      0x0014 => "-",
      0x0015 => "=",
      0x0016 => "<Backspace>",
      0x0017 => "<Tab>",
      0x0018 => "Q",
      0x0019 => "W",
      0x001A => "E",
      0x001B => "R",
      0x001C => "T",
      0x001D => "Y",
      0x001E => "U",
      0x001F => "I",
      0x0020 => "O",
      0x0021 => "P",
      0x0022 => "[",
      0x0023 => "]",
      0x0024 => "<Enter>",
      0x0025 => "<ControlLeft>",
      0x0026 => "A",
      0x0027 => "S",
      0x0028 => "D",
      0x0029 => "F",
      0x002A => "G",
      0x002B => "H",
      0x002C => "J",
      0x002D => "K",
      0x002E => "L",
      0x002F => ";",
      0x0030 => "'",
      0x0031 => "`",
      0x0032 => "<ShiftLeft>",
      0x0033 => "\\",
      0x0034 => "Z",
      0x0035 => "X",
      0x0036 => "C",
      0x0037 => "V",
      0x0038 => "B",
      0x0039 => "N",
      0x003A => "M",
      0x003B => ",",
      0x003C => ".",
      0x003D => "/",
      0x003E => "<ShiftRight>",
      0x003F => "<NumpadMultiply>",
      0x0040 => "<AltLeft>",
      0x0041 => "<Space>",
      0x0042 => "<CapsLock>",
      0x0043 => "<F1>",
      0x0044 => "<F2>",
      0x0045 => "<F3>",
      0x0046 => "<F4>",
      0x0047 => "<F5>",
      0x0048 => "<F6>",
      0x0049 => "<F7>",
      0x004A => "<F8>",
      0x004B => "<F9>",
      0x004C => "<F10>",
      0x004D => "<NumLock>",
      0x004E => "<ScrollLock>",
      0x004F => "<Numpad7>",
      0x0050 => "<Numpad8>",
      0x0051 => "<Numpad9>",
      0x0052 => "<NumpadSubtract>",
      0x0053 => "<Numpad4>",
      0x0054 => "<Numpad5>",
      0x0055 => "<Numpad6>",
      0x0056 => "<NumpadAdd>",
      0x0057 => "<Numpad1>",
      0x0058 => "<Numpad2>",
      0x0059 => "<Numpad3>",
      0x005A => "<Numpad0>",
      0x005B => "<NumpadDecimal>",
      0x005E => "<IntlBackslash>",
      0x005F => "<F11>",
      0x0060 => "<F12>",
      0x0061 => "<IntlRo>",
      0x0064 => "<Convert>",
      0x0065 => "<KanaMode>",
      0x0066 => "<NonConvert>",
      0x0068 => "<NumpadEnter>",
      0x0069 => "<ControlRight>",
      0x006A => "<NumpadDivide>",
      0x006B => "<PrintScreen>",
      0x006C => "<AltRight>",
      0x006E => "<Home>",
      0x006F => "<ArrowUp>",
      0x0070 => "<PageUp>",
      0x0071 => "<ArrowLeft>",
      0x0072 => "<ArrowRight>",
      0x0073 => "<End>",
      0x0074 => "<ArrowDown>",
      0x0075 => "<PageDown>",
      0x0076 => "<Insert>",
      0x0077 => "<Delete>",
      0x0079 => "<AudioVolumeMute>",
      0x007A => "<AudioVolumeDown>",
      0x007B => "<AudioVolumeUp>",
      0x007D => "<NumpadEqual>",
      0x007F => "<Pause>",
      0x0081 => "<NumpadComma>",
      0x0082 => "<Lang1>",
      0x0083 => "<Lang2>",
      0x0084 => "<IntlYen>",
      0x0085 => "<SuperLeft>",
      0x0086 => "<SuperRight>",
      0x0087 => "<ContextMenu>",
      0x0088 => "<BrowserStop>",
      0x0089 => "<Again>",
      0x008A => "<Props>",
      0x008B => "<Undo>",
      0x008C => "<Select>",
      0x008D => "<Copy>",
      0x008E => "<Open>",
      0x008F => "<Paste>",
      0x0090 => "<Find>",
      0x0091 => "<Cut>",
      0x0092 => "<Help>",
      0x0094 => "<LaunchApp2>",
      0x0097 => "<WakeUp>",
      0x0098 => "<LaunchApp1>",
      0x00A3 => "<LaunchMail>",
      0x00A4 => "<BrowserFavorites>",
      0x00A6 => "<BrowserBack>",
      0x00A7 => "<BrowserForward>",
      0x00A9 => "<Eject>",
      0x00AB => "<MediaTrackNext>",
      0x00AC => "<MediaPlayPause>",
      0x00AD => "<MediaTrackPrevious>",
      0x00AE => "<MediaStop>",
      0x00B3 => "<MediaSelect>",
      0x00B4 => "<BrowserHome>",
      0x00B5 => "<BrowserRefresh>",
      0x00E1 => "<BrowserSearch>",
      _ => "<Unidentified>",
    }
  }

fn main() {
    unsafe {
        // Create display
        let display = xlib::XOpenDisplay(null());
        if display == null_mut() {
            panic!("can't open display");
        }
        let screen_num = xlib::XDefaultScreen(display);
        // Use root window
        let root = xlib::XRootWindow(display, screen_num);
        // Select events for all input devices
        let mask = xinput2::XI_RawKeyPressMask | xinput2::XI_RawKeyReleaseMask;
        let mut event_mask = xinput2::XIEventMask {
            deviceid: xinput2::XIAllMasterDevices,
            mask: &mask as *const _ as *mut c_uchar,
            mask_len: std::mem::size_of_val(&mask) as c_int,
        };
        xinput2::XISelectEvents(display, root, &mut event_mask, 1);
        // Event loop
        let mut event: xlib::XEvent = zeroed();
        xlib::XNextEvent(display, &mut event); // Ignore first key release
        loop {
            xlib::XNextEvent(display, &mut event);
            if xlib::True == {
                xlib::XFilterEvent(&mut event, {
                    let xev: &xlib::XAnyEvent = event.as_ref();
                    xev.window
                })
            } {
                continue;
            }
            let event_type = event.get_type();
            match event_type {
                xlib::GenericEvent => {
                    let mut xev = event.generic_event_cookie;
                    if xlib::XGetEventData(display, &mut xev) == xlib::True {
                        match xev.evtype {
                            xinput2::XI_RawKeyPress | xinput2::XI_RawKeyRelease => {
                                let event_data: &xinput2::XIDeviceEvent = transmute(xev.data);
                                if xev.evtype == xinput2::XI_RawKeyPress {
                                    if event_data.flags & xinput2::XIKeyRepeat == 0 {
                                        println!("Key {} pressed", keycode(event_data.detail));
                                    }
                                } else {
                                    println!("Key {} released", keycode(event_data.detail));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
