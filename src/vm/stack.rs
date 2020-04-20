use crate::vm::data::Data;
use crate::vm::local::Local;

pub type Stack = Vec<Item>;

// must be under usize in size, so box everything?
// See: https://docs.rs/tagged-box/0.1.1/tagged_box/index.html
// I would like Passerine to be a no-dependancy implementation, however
// TODO: remove redundant boxes
// No clone!
// if data needs to be cloned, clone the data and put it into a new Item
#[derive(Debug, PartialEq)]
pub enum Item {
    Frame,
    // Lambda(Box<Bytecode>),
    // TODO: Locals: stored on the stack, or stored in frames?
    Local { local: Local, data: Data },
    Data(Data),
}
