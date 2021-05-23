use magit::str_fmt;

#[test]
fn simple() {
    assert_eq!("a b", str_fmt!("{} {}", "a", "b"));
}

#[test]
fn with_named() {
    let thing = "user";
    assert_eq!(
        "[user] foobar goodbye",
        str_fmt!("[{doesnotmatter}] foobar {hello}", thing, "goodbye")
    );
    assert_eq!("[🎉user]", str_fmt!("[🎉{}]", thing));
}

#[test]
fn many() {
    assert_eq!("a1b2c3xxx321zzz", str_fmt!("a{}b{}c{n}xxx{yyy}zzz", 1, 2, 3, 321));
}

#[test]
fn even_more() {
    assert_eq!(
        "a1b2c3xxx321zzz111222333444",
        str_fmt!("a{}b{}c{n}xxx{yyy}zzz{}{}{}{}", 1, 2, 3, 321, 111, 222, 333, 444)
    );
}

#[test]
fn str_fmt_macro() {
    assert_eq!("a1b2c3xxx321zzz", str_fmt!("a{}b{}c{n}xxx{yyy}zzz", 1, 2, 3, 321))
}
