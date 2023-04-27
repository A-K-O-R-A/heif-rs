// use std::str::Utf8Error;

use nom::{bytes::complete::take, IResult};

pub fn take_4b_str(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, data) = take(4usize)(i)?;
    match std::str::from_utf8(data) {
        Ok(s) => Ok((i, s)),
        // TODO, handle this error better (unrecoverable)
        Err(e) => panic!("{e}, {:x?}", &i[0..4]),
    }
}
