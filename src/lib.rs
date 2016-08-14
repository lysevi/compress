extern crate num;

pub mod leb128;

#[cfg(test)]
mod tests {
    // use leb128;
    use super::*;
    #[test]
    fn leb128_test() {
        let values = vec![11, 2, 3, 4, 5, 123, 321, 0, 111, 222];
        for a in values {
            let mut out: Vec<u8> = Vec::with_capacity(10);

            leb128::compress(a, &mut out);
            let dec = leb128::decompress(&out);
            assert_eq!(dec, a);
        }
    }
}
