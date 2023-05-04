// use std::str::Utf8Error;

use nom::{bytes::complete::take, IResult};

/// This takes 4 bytes and interprets as a utf8 encoded string
pub fn take_4b_str(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, data) = take(4usize)(i)?;
    match std::str::from_utf8(data) {
        Ok(s) => Ok((i, s)),
        // TODO, handle this error better (unrecoverable)
        Err(e) => panic!("{e}, {:x?}", &i[0..4]),
    }
}

/// This takes all remaining bytes and interprets
/// as a utf8 encoded string that is NUL-terminated
pub fn take_all_str(i: &[u8]) -> IResult<&[u8], &str> {
    match std::ffi::CStr::from_bytes_until_nul(i) {
        Ok(s) => match s.to_str() {
            Ok(s) => Ok((&[], s)),
            Err(e) => panic!("{e}, {:x?}", &i[0..4]),
        },
        // TODO, handle this error better (unrecoverable)
        Err(e) => panic!("{e}, {:x?}", &i[0..4]),
    }
}
