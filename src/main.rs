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
    let mut buffer = GapBuffer::new(50);
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
    // line_num = open_file(&mut buffer, line_num);
    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();

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
                        // println!("{}",line_size);
                        if cx <= line_size as u16 + 1 {
                            // println!("        bb    ");
                            // println!("{}", buffer.get_line_size(cy as usize + 1).0);
                            index = start + cx as usize - 3;
                        } else {
                            // println!("[oooo]");
                            cx = end as u16 + 3;
                            index = end;
                            //                            index = end;
                        }
                    }
                }
                Key::Down => {
                    if cy < line_num as u16 {
                        cy += 1;
                        line_size = buffer.get_line_size(cy as usize).0;
                        start = buffer.get_line_size(cy as usize).1;
                        end = buffer.get_line_size(cy as usize).2;

                        if line_size == 1 {
                            cx = 3;
                            index = start;
                        }
                        if cx <= line_size as u16 {
                            println!("ortontriont");
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
                    if cx < buffer.get_line_size(cy as usize).0 as u16 + 2 {

                        cx += 1;
                        index += 1;

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

fn open_file(buffer: &mut GapBuffer, mut line_num: usize) -> usize {
    use std::io::{BufRead, BufReader};
    use std::fs::File;
    use std::io::prelude::*;
    let mut string = String::from("");
    BufReader::new(File::open("/home/andre/test2").unwrap()).read_to_string(&mut string);
    for (index, c) in string.chars().enumerate() {
        if c == '\n' {
            line_num += 1;
        }
        buffer.insert(index, c);
    }
    line_num
}
