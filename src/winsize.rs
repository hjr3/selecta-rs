use libc::{c_int, c_ulong, c_ushort, STDOUT_FILENO};
use libc::funcs::bsd44::ioctl;

use std::io::{IoResult, standard_error, ResourceUnavailable};

#[repr(C)]
struct winsize {
    ws_row: c_ushort,     /* rows, in characters */
    ws_col: c_ushort,     /* columns, in characters */
    ws_xpixel: c_ushort,  /* horizontal size, pixels */
    ws_ypixel: c_ushort   /* vertical size, pixels */
}

const TIOCGWINSZ: c_ulong = 0x40087468;

pub fn winsize() -> IoResult<(u16, u16)> {
    let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };    

    match r {
        0 => Ok((w.ws_col as u16, w.ws_row as u16)),
        _ => {
            return Err(standard_error(ResourceUnavailable))
        }
    }
}

#[test]
fn test_winsize_has_valid_width_and_height() {
    let (width, height) = winsize().unwrap();
    assert!(width > 0);
    assert!(height > 0);
}
