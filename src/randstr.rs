use rand::distributions::{weighted::WeightedIndex, Distribution};
use rand::seq::SliceRandom;
use rand::Rng;

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

// 0 for no symbol
// 1 for ','
// 2 for '.'
const SYMBOL_WEIGHTS: &[usize] = &[100, 10, 5];

// Words with 3..=5 characters will be more common.
const WORD_SIZE_WEIGHTS: &[usize] = &[1, 2, 3, 3, 3, 2, 1];

/// Generate sentences with random ASCII characters.
pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let mut sentences = String::with_capacity(8 * 1024);

    let symbol_dist = WeightedIndex::new(SYMBOL_WEIGHTS).unwrap();
    let word_size_dist = WeightedIndex::new(WORD_SIZE_WEIGHTS).unwrap();

    let words = rng.gen_range(1000..2000);
    for _ in 0..words {
        if !sentences.is_empty() {
            sentences.push(' ');
        }

        // Word with random letters.
        let chars = word_size_dist.sample(&mut rng) + 1;
        for _ in 0..chars {
            sentences.push(char::from(*LETTERS.choose(&mut rng).unwrap()));
        }

        match symbol_dist.sample(&mut rng) {
            1 => sentences.push(','),
            2 => sentences.push('.'),
            _ => (),
        }
    }

    sentences
}
