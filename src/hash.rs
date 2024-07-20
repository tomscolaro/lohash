
use std::vec;
use xxhash_rust::xxh3::xxh3_64;


/// Takes a String and returns Vec overlapping tokens of size K
/// Example:
/// 
/// shingle_string("Hello", 2) -> ["He", "el", "ll", "lo"]
/// 
pub fn shingle_string(input_string:String, k:usize)-> Vec<String>{ 
    let mut tokens = Vec::new();
    let len = input_string.len();

    if k > len {
        return tokens;
    }

    for i in 0..=(len - k) {
        tokens.push(input_string[i..i + k].to_string());
    }

    tokens
}

pub fn compute_hash(tokens: &Vec<String>, num_hashes:usize, num_buckets:usize, noise_multiplier: &Vec<u64>, noise_added:&Vec<u64>) -> Vec<f32> {
    let mut v: Vec<f32> = vec![999_999.0; num_hashes as usize]; //stand in for vector of inf 
  
    for i in 0..num_hashes {
        for tok in tokens.iter(){ //minimizing hash.. 
            // let mut hasher = DefaultHasher::new();
            // tok.hash(&mut hasher);
            let hash_val = hash_string(tok);
            

            if v[i] >  (( noise_multiplier[i] as f32 * ( hash_val) + noise_added[i] as f32 ) % (num_buckets as f32)) {
                v[i] =  ( noise_multiplier[i] as f32 *  (hash_val) + noise_added[i] as f32 ) % (num_buckets as f32)
            }
        }    
    }
    v
}

pub fn compare_hash(vec_a: Vec<f32>, vec_b: Vec<f32>, num_hashes:usize) -> f32 {
    let matching = vec_a.iter().zip(& vec_b).filter(|&(a, b)| a == b).count();    

    (matching as f32)/ (num_hashes as f32)

}


fn hash_string(input: &str) -> f32 {
    xxh3_64(input.as_bytes()) as f32
}