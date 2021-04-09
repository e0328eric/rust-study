use crate::term_ansi::*;
use libc::{ioctl, tcgetattr, tcsetattr, termios, STDIN_FILENO, STDOUT_FILENO};
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::process::exit;

macro_rules! die {
    ($run: expr; $msg: literal) => {
        if $run == -1 {
            eprint!("{}\r\n", $msg);
            exit(1);
        }
    };
}

const fn ctrl_key(k: u8) -> u8 {
    k & 0x1F
}

pub struct Terminal {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    term: libc::termios,
}

impl Terminal {
    /*** Main Terminal ***/
    pub fn new() -> Self {
        let mut term = MaybeUninit::<termios>::uninit();
        // SAFTY: tcgetattr is a safe (maybe?) that defined in <termios.h>
        unsafe {
            die!(tcgetattr(STDIN_FILENO, term.as_mut_ptr()); "tcgetattr");
        }
        // SAFTY: We just initialize term using tcgetattr function.
        let term = unsafe { term.assume_init() };

        let mut raw = term;
        raw.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        raw.c_oflag &= !libc::OPOST;
        raw.c_cflag |= libc::CS8;
        raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        raw.c_cc[libc::VMIN] = 0;
        raw.c_cc[libc::VTIME] = 1;
        // SAFTY: tcsetattr is a safe (maybe?) that defined in <termios.h>
        unsafe {
            die!(tcsetattr(STDIN_FILENO, libc::TCSAFLUSH, &raw); "tcsetattr");
        }

        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            term,
        }
    }

    fn reset_term(&self) {
        // SAFTY: tcsetattr is a safe (maybe?) that defined in <termios.h>
        unsafe {
            die!(tcsetattr(STDIN_FILENO, libc::TCSAFLUSH, &self.term); "tcsetattr");
        }
    }

    pub fn get_window_size() -> io::Result<(usize, usize)> {
        let mut ws = MaybeUninit::<libc::winsize>::uninit();

        // SAFETY: ioctl is a (maybe?) save C function
        if unsafe { ioctl(STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) } == -1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot get window size",
            ));
        }
        // SAFETY: As we initialize ws at the above, we can use assume_init
        let ws = unsafe { ws.assume_init() };
        if ws.ws_col == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot get window size",
            ));
        }

        Ok((ws.ws_row as usize, ws.ws_col as usize))
    }

    /*** Input ***/
    fn read_key(&mut self) -> u8 {
        let mut c = [0u8];
        loop {
            match self.stdin.read(&mut c) {
                Ok(1) => return c[0],
                Ok(_) => { /* FALLTHROUGH */ }
                Err(_) => {
                    eprint!("read\r\n");
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn process_keypress(&mut self) {
        let c = self.read_key();

        match c {
            _ if c == ctrl_key(b'q') => {
                self.refresh_screen();
                self.reset_term();
                exit(0);
            }
            _ => { /* FALLTHROUGH */ }
        }
    }

    /*** Output ***/
    pub fn draw_rows(&mut self) {
        for _ in 0..24 {
            print!("~\r\n");
        }
    }

    pub fn refresh_screen(&mut self) {
        print!("{}", ERASE_ENTIRE_SCREEN);
        print!("{}", MOVE_CURSOR_TOP_LEFT);

        self.draw_rows();

        print!("{}", MOVE_CURSOR_TOP_LEFT);
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.reset_term();
    }
}
