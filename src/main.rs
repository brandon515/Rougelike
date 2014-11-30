extern crate ncurses;

fn get_exit_key() -> i32{
    ncurses::KEY_F(2)
}

fn setup(){
    ncurses::initscr();
    ncurses::noecho();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
    ncurses::ll::WINDOW win = ncurses::newwin(20, 20, 20, 20);
    ncurses::box(win, 0, 0);
    ncurses::wrefresh(win);
   /* let (input_sender, input_rec) = channel::<i32>();
    spawn(proc(){
        loop{
            let ch = ncurses::getch();
            input_sender.send(ch as i32);
            if ch == get_exit_key(){
                break;
            }
        }
    });
    input_rec*/
}


fn clean_up(){
    ncurses::endwin();
}

fn main() {
    setup();
    loop{
        let ch = ncurses::getch();
        if ch == get_exit_key(){
            break;
        }
        /*else if ch == ncurses::KEY_RESIZE{
        }*/
        ncurses::printw(ncurses::LINES.to_string().as_slice());
        ncurses::refresh();
    }
    clean_up();
}
