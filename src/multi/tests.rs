use super::{length_data, length_value, many0, many1};
use crate::input::Streaming;
use crate::Parser;
use crate::{
    bytes::tag,
    character::digit1 as digit,
    error::{ErrMode, ErrorKind, Needed, ParseError},
    lib::std::str::{self, FromStr},
    number::{be_u16, be_u8},
    IResult,
};
#[cfg(feature = "alloc")]
use crate::{
    lib::std::vec::Vec,
    multi::{
        count, fold_many0, fold_many1, fold_many_m_n, length_count, many_m_n, many_till0,
        separated_list0, separated_list1,
    },
};

#[test]
#[cfg(feature = "alloc")]
fn separated_list0_test() {
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list0(tag(","), tag("abcd"))(i)
    }
    fn multi_empty(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list0(tag(","), tag(""))(i)
    }
    fn empty_sep(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list0(tag(""), tag("abc"))(i)
    }
    fn multi_longsep(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list0(tag(".."), tag("abcd"))(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b",,abc"[..];
    let e = &b"abcd,abcd,ef"[..];
    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];
    let i = &b"abcabc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(Streaming(a)), Ok((Streaming(&b"ef"[..]), res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"ef"[..]), res2)));
    assert_eq!(
        multi(Streaming(c)),
        Ok((Streaming(&b"azerty"[..]), Vec::new()))
    );
    let res3 = vec![&b""[..], &b""[..], &b""[..]];
    assert_eq!(
        multi_empty(Streaming(d)),
        Ok((Streaming(&b"abc"[..]), res3))
    );
    let i_err_pos = &i[3..];
    assert_eq!(
        empty_sep(Streaming(i)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(i_err_pos),
            ErrorKind::SeparatedList
        )))
    );
    let res4 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(e)), Ok((Streaming(&b",ef"[..]), res4)));

    assert_eq!(
        multi(Streaming(f)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi_longsep(Streaming(g)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi(Streaming(h)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn separated_list1_test() {
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list1(tag(","), tag("abcd"))(i)
    }
    fn multi_longsep(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        separated_list1(tag(".."), tag("abcd"))(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b"abcd,abcd,ef"[..];

    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(Streaming(a)), Ok((Streaming(&b"ef"[..]), res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"ef"[..]), res2)));
    assert_eq!(
        multi(Streaming(c)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(c),
            ErrorKind::Tag
        )))
    );
    let res3 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(d)), Ok((Streaming(&b",ef"[..]), res3)));

    assert_eq!(
        multi(Streaming(f)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi_longsep(Streaming(g)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi(Streaming(h)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn many0_test() {
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        many0(tag("abcd"))(i)
    }
    fn multi_empty(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        many0(tag(""))(i)
    }

    assert_eq!(
        multi(Streaming(&b"abcdef"[..])),
        Ok((Streaming(&b"ef"[..]), vec![&b"abcd"[..]]))
    );
    assert_eq!(
        multi(Streaming(&b"abcdabcdefgh"[..])),
        Ok((Streaming(&b"efgh"[..]), vec![&b"abcd"[..], &b"abcd"[..]]))
    );
    assert_eq!(
        multi(Streaming(&b"azerty"[..])),
        Ok((Streaming(&b"azerty"[..]), Vec::new()))
    );
    assert_eq!(
        multi(Streaming(&b"abcdab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        multi(Streaming(&b"abcd"[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi(Streaming(&b""[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi_empty(Streaming(&b"abcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"abcdef"[..]),
            ErrorKind::Many0
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn many1_test() {
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        many1(tag("abcd"))(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(Streaming(a)), Ok((Streaming(&b"ef"[..]), res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"efgh"[..]), res2)));
    assert_eq!(
        multi(Streaming(c)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(c),
            ErrorKind::Tag
        )))
    );
    assert_eq!(
        multi(Streaming(d)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn many_till_test() {
    #[allow(clippy::type_complexity)]
    fn multi(i: &[u8]) -> IResult<&[u8], (Vec<&[u8]>, &[u8])> {
        many_till0(tag("abcd"), tag("efgh"))(i)
    }

    let a = b"abcdabcdefghabcd";
    let b = b"efghabcd";
    let c = b"azerty";

    let res_a = (vec![&b"abcd"[..], &b"abcd"[..]], &b"efgh"[..]);
    let res_b: (Vec<&[u8]>, &[u8]) = (Vec::new(), &b"efgh"[..]);
    assert_eq!(multi(&a[..]), Ok((&b"abcd"[..], res_a)));
    assert_eq!(multi(&b[..]), Ok((&b"abcd"[..], res_b)));
    assert_eq!(
        multi(&c[..]),
        Err(ErrMode::Backtrack(error_node_position!(
            &c[..],
            ErrorKind::ManyTill,
            error_position!(&c[..], ErrorKind::Tag)
        )))
    );
}

#[test]
#[cfg(feature = "std")]
fn infinite_many() {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
        println!("input: {:?}", input);
        Err(ErrMode::Backtrack(error_position!(input, ErrorKind::Tag)))
    }

    // should not go into an infinite loop
    fn multi0(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
        many0(tst)(i)
    }
    let a = &b"abcdef"[..];
    assert_eq!(multi0(a), Ok((a, Vec::new())));

    fn multi1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
        many1(tst)(i)
    }
    let a = &b"abcdef"[..];
    assert_eq!(
        multi1(a),
        Err(ErrMode::Backtrack(error_position!(a, ErrorKind::Tag)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn many_m_n_test() {
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        many_m_n(2, 4, tag("Abcd"))(i)
    }

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(
        multi(Streaming(a)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"ef"[..]),
            ErrorKind::Tag
        )))
    );
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"efgh"[..]), res1)));
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(c)), Ok((Streaming(&b"efgh"[..]), res2)));
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(d)), Ok((Streaming(&b"Abcdefgh"[..]), res3)));
    assert_eq!(
        multi(Streaming(e)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn count_test() {
    const TIMES: usize = 2;
    fn cnt_2(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        count(tag("abc"), TIMES)(i)
    }

    assert_eq!(
        cnt_2(Streaming(&b"abcabcabcdef"[..])),
        Ok((Streaming(&b"abcdef"[..]), vec![&b"abc"[..], &b"abc"[..]]))
    );
    assert_eq!(
        cnt_2(Streaming(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt_2(Streaming(&b"abcab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt_2(Streaming(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxx"[..]),
            ErrorKind::Tag
        )))
    );
    assert_eq!(
        cnt_2(Streaming(&b"xxxabcabcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxxabcabcdef"[..]),
            ErrorKind::Tag
        )))
    );
    assert_eq!(
        cnt_2(Streaming(&b"abcxxxabcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxxabcdef"[..]),
            ErrorKind::Tag
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn count_zero() {
    const TIMES: usize = 0;
    fn counter_2(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
        count(tag("abc"), TIMES)(i)
    }

    let done = &b"abcabcabcdef"[..];
    let parsed_done = Vec::new();
    let rest = done;
    let incomplete_1 = &b"ab"[..];
    let parsed_incompl_1 = Vec::new();
    let incomplete_2 = &b"abcab"[..];
    let parsed_incompl_2 = Vec::new();
    let error = &b"xxx"[..];
    let error_remain = &b"xxx"[..];
    let parsed_err = Vec::new();
    let error_1 = &b"xxxabcabcdef"[..];
    let parsed_err_1 = Vec::new();
    let error_1_remain = &b"xxxabcabcdef"[..];
    let error_2 = &b"abcxxxabcdef"[..];
    let parsed_err_2 = Vec::new();
    let error_2_remain = &b"abcxxxabcdef"[..];

    assert_eq!(counter_2(done), Ok((rest, parsed_done)));
    assert_eq!(
        counter_2(incomplete_1),
        Ok((incomplete_1, parsed_incompl_1))
    );
    assert_eq!(
        counter_2(incomplete_2),
        Ok((incomplete_2, parsed_incompl_2))
    );
    assert_eq!(counter_2(error), Ok((error_remain, parsed_err)));
    assert_eq!(counter_2(error_1), Ok((error_1_remain, parsed_err_1)));
    assert_eq!(counter_2(error_2), Ok((error_2_remain, parsed_err_2)));
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NilError;

impl<I> From<(I, ErrorKind)> for NilError {
    fn from(_: (I, ErrorKind)) -> Self {
        NilError
    }
}

impl<I> ParseError<I> for NilError {
    fn from_error_kind(_: I, _: ErrorKind) -> NilError {
        NilError
    }
    fn append(self, _: I, _: ErrorKind) -> NilError {
        NilError
    }
}

#[test]
#[cfg(feature = "alloc")]
fn length_count_test() {
    fn number(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, u32> {
        digit
            .map_res(str::from_utf8)
            .map_res(FromStr::from_str)
            .parse_next(i)
    }

    fn cnt(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        length_count(number, tag("abc"))(i)
    }

    assert_eq!(
        cnt(Streaming(&b"2abcabcabcdef"[..])),
        Ok((Streaming(&b"abcdef"[..]), vec![&b"abc"[..], &b"abc"[..]]))
    );
    assert_eq!(
        cnt(Streaming(&b"2ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt(Streaming(&b"3abcab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt(Streaming(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxx"[..]),
            ErrorKind::Digit
        )))
    );
    assert_eq!(
        cnt(Streaming(&b"2abcxxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxx"[..]),
            ErrorKind::Tag
        )))
    );
}

#[test]
fn length_data_test() {
    fn number(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, u32> {
        digit
            .map_res(str::from_utf8)
            .map_res(FromStr::from_str)
            .parse_next(i)
    }

    fn take(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, &[u8]> {
        length_data(number)(i)
    }

    assert_eq!(
        take(Streaming(&b"6abcabcabcdef"[..])),
        Ok((Streaming(&b"abcdef"[..]), &b"abcabc"[..]))
    );
    assert_eq!(
        take(Streaming(&b"3ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        take(Streaming(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"xxx"[..]),
            ErrorKind::Digit
        )))
    );
    assert_eq!(
        take(Streaming(&b"2abcxxx"[..])),
        Ok((Streaming(&b"cxxx"[..]), &b"ab"[..]))
    );
}

#[test]
fn length_value_test() {
    fn length_value_1(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, u16> {
        length_value(be_u8, be_u16)(i)
    }
    fn length_value_2(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, (u8, u8)> {
        length_value(be_u8, (be_u8, be_u8))(i)
    }

    let i1 = [0, 5, 6];
    assert_eq!(
        length_value_1(Streaming(&i1)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b""[..]),
            ErrorKind::Complete
        )))
    );
    assert_eq!(
        length_value_2(Streaming(&i1)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b""[..]),
            ErrorKind::Complete
        )))
    );

    let i2 = [1, 5, 6, 3];
    assert_eq!(
        length_value_1(Streaming(&i2)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&i2[1..2]),
            ErrorKind::Complete
        )))
    );
    assert_eq!(
        length_value_2(Streaming(&i2)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&i2[1..2]),
            ErrorKind::Complete
        )))
    );

    let i3 = [2, 5, 6, 3, 4, 5, 7];
    assert_eq!(
        length_value_1(Streaming(&i3)),
        Ok((Streaming(&i3[3..]), 1286))
    );
    assert_eq!(
        length_value_2(Streaming(&i3)),
        Ok((Streaming(&i3[3..]), (5, 6)))
    );

    let i4 = [3, 5, 6, 3, 4, 5];
    assert_eq!(
        length_value_1(Streaming(&i4)),
        Ok((Streaming(&i4[4..]), 1286))
    );
    assert_eq!(
        length_value_2(Streaming(&i4)),
        Ok((Streaming(&i4[4..]), (5, 6)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many0_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        fold_many0(tag("abcd"), Vec::new, fold_into_vec)(i)
    }
    fn multi_empty(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        fold_many0(tag(""), Vec::new, fold_into_vec)(i)
    }

    assert_eq!(
        multi(Streaming(&b"abcdef"[..])),
        Ok((Streaming(&b"ef"[..]), vec![&b"abcd"[..]]))
    );
    assert_eq!(
        multi(Streaming(&b"abcdabcdefgh"[..])),
        Ok((Streaming(&b"efgh"[..]), vec![&b"abcd"[..], &b"abcd"[..]]))
    );
    assert_eq!(
        multi(Streaming(&b"azerty"[..])),
        Ok((Streaming(&b"azerty"[..]), Vec::new()))
    );
    assert_eq!(
        multi(Streaming(&b"abcdab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        multi(Streaming(&b"abcd"[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi(Streaming(&b""[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi_empty(Streaming(&b"abcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"abcdef"[..]),
            ErrorKind::Many0
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many1_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        fold_many1(tag("abcd"), Vec::new, fold_into_vec)(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(Streaming(a)), Ok((Streaming(&b"ef"[..]), res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"efgh"[..]), res2)));
    assert_eq!(
        multi(Streaming(c)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(c),
            ErrorKind::Many1
        )))
    );
    assert_eq!(
        multi(Streaming(d)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many_m_n_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, Vec<&[u8]>> {
        fold_many_m_n(2, 4, tag("Abcd"), Vec::new, fold_into_vec)(i)
    }

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(
        multi(Streaming(a)),
        Err(ErrMode::Backtrack(error_position!(
            Streaming(&b"ef"[..]),
            ErrorKind::Tag
        )))
    );
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(b)), Ok((Streaming(&b"efgh"[..]), res1)));
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(c)), Ok((Streaming(&b"efgh"[..]), res2)));
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(Streaming(d)), Ok((Streaming(&b"Abcdefgh"[..]), res3)));
    assert_eq!(
        multi(Streaming(e)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
fn many0_count_test() {
    fn count0_nums(i: &[u8]) -> IResult<&[u8], usize> {
        many0((digit, tag(",")))(i)
    }

    assert_eq!(count0_nums(&b"123,junk"[..]), Ok((&b"junk"[..], 1)));

    assert_eq!(count0_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

    assert_eq!(
        count0_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
        Ok((&b"junk"[..], 10))
    );

    assert_eq!(count0_nums(&b"hello"[..]), Ok((&b"hello"[..], 0)));
}

#[test]
fn many1_count_test() {
    fn count1_nums(i: &[u8]) -> IResult<&[u8], usize> {
        many1((digit, tag(",")))(i)
    }

    assert_eq!(count1_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

    assert_eq!(
        count1_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
        Ok((&b"junk"[..], 10))
    );

    assert_eq!(
        count1_nums(&b"hello"[..]),
        Err(ErrMode::Backtrack(error_position!(
            &b"hello"[..],
            ErrorKind::Digit
        )))
    );
}
