#[test]
fn test_empty_struct() {
    let index = kwindex::KWIndex::new();

    assert_eq!(true, index.is_empty());
    assert_eq!(0, index.len());
}

#[test]
fn test_hello_world() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("Hello world.");

    assert_eq!(false, index.is_empty());
    assert_eq!(2, index.len());
    assert_eq!(1, index.count_matches("world"));
}

#[test]
fn test_punctuation_all() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("let's don't shouldn't won't");

    assert_eq!(true, index.is_empty());
    assert_eq!(0, index.len());
}

#[test]
fn test_punctuation_mixed() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("let's do and say we didn't.");

    assert_eq!(false, index.is_empty());
    assert_eq!(4, index.len());
    assert_eq!(1, index.count_matches("say"));
    assert_eq!(0, index.count_matches("didn't"));
}

#[test]
fn test_capitalization() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("The simple things are the best things in life.");

    assert_eq!(false, index.is_empty());
    assert_eq!(9, index.len());
    assert_eq!(1, index.count_matches("The"));
    assert_eq!(1, index.count_matches("the"));
}

#[test]
fn test_punctuation_variety() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("Should you? Well, I'm not sure if I \"should\", but I can.");

    assert_eq!(false, index.is_empty());
    assert_eq!(11, index.len());
    assert_eq!(1, index.count_matches("Well"));
    assert_eq!(1, index.count_matches("should"));
}

#[test]
fn incorrect_spacing() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("Hello sun,moon,and stars!");

    assert_eq!(false, index.is_empty());
    assert_eq!(2, index.len());
    assert_eq!(1, index.count_matches("Hello"));
    assert_eq!(1, index.count_matches("stars"));
}

#[test]
fn punctuation_before_word() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("'these ;are ?words");

    assert_eq!(false, index.is_empty());
    assert_eq!(3, index.len());
    assert_eq!(1, index.count_matches("these"));
    assert_eq!(1, index.count_matches("are"));
    assert_eq!(1, index.count_matches("words"));
}

#[test]
fn extra_space() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("   extra      spaces .");

    assert_eq!(false, index.is_empty());
    assert_eq!(2, index.len());
    assert_eq!(0, index.count_matches("."));
}

#[test]
fn duplicates() {
    let index = kwindex::KWIndex::new()
        .extend_from_text("a b c A b C b A");

    assert_eq!(false, index.is_empty());
    assert_eq!(8, index.len());
    assert_eq!(2, index.count_matches("A"));
    assert_eq!(1, index.count_matches("a"));
}
