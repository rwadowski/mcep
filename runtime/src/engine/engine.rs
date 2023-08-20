use crossbeam_channel::{Receiver, Sender};
use crate::DataFrame;

fn run_engine(source: Receiver<Sender<DataFrame>>, sink: Sender<DataFrame>) -> Result<(), String> {
    Ok(())
}