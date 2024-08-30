#![allow(unused)]

use std::io::Write;
use std::{thread, time};
// mpsc - multi producer single consumer
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

pub struct Spinner {
    tx: Sender<Option<String>>,
    // TODO: return type
    // TODO: why option
    join_handle: Option<thread::JoinHandle<()>>,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        // TODO: what dis?
        self.tx.send(None);
        self.join_handle.take().unwrap().join().unwrap();
    }
}

impl Spinner {
    pub fn new() -> Self {
        let frames = vec!["-", "\\", "|", "/"];

        // transmitter and receiver
        let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = channel();
        let mut stream = std::io::stdout();

        // TODO: when to call join
        let join_handle = thread::spawn(move || {
            loop {
                for f in frames.iter() {
                    // Does not block
                    let (stop, msg) = match rx.try_recv() {
                        Ok(msg) => (true, msg),
                        Err(TryRecvError::Disconnected) => (true, None),
                        Err(TryRecvError::Empty) => (false, None),
                    };

                    if stop {
                        if let Some(msg) = msg {
                            println!("\r{}", msg);
                        }
                        return ();
                    }

                    write!(stream, "\r{}", f).unwrap();
                    stream.flush().unwrap();

                    thread::sleep(time::Duration::from_millis(200));
                }
            }
        });

        Self {
            tx,
            join_handle: Some(join_handle),
        }
    }

    pub fn stop(&mut self) {
        self.tx.send(Some(String::from("stopped")));
    }
}

// TODO: implement Drop

fn main() {
    let mut spinner = Spinner::new();
    thread::sleep(time::Duration::from_secs(3));
    spinner.stop();
}
