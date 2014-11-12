extern crate ncurses;

fn setup(){
    ncurses::initscr();
    ncurses::noecho();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr, true);
}

fn is_running() -> bool{
    let ch = ncurses::getch();
    ncurses::printw("%c", ch);
    if ch == ncurses::KEY_F(1){
        return false;
    }
    return true;
}

fn main_loop(){
    ncurses::refresh();
}

fn clean_up(){
    ncurses::endwin();
}

fn main() {
    setup();
    while is_running(){
        main_loop();
    }
    clean_up();
}
