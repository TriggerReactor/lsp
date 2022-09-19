use super::*;

use expect_test::{expect, Expect};

#[test]
fn cursor_first() {
    let cursor = Cursor::new("hello world");

    assert_eq!(cursor.first(), 'h');
}

#[test]
fn cursor_second() {
    let cursor = Cursor::new("hello world");

    assert_eq!(cursor.second(), 'e');
}

fn lex_assert_eq(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();

    expect.assert_eq(&actual)
}

#[test]
fn line_comment() {
    lex_assert_eq(
        r"// line",
        expect![[r#"
            Token { kind: LineComment, len: 7 }
        "#]],
    )
}

#[test]
fn block_comment() {
    lex_assert_eq(
        r"
/**/
/* block */
/** also block */
        ",
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 11 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 17 }
            Token { kind: Whitespace, len: 9 }
        "#]],
    )
}

#[test]
fn nested_block_comment() {
    lex_assert_eq(
        r#"/* /* block /* */ */ */"heya""#,
        expect![[r#"
            Token { kind: BlockComment { terminated: true }, len: 23 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 6 }
        "#]],
    )
}

#[test]
fn literal_flavors() {
    lex_assert_eq(
        r#"
123
456.789
444.
"heya"
        "#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Decimal { empty_exponent: false } }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Decimal { empty_exponent: true } }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 6 }
            Token { kind: Whitespace, len: 9 }
        "#]]
    )
}
