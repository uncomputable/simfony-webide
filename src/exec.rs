use std::fmt;

use itertools::Itertools;
use simplicity::{Cmr, FailEntropy};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Error {
    FrameEof,
    MoveUnfinishedFrame,
    PrunedBranch(Cmr),
    FailNode(FailEntropy),
    JetsNotSupported,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FrameEof => write!(f, "Unexpected end of frame"),
            Error::MoveUnfinishedFrame => write!(f, "Unfinished frame cannot be moved"),
            Error::PrunedBranch(cmr) => write!(f, "Cannot execute pruned branch: {cmr}"),
            Error::FailNode(entropy) => write!(f, "Cannot execute fail node: {entropy}"),
            Error::JetsNotSupported => write!(f, "Jets are currently not supported"),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Frame {
    cells: Vec<bool>,
    cursor: usize,
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, &cell) in self.cells.iter().enumerate() {
            if i == self.cursor {
                write!(f, "∧")?;
            }
            write!(f, "{}", if cell { '1' } else { '0' })?;
        }
        if self.cursor == self.cells.len() {
            write!(f, "∧")?;
        }

        write!(f, "]")
    }
}

impl Frame {
    pub fn new(bit_size: usize) -> Self {
        Self {
            cells: vec![false; bit_size],
            cursor: 0,
        }
    }

    pub fn write(&mut self, bit: bool) -> Result<(), Error> {
        debug_assert!(self.cursor <= self.cells.len());
        if self.cells.len() <= self.cursor {
            Err(Error::FrameEof)
        } else {
            self.cells[self.cursor] = bit;
            self.cursor = self.cursor.saturating_add(1);
            Ok(())
        }
    }

    pub fn peek(&self) -> bool {
        debug_assert!(self.cursor <= self.cells.len());
        self.cells[self.cursor]
    }

    pub fn advance_cursor(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(self.cursor <= self.cells.len());
        if self.cursor.saturating_add(bit_len) <= self.cells.len() {
            self.cursor = self.cursor.saturating_add(bit_len);
            Ok(())
        } else {
            Err(Error::FrameEof)
        }
    }

    pub fn retract_cursor(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(self.cursor <= self.cells.len());
        if let Some(new_cursor) = self.cursor.checked_sub(bit_len) {
            self.cursor = new_cursor;
            Ok(())
        } else {
            Err(Error::FrameEof)
        }
    }

    pub fn copy(&self, other: &mut Self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(self.cursor <= self.cells.len());
        let self_new_cursor = self.cursor.saturating_add(bit_len);
        let other_new_cursor = other.cursor.saturating_add(bit_len);

        if other_new_cursor <= other.cells.len() {
            other.cells[other.cursor..other_new_cursor]
                .copy_from_slice(&self.cells[self.cursor..self_new_cursor]);
            other.cursor = other_new_cursor;
            Ok(())
        } else {
            Err(Error::FrameEof)
        }
    }

    pub fn write_bitstring(&mut self, bitstring: &[bool]) -> Result<(), Error> {
        debug_assert!(self.cursor <= self.cells.len());
        let self_new_cursor = self.cursor.saturating_add(bitstring.len());

        if self_new_cursor <= self.cells.len() {
            self.cells[self.cursor..self_new_cursor].copy_from_slice(bitstring);
            self.cursor = self_new_cursor;
            Ok(())
        } else {
            Err(Error::FrameEof)
        }
    }

    pub fn is_finished(&self) -> bool {
        self.cursor == self.cells.len()
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BitMachine {
    read_stack: Vec<Frame>,
    write_stack: Vec<Frame>,
}

impl fmt::Display for BitMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_stack = |stack: &[Frame]| stack.iter().map(|x| format!("{}", x)).join(" ");
        write!(
            f,
            "Read {} Write {}",
            fmt_stack(&self.read_stack),
            fmt_stack(&self.write_stack)
        )
    }
}

impl BitMachine {
    pub fn for_program() -> Self {
        Self {
            read_stack: vec![Frame::new(0)],  // unit source value
            write_stack: vec![Frame::new(0)], // unit target value
        }
    }

    pub fn read_stack(&self) -> &[Frame] {
        &self.read_stack
    }

    pub fn write_stack(&self) -> &[Frame] {
        &self.write_stack
    }

    pub fn new_frame(&mut self, bit_len: usize) {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        self.write_stack.push(Frame::new(bit_len));
    }

    pub fn move_frame(&mut self) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let mut write = self.write_stack.pop().unwrap();
        if write.is_finished() {
            write.reset_cursor();
            self.read_stack.push(write);
            Ok(())
        } else {
            Err(Error::MoveUnfinishedFrame)
        }
    }

    pub fn drop_frame(&mut self) {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let _ = self.read_stack.pop().unwrap();
    }

    pub fn write(&mut self, bit: bool) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let write = self.write_stack.last_mut().unwrap();
        write.write(bit)
    }

    pub fn write_bitstring(&mut self, bitstring: &[bool]) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let write = self.write_stack.last_mut().unwrap();
        write.write_bitstring(bitstring)
    }

    pub fn skip(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let write = self.write_stack.last_mut().unwrap();
        write.advance_cursor(bit_len)
    }

    pub fn copy(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let read = self.read_stack.last_mut().unwrap();
        let write = self.write_stack.last_mut().unwrap();
        read.copy(write, bit_len)
    }

    pub fn fwd(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let read = self.read_stack.last_mut().unwrap();
        read.advance_cursor(bit_len)
    }

    pub fn bwd(&mut self, bit_len: usize) -> Result<(), Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let read = self.read_stack.last_mut().unwrap();
        read.retract_cursor(bit_len)
    }

    pub fn peek(&mut self) -> Result<bool, Error> {
        debug_assert!(!self.read_stack.is_empty() && !self.write_stack.is_empty());
        let read = self.read_stack.last_mut().unwrap();
        Ok(read.peek())
    }
}
