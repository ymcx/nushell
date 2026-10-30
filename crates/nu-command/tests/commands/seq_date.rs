use nu_protocol::{ParseError, Type};
use nu_test_support::prelude::*;
use pretty_assertions::assert_matches;

#[test]
fn fails_on_datetime_input() -> Result {
    let err = test()
        .run("seq date --begin-date (date now)")
        .expect_parse_error()?;

    assert_matches!(err, ParseError::TypeMismatch(Type::String, Type::Date, _));
    Ok(())
}

#[test]
fn fails_when_increment_not_integer_or_duration() -> Result {
    let err = test()
        .run("seq date --begin-date 2020-01-01 --increment 1.1")
        .expect_parse_error()?;

    assert_matches!(
        err,
        ParseError::ExpectedWithStringMsg(expected, _)
            if expected == "one of a list of accepted shapes: [Duration, Int]"
    );
    Ok(())
}

#[test]
fn i64_min_reverse_overflow() -> Result {
    test()
        .run("seq date --begin-date 2020-01-01 --days -9223372036854775808 --reverse")
        .expect_shell_error()?;
    test()
        .run("seq date --begin-date 2020-01-01 --days -9223372036854775807 --reverse")
        .expect_shell_error()?;
    test()
        .run("seq date --begin-date 2020-01-01 --days 9223372036854775807 --reverse")
        .expect_shell_error()?;
    test()
        .run("seq date --begin-date 2020-01-01 --days -9223372036854775808")
        .expect_shell_error()?;
    test()
        .run("seq date --begin-date 2020-01-01 --days 9223372036854775807")
        .expect_shell_error()?;

    Ok(())
}
