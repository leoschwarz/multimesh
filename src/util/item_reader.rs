use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;
use std::str::Lines;

#[derive(Clone, Debug, Fail)]
pub enum ItemReaderError {
    #[fail(display = "Unexpected EOF")]
    UnexpectedEof,

    #[fail(display = "Parsing value failed: {}", _0)]
    Parse(String),
}

pub(crate) struct ItemReader<'s> {
    lines: Lines<'s>,
    line_buf: VecDeque<&'s str>,
}

impl<'s> ItemReader<'s> {
    pub(crate) fn new(data: &'s str) -> Self {
        ItemReader {
            lines: data.lines(),
            line_buf: VecDeque::new(),
        }
    }

    pub(crate) fn next_parse<T>(&mut self) -> Result<T, ItemReaderError>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        self.next_result().and_then(|s| {
            s.parse()
                .map_err(|e| ItemReaderError::Parse(format!("{}", e)))
        })
    }

    pub(crate) fn next_result(&mut self) -> Result<&'s str, ItemReaderError> {
        self.next().ok_or(ItemReaderError::UnexpectedEof)
    }

    /// Read all items until the end of the current line.
    pub(crate) fn next_until_eol(&mut self) -> Option<&'s str> {
        self.next_item(false)
    }

    pub(crate) fn next_parse_until_eol<T>(&mut self) -> Result<T, ItemReaderError>
        where
            T: FromStr,
            <T as FromStr>::Err: Display,
    {
        self.next_until_eol().ok_or(ItemReaderError::UnexpectedEof).and_then(|s| {
            s.parse()
                .map_err(|e| ItemReaderError::Parse(format!("{}", e)))
        })
    }

    fn next_item(&mut self, ignore_newline: bool) -> Option<&'s str> {
        let probe_buf = |line_buf: &mut VecDeque<&'s str>| {
            while let Some(item) = line_buf.remove(0) {
                if !item.trim().is_empty() {
                    return Some(item);
                }
            }
            None
        };

        if let Some(item) = probe_buf(&mut self.line_buf) {
            return Some(item);
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s+").unwrap();
        }

        while let Some(line) = self.lines.next() {
            if !line.starts_with("#") && !line.trim().is_empty() {
                self.line_buf.extend(RE.split(line));
                if let Some(item) = probe_buf(&mut self.line_buf) {
                    return Some(item);
                }
            }

            if !ignore_newline {
                return None;
            }
        }

        None
    }
}

impl<'s> Iterator for ItemReader<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_reader() {
        let source = " abc   xyz  -1\n\r\t \n  abc xyz";
        let mut reader = ItemReader::new(source);

        assert_eq!(reader.next(), "abc".into());
        assert_eq!(reader.next(), "xyz".into());
        assert_eq!(reader.next(), "-1".into());
        assert_eq!(reader.next(), "abc".into());
        assert_eq!(reader.next(), "xyz".into());
        assert_eq!(reader.next(), None);
    }
}
