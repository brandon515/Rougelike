extern crate ncurses;

fn setup() -> Receiver<String>{
    ncurses::initscr();
    ncurses::noecho();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
    let (inputSender, inputRec) = channel();
    spawn(proc(){
        sender.send(ncurses::getch());
    });
    inputRec
}

fn is_running(rec: Receiver<>) -> bool{
    let res = rec.try_recv().ok();
    let ch = match res{
        Some(x) => x,
        None    => pass,
    };
    ncurses::attron(ncurses::A_BLINK());
    ncurses::addch();
    ncurses::attroff(ncurses::A_BLINK());
    if ch == ncurses::KEY_F(2){
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
    let ch = setup();
    while is_running(ch){
        main_loop();
    }
    clean_up();
}
