#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

/// Make a new empty target words list.
impl<'a> KWIndex<'a> {
    pub fn new() -> Self {
        KWIndex(Vec::new())
    }

    //Do checks for if it is a word. Then if it is then add it
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        for mut s in target.split_whitespace() {
            let mut word = true;
            s = s.trim_matches(|c: char| !c.is_alphabetic());
            if !s.is_empty() {
                for c in s.chars() {
                    if !c.is_alphabetic() {
                        word = false;
                        break;
                    }
                }
            }
            if word && !s.is_empty() {
                self.0.push(s);
            }
        }
        self
    }
    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut matches = 0;
        for word in self.0.iter() {
            if word == &keyword {
                matches += 1;
            }
        }
        matches
    }

    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
