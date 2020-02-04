#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

    /// Make a new empty target words list.
impl KWIndex {
    pub fn new() -> Self {
       KWIndex{ Vec::new() }
    } 

    //Do checks for if it is a word. Then if it is then add it
   pub fn extend_from_text(mut self, target: &'a str) -> Self {
        self.0 += target;
        self
   }
// hi
    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut matches = 0;
        for &word in &self {
            if &word == keyword {
                matches++;
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
    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
