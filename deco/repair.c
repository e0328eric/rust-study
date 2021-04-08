//
// Created by Almagest on 2021/04/07.
// Repair a terminal without terminating terminal itself
//

#include <unistd.h>
#include <termios.h>

int main(void)
{
    struct termios term;
    tcgetattr(STDIN_FILENO, &term);
    term.c_iflag &= 0; term.c_iflag |= 0x2b02;
    term.c_oflag &= 0; term.c_oflag |= 0x3;
    term.c_cflag &= 0; term.c_cflag |= 0x4b00;
    term.c_lflag &= 0; term.c_lflag |= 0x200005cb;
    term.c_cc[VMIN] = 1;
    term.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &term);

    return 0;
}
