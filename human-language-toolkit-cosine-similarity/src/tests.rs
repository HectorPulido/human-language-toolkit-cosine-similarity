#[cfg(test)]
use crate::*;

#[test]
fn test_compute_term_frequency() {
    let data = String::from("never stop learning, never stop");
    let term_freq = compute_term_frequency(&data);

    assert_eq!(term_freq.len(), 3);
    assert_eq!(term_freq["never"], 2);
    assert_eq!(term_freq["stop"], 2);
    assert_eq!(term_freq["learning,"], 1);
}

#[test]
fn test_normalized_term_frequency() {
    let data = String::from("never stop learning, never stop");
    let term = String::from("never");
    let term_norm_freq = normalized_term_frequency(term, &data);

    assert_eq!(term_norm_freq, 0.4);
}

#[test]
fn test_compute_normalized_term_frequency() {
    let data = String::from("never stop learning, never stop");
    let term_freq = compute_normalized_term_frequency(&data);

    assert_eq!(term_freq.len(), 3);
    assert_eq!(term_freq["never"], 2.0 / 5.0);
    assert_eq!(term_freq["stop"], 2.0 / 5.0);
    assert_eq!(term_freq["learning,"], 1.0 / 5.0);
}

#[test]
fn test_inverse_document_frequency() {
    let data = vec![
        String::from("I want to start learning to charge something in life"),
        String::from("reading something about life no one else knows"),
        String::from("Never stop learning"),
    ];
    let term = String::from("math");
    let idf = inverse_document_frequency(term, &data);
    assert_eq!(idf, 1.0);

    let term = String::from("Never");
    let idf = inverse_document_frequency(term, &data);
    assert_eq!(idf, 2.0986123);

    let term = String::from("I");
    let idf = inverse_document_frequency(term, &data);
    assert_eq!(idf, 2.0986123);
}

#[test]
fn test_compute_inverse_document_frequency() {
    let data = vec![
        String::from("I want to start learning to charge something in life"),
        String::from("reading something about life no one else knows"),
        String::from("Never stop learning"),
    ];
    let idf_hashmap = compute_inverse_document_frequency(&data);
    assert_eq!(idf_hashmap.len(), 17);
    assert_eq!(idf_hashmap["i"], 2.0986123);
    assert_eq!(idf_hashmap["want"], 2.0986123);
    assert_eq!(idf_hashmap["learning"], 1.4054651);
    assert_eq!(idf_hashmap["life"], 1.4054651);
}

#[test]
fn test_compute_term_frequency_inverse_document_frequency_all_docs() {
    let data = vec![
        String::from("I want to start learning to charge something in life"),
        String::from("reading something about life no one else knows"),
        String::from("Never stop learning"),
    ];
    let query = String::from("life learning");

    let idf = compute_inverse_document_frequency(&data);
    let tf = compute_normalized_term_frequency_batch(&data);
}
