#![cfg(test)]

use crate::assert_evals_to;
use crate::assert_llvm_evals_to;
use indoc::indoc;
use roc_std::RocStr;
use std::cmp::min;

const ROC_STR_MEM_SIZE: usize = core::mem::size_of::<RocStr>();

fn small_str(str: &str) -> [u8; ROC_STR_MEM_SIZE] {
    let mut bytes: [u8; ROC_STR_MEM_SIZE] = Default::default();

    let mut index: usize = 0;
    while index < ROC_STR_MEM_SIZE {
        bytes[index] = 0;
        index += 1;
    }

    let str_bytes = str.as_bytes();

    let output_len: usize = min(str_bytes.len(), ROC_STR_MEM_SIZE);
    index = 0;
    while index < output_len {
        bytes[index] = str_bytes[index];
        index += 1;
    }

    bytes[ROC_STR_MEM_SIZE - 1] = 0b1000_0000 ^ (output_len as u8);

    bytes
}

#[test]
fn str_split_bigger_delimiter_small_str() {
    assert_evals_to!(
        indoc!(
            r#"
                    List.len (Str.split "hello" "JJJJ there")
                "#
        ),
        1,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                    when List.first (Str.split "JJJ" "JJJJ there") is
                        Ok str ->
                            Str.countGraphemes str

                        _ ->
                            -1

                "#
        ),
        3,
        i64
    );
}

#[test]
fn str_split_str_concat_repeated() {
    assert_evals_to!(
        indoc!(
            r#"
                    when List.first (Str.split "JJJJJ" "JJJJ there") is
                        Ok str ->
                            str
                                |> Str.concat str
                                |> Str.concat str
                                |> Str.concat str
                                |> Str.concat str

                        _ ->
                            "Not Str!"

                "#
        ),
        "JJJJJJJJJJJJJJJJJJJJJJJJJ",
        &'static str
    );
}

#[test]
fn str_split_small_str_bigger_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    when
                        List.first
                            (Str.split "JJJ" "0123456789abcdefghi")
                    is
                        Ok str -> str
                        _ -> ""
                "#
        ),
        small_str("JJJ"),
        [u8; ROC_STR_MEM_SIZE]
    );
}

#[test]
fn str_split_big_str_small_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "01234567789abcdefghi?01234567789abcdefghi" "?"
                "#
        ),
        &["01234567789abcdefghi", "01234567789abcdefghi"],
        &'static [&'static str]
    );

    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "01234567789abcdefghi 3ch 01234567789abcdefghi" "3ch"
                "#
        ),
        &["01234567789abcdefghi ", " 01234567789abcdefghi"],
        &'static [&'static str]
    );
}

#[test]
fn str_split_small_str_small_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "J!J!J" "!"
                "#
        ),
        &[small_str("J"), small_str("J"), small_str("J")],
        &'static [[u8; ROC_STR_MEM_SIZE]]
    );
}

#[test]
fn str_split_bigger_delimiter_big_strs() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "string to split is shorter"
                        "than the delimiter which happens to be very very long"
                "#
        ),
        &["string to split is shorter"],
        &'static [&'static str]
    );
}

#[test]
fn str_split_empty_strs() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "" ""
                "#
        ),
        &[small_str("")],
        &'static [[u8; ROC_STR_MEM_SIZE]]
    )
}

#[test]
fn str_split_minimal_example() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "a," ","
                "#
        ),
        &[small_str("a"), small_str("")],
        &'static [[u8; ROC_STR_MEM_SIZE]]
    )
}

#[test]
fn str_split_small_str_big_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "1---- ---- ---- ---- ----2---- ---- ---- ---- ----"
                        "---- ---- ---- ---- ----"
                        |> List.len
                "#
        ),
        3,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "1---- ---- ---- ---- ----2---- ---- ---- ---- ----"
                        "---- ---- ---- ---- ----"
                "#
        ),
        &[small_str("1"), small_str("2"), small_str("")],
        &'static [[u8; ROC_STR_MEM_SIZE]]
    );
}

#[test]
fn str_split_small_str_20_char_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "3|-- -- -- -- -- -- |4|-- -- -- -- -- -- |"
                        "|-- -- -- -- -- -- |"
                "#
        ),
        &[small_str("3"), small_str("4"), small_str("")],
        &'static [[u8; ROC_STR_MEM_SIZE]]
    );
}

#[test]
fn str_concat_big_to_big() {
    assert_evals_to!(
            indoc!(
                r#"
                    Str.concat
                        "First string that is fairly long. Longer strings make for different errors. "
                        "Second string that is also fairly long. Two long strings test things that might not appear with short strings."
                "#
            ),
            "First string that is fairly long. Longer strings make for different errors. Second string that is also fairly long. Two long strings test things that might not appear with short strings.",
            &'static str
        );
}

#[test]
fn small_str_literal() {
    assert_evals_to!(
        "\"JJJJJJJJJJJJJJJ\"",
        [
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0b1000_1111
        ],
        [u8; 16]
    );
}

#[test]
fn small_str_zeroed_literal() {
    // Verifies that we zero out unused bytes in the string.
    // This is important so that string equality tests don't randomly
    // fail due to unused memory being there!
    assert_evals_to!(
        "\"J\"",
        [
            0x4a,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0b1000_0001
        ],
        [u8; 16]
    );
}

#[test]
fn small_str_concat_empty_first_arg() {
    assert_evals_to!(
        r#"Str.concat "" "JJJJJJJJJJJJJJJ""#,
        [
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0b1000_1111
        ],
        [u8; 16]
    );
}

#[test]
fn small_str_concat_empty_second_arg() {
    assert_evals_to!(
        r#"Str.concat "JJJJJJJJJJJJJJJ" """#,
        [
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0b1000_1111
        ],
        [u8; 16]
    );
}

#[test]
fn small_str_concat_small_to_big() {
    assert_evals_to!(
        r#"Str.concat "abc" " this is longer than 15 chars""#,
        "abc this is longer than 15 chars",
        &'static str
    );
}

#[test]
fn small_str_concat_small_to_small_staying_small() {
    assert_evals_to!(
        r#"Str.concat "J" "JJJJJJJJJJJJJJ""#,
        [
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0x4a,
            0b1000_1111
        ],
        [u8; 16]
    );
}

#[test]
fn small_str_concat_small_to_small_overflow_to_big() {
    assert_evals_to!(
        r#"Str.concat "abcdefghijklm" "nopqrstuvwxyz""#,
        "abcdefghijklmnopqrstuvwxyz",
        &'static str
    );
}

#[test]
fn str_concat_empty() {
    assert_evals_to!(r#"Str.concat "" """#, "", &'static str);
}

#[test]
fn small_str_is_empty() {
    assert_evals_to!(r#"Str.isEmpty "abc""#, false, bool);
}

#[test]
fn big_str_is_empty() {
    assert_evals_to!(
        r#"Str.isEmpty "this is more than 15 chars long""#,
        false,
        bool
    );
}

#[test]
fn empty_str_is_empty() {
    assert_evals_to!(r#"Str.isEmpty """#, true, bool);
}

#[test]
fn str_starts_with() {
    assert_evals_to!(r#"Str.startsWith "hello world" "hell""#, true, bool);
    assert_evals_to!(r#"Str.startsWith "hello world" """#, true, bool);
    assert_evals_to!(r#"Str.startsWith "nope" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.startsWith "hell" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.startsWith "" "hello world""#, false, bool);
}

#[test]
fn str_ends_with() {
    assert_evals_to!(r#"Str.endsWith "hello world" "world""#, true, bool);
    assert_evals_to!(r#"Str.endsWith "nope" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.endsWith "" "hello world""#, false, bool);
}

#[test]
fn str_count_graphemes_small_str() {
    assert_evals_to!(r#"Str.countGraphemes "å🤔""#, 2, usize);
}

#[test]
fn str_count_graphemes_three_js() {
    assert_evals_to!(r#"Str.countGraphemes "JJJ""#, 3, usize);
}

#[test]
fn str_count_graphemes_big_str() {
    assert_evals_to!(
        r#"Str.countGraphemes "6🤔å🤔e¥🤔çppkd🙃1jdal🦯asdfa∆ltråø˚waia8918.,🏅jjc""#,
        45,
        usize
    );
}

#[test]
fn str_starts_with_same_big_str() {
    assert_evals_to!(
        r#"Str.startsWith "123456789123456789" "123456789123456789""#,
        true,
        bool
    );
}

#[test]
fn str_starts_with_different_big_str() {
    assert_evals_to!(
        r#"Str.startsWith "12345678912345678910" "123456789123456789""#,
        true,
        bool
    );
}

#[test]
fn str_starts_with_same_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "1234""#, true, bool);
}

#[test]
fn str_starts_with_different_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "12""#, true, bool);
}
#[test]
fn str_starts_with_false_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "23""#, false, bool);
}

#[test]
fn str_from_int() {
    assert_evals_to!(
        r#"Str.fromInt 1234"#,
        roc_std::RocStr::from_slice("1234".as_bytes()),
        roc_std::RocStr
    );
    assert_evals_to!(
        r#"Str.fromInt 0"#,
        roc_std::RocStr::from_slice("0".as_bytes()),
        roc_std::RocStr
    );
    assert_evals_to!(
        r#"Str.fromInt -1"#,
        roc_std::RocStr::from_slice("-1".as_bytes()),
        roc_std::RocStr
    );

    let max = format!("{}", i64::MAX);
    assert_evals_to!(r#"Str.fromInt Num.maxInt"#, &max, &'static str);

    let min = format!("{}", i64::MIN);
    assert_evals_to!(r#"Str.fromInt Num.minInt"#, &min, &'static str);
}

#[test]
fn str_from_utf8_pass_single_ascii() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97 ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_many_ascii() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 98, 99, 0x7E ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("abc~".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_single_unicode() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 0xE2, 0x88, 0x86 ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("∆".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_many_unicode() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 0xE2, 0x88, 0x86, 0xC5, 0x93, 0xC2, 0xAC ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("∆œ¬".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_single_grapheme() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 0xF0, 0x9F, 0x92, 0x96 ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("💖".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_many_grapheme() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 0xF0, 0x9F, 0x92, 0x96, 0xF0, 0x9F, 0xA4, 0xA0, 0xF0, 0x9F, 0x9A, 0x80 ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("💖🤠🚀".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_pass_all() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 0xF0, 0x9F, 0x92, 0x96, 98, 0xE2, 0x88, 0x86 ] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("💖b∆".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_invalid_start_byte() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 98, 0x80, 99 ] is
                        Err (BadUtf8 InvalidStartByte byteIndex) ->
                            if byteIndex == 2 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_unexpected_end_of_sequence() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 98, 99, 0xC2 ] is
                        Err (BadUtf8 UnexpectedEndOfSequence byteIndex) ->
                            if byteIndex == 3 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_expected_continuation() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 98, 99, 0xC2, 0x00 ] is
                        Err (BadUtf8 ExpectedContinuation byteIndex) ->
                            if byteIndex == 3 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_overlong_encoding() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 0xF0, 0x80, 0x80, 0x80 ] is
                        Err (BadUtf8 OverlongEncoding byteIndex) ->
                            if byteIndex == 1 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_codepoint_too_large() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 0xF4, 0x90, 0x80, 0x80 ] is
                        Err (BadUtf8 CodepointTooLarge byteIndex) ->
                            if byteIndex == 1 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_from_utf8_fail_surrogate_half() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [ 97, 98, 0xED, 0xA0, 0x80 ] is
                        Err (BadUtf8 EncodesSurrogateHalf byteIndex) ->
                            if byteIndex == 2 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        roc_std::RocStr::from_slice("a".as_bytes()),
        roc_std::RocStr
    );
}

#[test]
fn str_equality() {
    assert_evals_to!(r#""a" == "a""#, true, bool);
    assert_evals_to!(
        r#""loremipsumdolarsitamet" == "loremipsumdolarsitamet""#,
        true,
        bool
    );
    assert_evals_to!(r#""a" != "b""#, true, bool);
    assert_evals_to!(r#""a" == "b""#, false, bool);
}

#[test]
fn str_clone() {
    use roc_std::RocStr;
    let long = RocStr::from_slice("loremipsumdolarsitamet".as_bytes());
    let short = RocStr::from_slice("x".as_bytes());
    let empty = RocStr::from_slice("".as_bytes());

    debug_assert_eq!(long.clone(), long);
    debug_assert_eq!(short.clone(), short);
    debug_assert_eq!(empty.clone(), empty);
}

#[test]
fn nested_recursive_literal() {
    assert_evals_to!(
        indoc!(
            r#"
                Expr : [ Add Expr Expr, Val I64, Var I64 ]

                expr : Expr
                expr = Add (Add (Val 3) (Val 1)) (Add (Val 1) (Var 1))

                printExpr : Expr -> Str
                printExpr = \e ->
                    when e is
                        Add a b ->
                            "Add ("
                                |> Str.concat (printExpr a)
                                |> Str.concat ") ("
                                |> Str.concat (printExpr b)
                                |> Str.concat ")"
                        Val v -> "Val " |> Str.concat (Str.fromInt v)
                        Var v -> "Var " |> Str.concat (Str.fromInt v)

                printExpr expr
                "#
        ),
        "Add (Add (Val 3) (Val 1)) (Add (Val 1) (Var 1))",
        &'static str
    );
}

#[test]
fn str_join_comma_small() {
    assert_evals_to!(
        r#"Str.joinWith ["1", "2"] ", " "#,
        RocStr::from("1, 2"),
        RocStr
    );
}

#[test]
fn str_join_comma_big() {
    assert_evals_to!(
        r#"Str.joinWith ["10000000", "2000000", "30000000"] ", " "#,
        RocStr::from("10000000, 2000000, 30000000"),
        RocStr
    );
}

#[test]
fn str_join_comma_single() {
    assert_evals_to!(r#"Str.joinWith ["1"] ", " "#, RocStr::from("1"), RocStr);
}

#[test]
fn str_from_float() {
    assert_evals_to!(r#"Str.fromFloat 3.14"#, RocStr::from("3.140000"), RocStr);
}

#[test]
fn str_to_bytes() {
    assert_evals_to!(r#"Str.toBytes "hello""#, &[104, 101, 108, 108, 111], &[u8]);
    assert_evals_to!(
        r#"Str.toBytes "this is a long string""#,
        &[
            116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103, 32, 115, 116, 114,
            105, 110, 103
        ],
        &[u8]
    );
}