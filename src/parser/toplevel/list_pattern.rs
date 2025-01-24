use std::str::pattern as stdpat;

#[derive(Clone)]
pub(super) enum ListPattern { 
    Unordered,
    Numbered,
    Alphabetical,
    NumberedRounded,
    AlphabeticalRounded
}
pub(super) struct ListPatternSearcher<'a> {
    haystack: &'a str,
    position: usize,
    searcher: ListPattern,
}

impl<'a> ListPatternSearcher<'a> {
    pub fn new(haystack: &'a str, pattern: ListPattern) -> Self {
        Self {
            haystack,
            position: 0,
            searcher: pattern
        }
    }

    fn next_unordered(&mut self) -> stdpat::SearchStep {
        let old_finger = self.position;
        let slice = unsafe { self.haystack.get_unchecked(old_finger..) };
        let mut iter = slice.chars();
        // let old_len = iter.iter.len();
        if let Some(ch) = iter.next() {
            // add byte offset of current character
            // without re-encoding as utf-8
            self.position += ch.len_utf8();
            if ch == '-' {
                if let Some(ch) = iter.next() {
                    self.position += ch.len_utf8();
                    if ch.is_whitespace() {
                        stdpat::SearchStep::Match(old_finger, self.position)
                    } else {
                        stdpat::SearchStep::Reject(old_finger, self.position)
                    }
                }
                else {
                    stdpat::SearchStep::Reject(old_finger, self.position)
                }
            } else {
                stdpat::SearchStep::Reject(old_finger, self.position)
            }
        } else {
            stdpat::SearchStep::Done
        }
    }

    fn next_numbered(&mut self) -> stdpat::SearchStep {
        let old_finger = self.position;
        let slice = unsafe { self.haystack.get_unchecked(old_finger..) };
        let mut iter = slice.chars();
        let Some(ch) = iter.next() else { return stdpat::SearchStep::Done };
        // add byte offset of current character
        // without re-encoding as utf-8
        self.position += ch.len_utf8();

        if ch.is_digit(10) {
            let mut peeker = iter.peekable();
            let Some(ch) = peeker.next() else { return stdpat::SearchStep::Reject(old_finger, self.position) };

            let mut ch = ch;
            self.position += ch.len_utf8();
            if ch.is_digit(10) {
                while peeker.peek().is_some_and(|ch| ch.is_digit(10)) {
                    if let Some(ch) = peeker.next() {
                        self.position += ch.len_utf8()
                    } else {
                        panic!("shouldn't be possible");
                    }
                }
                if let Some(nxt) = peeker.next() {
                    self.position += nxt.len_utf8();
                    ch = nxt;
                } else {
                    return stdpat::SearchStep::Reject(old_finger, self.position)
                }
            }

            // wtf? 
            
            if let ListPattern::NumberedRounded = self.searcher {
                let ')' = ch else { return stdpat::SearchStep::Reject(old_finger, self.position) };
            } else {
                let '.' = ch else { return stdpat::SearchStep::Reject(old_finger, self.position) };
            }
            let Some(ch) = peeker.next() else { return stdpat::SearchStep::Reject(old_finger, self.position) };

            self.position += ch.len_utf8();
            if ch.is_whitespace() {
                stdpat::SearchStep::Match(old_finger, self.position)
            } else {
                stdpat::SearchStep::Reject(old_finger, self.position)
            }
        } else {
            stdpat::SearchStep::Reject(old_finger, self.position)
        }
    }
    
    fn next_alphabetical(&mut self) -> stdpat::SearchStep {
        let old_finger = self.position;
        let slice = unsafe { self.haystack.get_unchecked(old_finger..) };
        let mut iter = slice.chars();
        let Some(ch) = iter.next() else { return stdpat::SearchStep::Done };
        // add byte offset of current character
        // without re-encoding as utf-8
        self.position += ch.len_utf8();

        if ch.is_alphabetic() {
            let mut peeker = iter.peekable();
            let Some(ch) = peeker.next() else { return stdpat::SearchStep::Reject(old_finger, self.position) };

            let mut ch = ch;
            self.position += ch.len_utf8();
            if ch.is_alphabetic() {
                while peeker.peek().is_some_and(|ch| ch.is_digit(10)) {
                    if let Some(ch) = peeker.next() {
                        self.position += ch.len_utf8()
                    } else {
                        panic!("shouldn't be possible");
                    }
                }
                if let Some(nxt) = peeker.next() {
                    self.position += nxt.len_utf8();
                    ch = nxt;
                } else {
                    return stdpat::SearchStep::Reject(old_finger, self.position)
                }
            }

            // wtf? 
            
            if let ListPattern::AlphabeticalRounded = self.searcher {
                let ')' = ch else { return stdpat::SearchStep::Reject(old_finger, self.position) };
            } else {
                let '.' = ch else { return stdpat::SearchStep::Reject(old_finger, self.position) };
            }
            let Some(ch) = peeker.next() else { return stdpat::SearchStep::Reject(old_finger, self.position) };

            self.position += ch.len_utf8();
            if ch.is_whitespace() {
                stdpat::SearchStep::Match(old_finger, self.position)
            } else {
                stdpat::SearchStep::Reject(old_finger, self.position)
            }
        } else {
            stdpat::SearchStep::Reject(old_finger, self.position)
        }
    }
}

unsafe impl<'a> stdpat::Searcher<'a> for ListPatternSearcher<'a> {
    fn haystack(&self) -> &'a str {
        self.haystack
    }

    fn next(&mut self) -> stdpat::SearchStep {
        match self.searcher {
            ListPattern::Unordered => self.next_unordered(),
            ListPattern::NumberedRounded |
            ListPattern::Numbered => self.next_numbered(),
            ListPattern::AlphabeticalRounded |
            ListPattern::Alphabetical => self.next_alphabetical(),
        }
    }
}

impl std::str::pattern::Pattern for ListPattern {
    type Searcher<'a> = ListPatternSearcher<'a>;

    // Required method
    fn into_searcher(self, haystack: &str) -> Self::Searcher<'_> {
            ListPatternSearcher::new(haystack, self)
    }
}


mod tests {
    use super::*;

    #[test]
    fn starts_with_unordered() {
        let result = "- bla bla bla".starts_with(ListPattern::Unordered);
        assert_eq!(result, true);
    }
    
    #[test]
    fn basic_numbered() {
        let result = "1. bla bla bla".starts_with(ListPattern::Numbered);
        assert_eq!(result, true);
    }

    #[test]
    fn get_marker_unordered() {
        let mut result = "- bla bla bla".split_inclusive(ListPattern::Unordered);
        assert_eq!(result.next(), Some("- "));
    }
    
    #[test]
    fn get_marker_single_char_numbered() {
        let mut result = "1. bla bla bla".split_inclusive(ListPattern::Numbered);
        assert_eq!(result.next(), Some("1. "));
    }
    
    #[test]
    fn get_marker_large_numbered() {
        let mut result = "4917. bla bla bla".split_inclusive(ListPattern::Numbered);
        assert_eq!(result.next(), Some("4917. "));
    }
}



