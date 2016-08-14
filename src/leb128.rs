use num::{Num, NumCast, ToPrimitive};

use std::ops::{BitAnd, BitOr, Shr, Shl};
use std::default::Default;

pub fn gen_compress<T:Num+
                                        NumCast+ToPrimitive+Copy+
                                        BitAnd<T, Output=T>+
                                        Shr<T, Output=T>>
                                        (v: T, output: &mut Vec<u8>) {
    let mut x = v;
    let _0: T = NumCast::from(0usize).unwrap();
    let _v7f = NumCast::from(0x7f_u8).unwrap();
    let _v7 = NumCast::from(7_u8).unwrap();
    loop {
        let op: T = x & _v7f;
        let mut sub_res: u8 = op.to_u8().unwrap();
        x = x >> _v7;
        if x != _0 {
            sub_res |= 0x80;
        }
        output.push(sub_res);
        if x == _0 {
            break;
        }
    }
}

pub fn gen_decompress<T:Num+
                                        NumCast+ToPrimitive+Copy+Default+
                                        BitAnd<T, Output=T>+BitOr<T, Output=T>+
                                        Shl<T, Output=T>+>(values: &Vec<u8>) ->T{
    let mut result: T = T::default();
    let mut bytes = 0;
    let _0: T = NumCast::from(0usize).unwrap();
    let _v80 = NumCast::from(0x80).unwrap();

    for v in values {
        let readed: T = NumCast::from(*v).unwrap();
        let sr: T = readed & NumCast::from(0x7f).unwrap();
        result = result | (sr << NumCast::from(7 * bytes).unwrap());
        bytes += 1;
        if readed & _v80 == _0 {
            break;
        }
    }
    return result;
}

pub fn compress(v: u64, output: &mut Vec<u8>) {
    gen_compress(v, output);
}

pub fn decompress(values: &Vec<u8>) -> u64 {
    return gen_decompress(values);
}
