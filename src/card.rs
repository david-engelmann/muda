//! The rates card reader (SB-20): a strict scanner
//! over the decoded text that decides every refusal
//! in parse order, as the Python's json.loads with
//! its duplicate hook does. A duplicate key, decoded
//! before it is compared, refuses by name at the
//! close of its object, where the hook fires; a text
//! nested past the reader's limit refuses as not
//! JSON where the nesting reaches it; an integer
//! literal past the interpreter's digit limit
//! refuses as not JSON. Everything else the hook
//! accepts, this accepts: NaN and the infinities
//! read as floats, duplicate-free objects keep their
//! document order, and integers keep their digits.

use crate::value::{Text, Value};

/// The card reader's nesting limit, in containers
/// from the text's root (the card object is 1),
/// calibrated against the Python's reader at the
/// shipped CLI: an array chain of 9993 under a key
/// parses (total 9994) and 9994 refuses.
pub(crate) const DEPTH_LIMIT: usize = 9994;

/// An integer literal's digit limit, the
/// interpreter's conversion bound: past it the
/// Python's reader raises and the card is not JSON.
pub(crate) const INT_DIGIT_LIMIT: usize = 4300;

#[derive(Debug, PartialEq)]
pub enum CardError {
    /// The first key, in pair order, that an object
    /// names twice, detected when the object closes.
    Duplicate(Text),
    /// Any other refusal of the text: not JSON.
    NotJson,
}

enum Frame {
    Array(Vec<Value>),
    Object {
        entries: Vec<(Text, Value)>,
        pending: Option<Text>,
    },
}

struct Scanner<'a> {
    units: &'a [u16],
    at: usize,
}

impl<'a> Scanner<'a> {
    fn peek(&self) -> Option<u16> {
        self.units.get(self.at).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(unit) = self.peek() {
            if matches!(unit, 0x20 | 0x09 | 0x0A | 0x0D) {
                self.at += 1;
            } else {
                break;
            }
        }
    }

    fn literal(&mut self, word: &str) -> Result<(), CardError> {
        for expected in word.encode_utf16() {
            if self.peek() != Some(expected) {
                return Err(CardError::NotJson);
            }
            self.at += 1;
        }
        Ok(())
    }

    fn string(&mut self) -> Result<Text, CardError> {
        // The caller has seen the opening quote.
        self.at += 1;
        let mut out: Vec<u16> = Vec::new();
        loop {
            let unit = self.peek().ok_or(CardError::NotJson)?;
            self.at += 1;
            match unit {
                0x22 => return Ok(Text::from_units(out)),
                0x5C => {
                    let escape = self.peek().ok_or(CardError::NotJson)?;
                    self.at += 1;
                    match escape {
                        0x22 => out.push(0x22),
                        0x5C => out.push(0x5C),
                        0x2F => out.push(0x2F),
                        0x62 => out.push(0x08),
                        0x66 => out.push(0x0C),
                        0x6E => out.push(0x0A),
                        0x72 => out.push(0x0D),
                        0x74 => out.push(0x09),
                        0x75 => {
                            let mut value: u32 = 0;
                            for _ in 0..4 {
                                let digit = self.peek().ok_or(CardError::NotJson)?;
                                self.at += 1;
                                value = value * 16
                                    + match digit {
                                        0x30..=0x39 => (digit - 0x30) as u32,
                                        0x61..=0x66 => (digit - 0x61) as u32 + 10,
                                        0x41..=0x46 => (digit - 0x41) as u32 + 10,
                                        _ => return Err(CardError::NotJson),
                                    };
                            }
                            out.push(value as u16);
                        }
                        _ => return Err(CardError::NotJson),
                    }
                }
                0x00..=0x1F => return Err(CardError::NotJson),
                _ => out.push(unit),
            }
        }
    }

    fn number(&mut self) -> Result<Value, CardError> {
        let start = self.at;
        if self.peek() == Some(0x2D) {
            self.at += 1;
        }
        // The integer part: 0, or a nonzero digit run.
        match self.peek() {
            Some(0x30) => {
                self.at += 1;
            }
            Some(0x31..=0x39) => {
                while matches!(self.peek(), Some(0x30..=0x39)) {
                    self.at += 1;
                }
            }
            _ => return Err(CardError::NotJson),
        }
        let mut is_float = false;
        if self.peek() == Some(0x2E) {
            is_float = true;
            self.at += 1;
            if !matches!(self.peek(), Some(0x30..=0x39)) {
                return Err(CardError::NotJson);
            }
            while matches!(self.peek(), Some(0x30..=0x39)) {
                self.at += 1;
            }
        }
        if matches!(self.peek(), Some(0x65) | Some(0x45)) {
            is_float = true;
            self.at += 1;
            if matches!(self.peek(), Some(0x2B) | Some(0x2D)) {
                self.at += 1;
            }
            if !matches!(self.peek(), Some(0x30..=0x39)) {
                return Err(CardError::NotJson);
            }
            while matches!(self.peek(), Some(0x30..=0x39)) {
                self.at += 1;
            }
        }
        let token: String = self.units[start..self.at]
            .iter()
            .map(|&unit| unit as u8 as char)
            .collect();
        if is_float {
            token
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| CardError::NotJson)
        } else {
            let digits = token.trim_start_matches('-').len();
            if digits > INT_DIGIT_LIMIT {
                return Err(CardError::NotJson);
            }
            // CPython's int reading drops the sign of
            // zero: -0 is the integer 0, and its repr
            // is "0".
            let token = if token == "-0" {
                "0".to_string()
            } else {
                token
            };
            Ok(Value::Int(token))
        }
    }
}

/// Attach a completed value to the open frame, or
/// finish the parse when no frame is open.
fn attach(stack: &mut [Frame], value: Value) -> Result<Option<Value>, CardError> {
    match stack.last_mut() {
        None => Ok(Some(value)),
        Some(Frame::Array(items)) => {
            items.push(value);
            Ok(None)
        }
        Some(Frame::Object { entries, pending }) => {
            let key = pending.take().ok_or(CardError::NotJson)?;
            entries.push((key, value));
            Ok(None)
        }
    }
}

/// Close the innermost object, firing the duplicate
/// check the Python's hook performs at the close:
/// the first key, in pair order, that appears twice.
fn close_object(frame: Frame) -> Result<Value, CardError> {
    match frame {
        Frame::Array(items) => Ok(Value::Array(items)),
        Frame::Object { entries, .. } => {
            let mut seen: Vec<&Text> = Vec::with_capacity(entries.len());
            for (key, _) in &entries {
                if seen.contains(&key) {
                    return Err(CardError::Duplicate(key.clone()));
                }
                seen.push(key);
            }
            Ok(Value::Object(entries))
        }
    }
}

/// Parse the card text (already decoded from UTF-8
/// with any BOM removed by the caller, as the
/// Python's utf-8-sig decode leaves it).
pub fn parse_card(units: &[u16]) -> Result<Value, CardError> {
    let mut scanner = Scanner { units, at: 0 };
    let mut stack: Vec<Frame> = Vec::new();
    // The value just completed and not yet attached;
    // the loop attaches before reading further.
    let mut root: Option<Value> = None;
    'outer: loop {
        // Decide what comes next from the open frame.
        enum Next {
            KeyOrEnd,
            Colon,
            Value,
            CommaOrEnd,
        }
        let next = match stack.last() {
            None => {
                if root.is_some() {
                    break 'outer;
                }
                Next::Value
            }
            Some(Frame::Array(items)) => {
                if items.is_empty() {
                    Next::Value
                } else {
                    Next::CommaOrEnd
                }
            }
            Some(Frame::Object { entries, pending }) => {
                if pending.is_some() {
                    Next::Colon
                } else if entries.is_empty() {
                    Next::KeyOrEnd
                } else {
                    Next::CommaOrEnd
                }
            }
        };
        match next {
            Next::KeyOrEnd => {
                scanner.skip_ws();
                match scanner.peek() {
                    Some(0x7D) => {
                        scanner.at += 1;
                        let frame = stack.pop().ok_or(CardError::NotJson)?;
                        let value = close_object(frame)?;
                        if let Some(done) = attach(&mut stack, value)? {
                            root = Some(done);
                        }
                    }
                    Some(0x22) => {
                        let key = scanner.string()?;
                        if let Some(Frame::Object { pending, .. }) = stack.last_mut() {
                            *pending = Some(key);
                        }
                    }
                    _ => return Err(CardError::NotJson),
                }
            }
            Next::Colon => {
                scanner.skip_ws();
                if scanner.peek() != Some(0x3A) {
                    return Err(CardError::NotJson);
                }
                scanner.at += 1;
                // Read the member value in the next turn.
                // (The state derives from pending staying
                // set; force the value path by hand.)
                let value = read_value(&mut scanner, &mut stack)?;
                if let Some(done) = value {
                    if let Some(done) = attach(&mut stack, done)? {
                        root = Some(done);
                    }
                }
            }
            Next::Value => {
                let value = read_value(&mut scanner, &mut stack)?;
                if let Some(done) = value {
                    if let Some(done) = attach(&mut stack, done)? {
                        root = Some(done);
                    }
                }
            }
            Next::CommaOrEnd => {
                scanner.skip_ws();
                match scanner.peek() {
                    Some(0x2C) => {
                        scanner.at += 1;
                        match stack.last_mut() {
                            Some(Frame::Object {
                                entries: _,
                                pending,
                            }) => {
                                scanner.skip_ws();
                                if scanner.peek() != Some(0x22) {
                                    return Err(CardError::NotJson);
                                }
                                let key = scanner.string()?;
                                *pending = Some(key);
                            }
                            Some(Frame::Array(_)) => {
                                let value = read_value(&mut scanner, &mut stack)?;
                                if let Some(done) = value {
                                    if let Some(done) = attach(&mut stack, done)? {
                                        root = Some(done);
                                    }
                                }
                            }
                            None => return Err(CardError::NotJson),
                        }
                    }
                    Some(0x5D) => {
                        // An array's close: only valid atop
                        // an array frame.
                        match stack.last() {
                            Some(Frame::Array(_)) => {
                                scanner.at += 1;
                                let frame = stack.pop().ok_or(CardError::NotJson)?;
                                let value = close_object(frame)?;
                                if let Some(done) = attach(&mut stack, value)? {
                                    root = Some(done);
                                }
                            }
                            _ => return Err(CardError::NotJson),
                        }
                    }
                    Some(0x7D) => match stack.last() {
                        Some(Frame::Object { .. }) => {
                            scanner.at += 1;
                            let frame = stack.pop().ok_or(CardError::NotJson)?;
                            let value = close_object(frame)?;
                            if let Some(done) = attach(&mut stack, value)? {
                                root = Some(done);
                            }
                        }
                        _ => return Err(CardError::NotJson),
                    },
                    _ => return Err(CardError::NotJson),
                }
            }
        }
    }
    scanner.skip_ws();
    if scanner.at != units.len() {
        return Err(CardError::NotJson);
    }
    root.ok_or(CardError::NotJson)
}

/// Read one value at the scanner. Scalars return
/// Some; an opening bracket pushes a frame and
/// returns None, except an immediately closed empty
/// container, which completes at once (with the
/// duplicate check trivially passing).
fn read_value(scanner: &mut Scanner, stack: &mut Vec<Frame>) -> Result<Option<Value>, CardError> {
    scanner.skip_ws();
    match scanner.peek() {
        Some(0x7B) => {
            if stack.len() + 1 > DEPTH_LIMIT {
                return Err(CardError::NotJson);
            }
            scanner.at += 1;
            scanner.skip_ws();
            if scanner.peek() == Some(0x7D) {
                scanner.at += 1;
                return Ok(Some(Value::Object(Vec::new())));
            }
            stack.push(Frame::Object {
                entries: Vec::new(),
                pending: None,
            });
            Ok(None)
        }
        Some(0x5B) => {
            if stack.len() + 1 > DEPTH_LIMIT {
                return Err(CardError::NotJson);
            }
            scanner.at += 1;
            scanner.skip_ws();
            if scanner.peek() == Some(0x5D) {
                scanner.at += 1;
                return Ok(Some(Value::Array(Vec::new())));
            }
            stack.push(Frame::Array(Vec::new()));
            Ok(None)
        }
        Some(0x22) => Ok(Some(Value::Str(scanner.string()?))),
        Some(0x74) => {
            scanner.literal("true")?;
            Ok(Some(Value::Bool(true)))
        }
        Some(0x66) => {
            scanner.literal("false")?;
            Ok(Some(Value::Bool(false)))
        }
        Some(0x6E) => {
            scanner.literal("null")?;
            Ok(Some(Value::Null))
        }
        Some(0x4E) => {
            scanner.literal("NaN")?;
            Ok(Some(Value::Float(f64::NAN)))
        }
        Some(0x49) => {
            scanner.literal("Infinity")?;
            Ok(Some(Value::Float(f64::INFINITY)))
        }
        Some(0x2D) => {
            if unit_at(scanner, 1) == Some(0x49) {
                scanner.at += 1;
                scanner.literal("Infinity")?;
                Ok(Some(Value::Float(f64::NEG_INFINITY)))
            } else {
                Ok(Some(scanner.number()?))
            }
        }
        Some(0x30..=0x39) => Ok(Some(scanner.number()?)),
        _ => Err(CardError::NotJson),
    }
}

fn unit_at(scanner: &Scanner, offset: usize) -> Option<u16> {
    scanner.units.get(scanner.at + offset).copied()
}
