use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal;
use encode::Frame;
use image::{imageops::FilterType, io::Reader as ImageReader};
use std::io::stdout;
use std::sync::mpsc;
use std::{
    error::Error,
    thread,
    time::{Duration, SystemTime},
};
mod encode;

const FRAME_RATE: u128 = 90;
const FRAME_TIME: u128 = 1000 / FRAME_RATE;
const BUF_SIZE: usize = 200;

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = terminal::size().map(|(x, y)| (x as usize, y as usize))?;

    let (frame_sender, frame_recv) = mpsc::sync_channel(BUF_SIZE);

    thread::spawn(move || {
        for i in 1..3300 {
            let img = ImageReader::open(format!("frames/{:04}.jpg", i))
                .unwrap()
                .decode()
                .unwrap();
            let img = img.resize_exact(width as u32, (height - 1) as u32, FilterType::Nearest);
            let frame = Frame::from(img);
            frame_sender.send(frame.print(width, height)).unwrap();
        }
    });

    for frame in frame_recv {
        let start = SystemTime::now();
        update(frame)?;

        let cost_time = SystemTime::now().duration_since(start).unwrap().as_millis();
        print!("\rframe time: {cost_time}ms  ");
        if cost_time < FRAME_TIME {
            thread::sleep(Duration::from_millis((FRAME_TIME - cost_time) as u64));
        }
    }

    Ok(())
}

fn update(frame: String) -> Result<(), Box<dyn Error>> {
    execute!(stdout(), MoveTo(0, 0))?;
    print!("{frame}");

    Ok(())
}
