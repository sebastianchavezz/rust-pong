extern crate ncurses;

use ncurses::*;
use std::{thread, time};
const PADDLE_SIZE: i32 = 3;

const BALL: char = 'O';

struct Paddle {
    x: i32,
    y: i32,
}

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn draw_paddle(paddle: &Paddle, win: WINDOW){
    for i in 0..PADDLE_SIZE{
        mvwaddch(win, paddle.y + i, paddle.x, ACS_CKBOARD() as u32);
    }
}

fn draw_ball(ball: &Ball, win: WINDOW){
    mvwaddch(win, ball.y, ball.x, BALL as u32);
}
fn main(){
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);

    let win = stdscr();
    let mut paddle1 = Paddle{x: 0, y: 0};
    let mut paddle2 = Paddle{x: 0, y: 0};
    let mut ball = Ball{x: 0, y: 0, dx: 1, dy: 1};

    loop{
        clear();
        getmaxyx(win, &mut paddle1.y, &mut paddle1.x);
        draw_paddle(&paddle1, win);
        draw_paddle(&paddle2, win);
        draw_ball(&ball, win);
        
        let key = getch();

        if key == 's' as i32{
            paddle1.y -= 1;
        }
        if key == 'z' as i32{
            paddle1.y += 1;
        }

        ball.x += ball.dx;
        ball.y += ball.dy;

        if ball.x < 0 || ball.x > paddle1.x{
            ball.dx *= -1;
        }
        if ball.y < 0 || ball.y > paddle1.y{
            ball.dy *= -1;
        }
        thread::sleep(time::Duration::from_millis(10));

    }
    endwin();
}
