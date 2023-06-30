use sha3::{Digest, Sha3_512};


pub fn sha3_512(s: String) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(s);
    let result = hasher.finalize();
    return format!("{:X}", result);
}
