use rand::prelude::*;
use rand::distributions::Alphanumeric;
use nix::ioctl_read_bad;
use nix::libc::winsize;
use nix::libc::TIOCGWINSZ;

ioctl_read_bad!(tiocgwinsz, TIOCGWINSZ, winsize);

/// Get the current width of the terminal window
pub fn get_term_width() -> u16 {
    unsafe {
        let mut data: winsize = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        tiocgwinsz(1, &mut data as *mut winsize).unwrap();
        data.ws_col
    }
}

/// Get a randomly generated, 4 character id
pub fn get_random_id() -> String {
    let mut rng = rand::thread_rng();

    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(4)
        .collect::<String>()
        .to_lowercase()
}
