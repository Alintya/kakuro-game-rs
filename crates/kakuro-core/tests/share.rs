use kakuro_core::{PuzzleSpec, ShareCode, generate};

#[test]
fn code_round_trips_to_the_same_puzzle() {
    for spec in [
        PuzzleSpec::Beginner,
        PuzzleSpec::Custom { rows: 4, cols: 4 },
        PuzzleSpec::Custom { rows: 8, cols: 10 },
    ] {
        let puzzle = generate(spec, 1).unwrap();
        let code = ShareCode::parse(&puzzle.code()).unwrap();
        assert_eq!(code, ShareCode::of(&puzzle), "{}", puzzle.code());
        assert_eq!(code.open().unwrap(), puzzle, "{}", puzzle.code());
    }
}

#[test]
fn display_is_stable() {
    let custom = ShareCode {
        spec: PuzzleSpec::Custom { rows: 8, cols: 10 },
        seed: 17,
        check: 0x0c2e,
    };
    assert_eq!(custom.to_string(), "8x10-17-0c2e");
    let beginner = ShareCode {
        spec: PuzzleSpec::Beginner,
        seed: 0,
        check: 0,
    };
    assert_eq!(beginner.to_string(), "beginner-0-0000");
    // A square custom size stays custom rather than turning into a preset.
    let square = ShareCode {
        spec: PuzzleSpec::Custom { rows: 6, cols: 6 },
        ..beginner
    };
    assert_eq!(ShareCode::parse(&square.to_string()).unwrap(), square);
}

#[test]
fn parse_tolerates_case_and_whitespace() {
    assert_eq!(
        ShareCode::parse("  EXPERT-7-00AB\n").unwrap(),
        ShareCode::parse("expert-7-00ab").unwrap()
    );
}

#[test]
fn rejects_malformed_codes() {
    for code in [
        "",
        "expert",
        "expert-7",
        "expert-7-zzzz",
        "expert-7-00000",
        "expert-7-+abc",
        "expert--7-0000",
        "hard-7-0000",
        "3x3-7-0000",
        "expert-4294967296-0000",
        "expert-7-0000-extra",
        "+5x6-1-0000",
        "05x06-1-0000",
        "5x06-1-0000",
        " expert -7-0000",
        "expert-+7-0000",
        "expert-007-0000",
        "expert- 7-0000",
        "expert-7-0ab",
    ] {
        let err = ShareCode::parse(code).unwrap_err().to_string();
        assert!(err.starts_with("invalid input: "), "{code:?}: {err}");
    }
    assert_eq!(
        ShareCode::parse("3x3-7-0000").unwrap_err().to_string(),
        "invalid input: rows and cols must be 4..=15"
    );
}

#[test]
fn open_rejects_a_wrong_check() {
    let code = ShareCode::of(&generate(PuzzleSpec::Beginner, 1).unwrap());
    let wrong = ShareCode {
        check: code.check ^ 1,
        ..code
    };
    assert_eq!(
        wrong.open().unwrap_err().to_string(),
        "invalid input: code and puzzle don't match (typo, or a different app version)"
    );
}

#[test]
fn check_depends_on_the_grid() {
    let checks: Vec<u16> = (1..=5)
        .map(|seed| ShareCode::of(&generate(PuzzleSpec::Beginner, seed).unwrap()).check)
        .collect();
    let mut distinct = checks.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), checks.len(), "checks {checks:?}");
}

/// Codes shared between installs must keep opening the same puzzle: if this
/// changes, every code handed out so far stops working.
#[test]
fn known_code_is_stable() {
    assert_eq!(
        generate(PuzzleSpec::Expert, 7).unwrap().code(),
        "expert-7-678c"
    );
}
