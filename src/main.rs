const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };
        
        let c0 = b0 >> 2;
        let c1 = ((b0 & 3) << 4) | (b1 >> 4);
        let c2 = ((b1 & 15) << 2) | (b2 >> 6);
        let c3 = b2 & 63;
        
        result.push(ALPHABET[c0] as char);
        result.push(ALPHABET[c1] as char);
        if i + 1 < data.len() {
            result.push(ALPHABET[c2] as char);
        } else {
            result.push('=');
        }
        if i + 2 < data.len() {
            result.push(ALPHABET[c3] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

fn main() {
    let input = b"Hello, World!";
    println!("Base64 Encoded: {}", encode(input));
}
