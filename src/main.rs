extern crate rudit;
extern crate termion;

use std::io::{Write, stdout, Stdout};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(30);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    let mut line_num = 1;
    let (mut line_size, mut start, mut end) = buffer.get_line_size(cy as usize);

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
        .unwrap();
    stdout.flush().unwrap();
    'main: loop {
        // write!(stdout, "{}", termion::clear::All).unwrap();

        draw_lines(&mut stdout, &buffer.buffer);
        draw_info(&mut stdout,
                  index,
                  line_num,
                  cx,
                  cy,
                  line_size,
                  start,
                  end,
                  buffer.gap_start,
                  buffer.gap_end);
        draw_cursor(&mut stdout, cx, cy);
        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    buffer.insert(index, '\n');
                    cx = 3;
                    cy += 1;
                    index += 1;
                    line_num += 1;
                    line_size = buffer.get_line_size(cy as usize).0;
                    start = buffer.get_line_size(cy as usize).1;
                    end = buffer.get_line_size(cy as usize).2;
                }
                Key::Char(c) => {
                    buffer.insert(index, c);
                    index += 1;
                    cx += 1;
                    line_size = buffer.get_line_size(cy as usize).0;
                    start = buffer.get_line_size(cy as usize).1;
                    end = buffer.get_line_size(cy as usize).2;

                }
                Key::Up => {
                    if cy > 1 {
                        cy -= 1;
                        line_size = buffer.get_line_size(cy as usize).0;
                        start = buffer.get_line_size(cy as usize).1;
                        end = buffer.get_line_size(cy as usize).2;

                        //if cx - 3 < line_size as u16 {
                        //    index = start + cx as usize - 3;
                        //} else {
                        //    index = end;
                        //}
                        index = start;
                    }
                }
                Key::Down => {
                    if cy < line_num as u16 {
                        cy += 1;
                        line_size = buffer.get_line_size(cy as usize).0;
                        start = buffer.get_line_size(cy as usize).1;
                        end = buffer.get_line_size(cy as usize).2;
                        if start == end {
                            index = end;
                        }
                        if cx - 3 < buffer.get_line_size(cy as usize - 1).0 as u16 {
                            index = start + cx as usize - 3;
                        }
                    }
                }
                Key::Left => {
                    if cx > 3 {
                        cx -= 1;
                        index -= 1;
                        line_size = buffer.get_line_size(cy as usize).0;
                        start = buffer.get_line_size(cy as usize).1;
                        end = buffer.get_line_size(cy as usize).2;
                    }
                }
                Key::Right => {
                    // not working correctly
                    if cx - 3 < buffer.get_line_size(cy as usize).0 as u16 &&
                       buffer.buffer[index] != '\n' {
                        cx += 1;
                        index += 1;
                        line_size = buffer.get_line_size(cy as usize).0;
                        start = buffer.get_line_size(cy as usize).1;
                        end = buffer.get_line_size(cy as usize).2;
                    }
                }
                Key::Esc => break,
                _ => (),
            }
            continue 'main;
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();
        break;
    }
}
fn draw_info(stdout: &mut RawTerminal<Stdout>,
             index: usize,
             line_num: usize,
             cx: u16,
             cy: u16,
             line_size: usize,
             start: usize,
             end: usize,
             gs: usize,
             ge: usize) {
    write!(stdout,
           "{} index {} line_num {} cx {} cy {} line_size {} start   {} end   {}      gs {} ge {}",
           termion::cursor::Goto(0, termion::terminal_size().unwrap().1),
           index,
           line_num,
           cx - 2,
           cy,
           line_size,
           start,
           end,
           gs,
           ge)
        .unwrap();
}
fn draw_lines(stdout: &mut RawTerminal<Stdout>, buffer: &Vec<char>) {
    let s: String = buffer.iter().collect();
    for (index, i) in s.split('\n').enumerate() {
        write!(stdout,
               "{}{}{}{}",
               Goto(0, (index + 1) as u16),
               index + 1,
               Goto(3, (index + 1) as u16),
               i)
            .unwrap();
    }
}

fn draw_cursor(stdout: &mut RawTerminal<Stdout>, cx: u16, cy: u16) {
    write!(stdout, "{}", termion::cursor::Goto(cx, cy)).unwrap();
}
