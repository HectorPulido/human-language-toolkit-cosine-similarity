mod tests;

use std::collections::HashMap;

pub fn compute_term_frequency(doc: &String) -> HashMap<String, u32> {
    let doc_words = doc.to_lowercase();
    let doc_words = doc_words.split(' ');
    let mut word_dict: HashMap<String, u32> = HashMap::new();

    for word in doc_words {
        word_dict.entry(word.to_string()).or_insert(0);
        word_dict.insert(word.to_string(), word_dict[word] + 1);
    }
    return word_dict;
}

pub fn normalized_term_frequency(term: String, doc: &String) -> f32 {
    let term = term.to_lowercase();
    let doc_words = doc.to_lowercase();
    let doc_words = doc_words.split(' ');
    let mut quantity = 0.0;
    let mut freq = 0.0;

    for word in doc_words {
        if word == term {
            freq += 1.0;
        }
        quantity += 1.0;
    }

    return freq / quantity;
}

pub fn compute_normalized_term_frequency(doc: &String) -> HashMap<String, f32> {
    let doc_words = doc.split(' ');
    let mut word_dict: HashMap<String, f32> = HashMap::new();

    for word in doc_words {
        let ntf = normalized_term_frequency(word.to_string(), &doc);
        word_dict.insert(word.to_string(), ntf);
    }

    return word_dict;
}

pub fn compute_normalized_term_frequency_batch(
    docs: &Vec<String>,
) -> HashMap<String, HashMap<String, f32>> {
    let mut data: HashMap<String, HashMap<String, f32>> = HashMap::new();

    for doc in docs {
        data.insert(doc.to_string(), compute_normalized_term_frequency(doc));
    }

    return data;
}

pub fn inverse_document_frequency(term: String, docs: &Vec<String>) -> f32 {
    let term = term.to_lowercase();
    let mut num_doc_with_this_term = 0;

    for doc in docs {
        let doc_words = doc.to_lowercase();
        let doc_words = doc_words.split(' ');

        for word in doc_words {
            if word == term {
                num_doc_with_this_term += 1;
                continue;
            }
        }
    }

    if num_doc_with_this_term == 0 {
        return 1.0;
    }

    let num_doc_with_this_term = num_doc_with_this_term as f32;
    let docs_len = docs.len() as f32;

    return 1.0 + (docs_len / num_doc_with_this_term).ln();
}

pub fn compute_inverse_document_frequency(docs: &Vec<String>) -> HashMap<String, f32> {
    let mut word_dict: HashMap<String, f32> = HashMap::new();

    for doc in docs {
        let doc_words = doc.to_lowercase();
        let doc_words = doc_words.split(' ');

        for word in doc_words {
            if word_dict.contains_key(word) {
                continue;
            }

            word_dict.insert(
                word.to_string(),
                inverse_document_frequency(word.to_string(), docs),
            );
        }
    }

    println!("{:?}", word_dict);

    return word_dict;
}

pub fn compute_term_frequency_inverse_document_frequency_all_docs(
    docs: &Vec<String>,
    query: String,
    idf_dict: HashMap<String, f32>,
    tf_dic: HashMap<String, HashMap<String, f32>>,
) -> Vec<f32> {
    let query_tokens = query.to_lowercase();
    let query_tokens: Vec<&str> = query_tokens.split(' ').collect();

    let mut tf_idf: Vec<f32> = Vec::new();

    for doc in docs {
        let doc_words = doc.to_lowercase();
        let doc_words = doc_words.split(' ');

        let tf = &tf_dic[doc];

        for word in doc_words {
            for token in &query_tokens {
                if word == *token {
                    tf_idf.push(idf_dict[word] * tf[word])
                }
            }
        }
    }
    return tf_idf;
}

pub fn compute_tf_idf_all_docs(
    docs: &Vec<String>,
    query: String,
    idf_dict: HashMap<String, f32>,
    tf_dic: HashMap<String, HashMap<String, f32>>,
) -> Vec<f32> {
    return compute_term_frequency_inverse_document_frequency_all_docs(
        docs, query, idf_dict, tf_dic,
    );
}
