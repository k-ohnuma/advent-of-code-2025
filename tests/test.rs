use assert_cmd::Command;

#[test]
fn test1_1() {
    let input = include_str!("../input/day1/input.txt");
    let output = include_str!("../input/day1/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("1-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test1_2() {
    let input = include_str!("../input/day1/input.txt");
    let output = include_str!("../input/day1/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("1-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test2_1() {
    let input = include_str!("../input/day2/input.txt");
    let output = include_str!("../input/day2/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("2-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test2_2() {
    let input = include_str!("../input/day2/input.txt");
    let output = include_str!("../input/day2/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("2-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test3_1() {
    let input = include_str!("../input/day3/input.txt");
    let output = include_str!("../input/day3/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("3-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test3_2() {
    let input = include_str!("../input/day3/input.txt");
    let output = include_str!("../input/day3/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("3-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test4_1() {
    let input = include_str!("../input/day4/input.txt");
    let output = include_str!("../input/day4/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("4-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test4_2() {
    let input = include_str!("../input/day4/input.txt");
    let output = include_str!("../input/day4/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("4-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test5_1() {
    let input = include_str!("../input/day5/input.txt");
    let output = include_str!("../input/day5/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("5-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test5_2() {
    let input = include_str!("../input/day5/input.txt");
    let output = include_str!("../input/day5/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("5-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test6_1() {
    let input = include_str!("../input/day6/input.txt");
    let output = include_str!("../input/day6/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("6-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}
#[test]
fn test6_2() {
    let input = include_str!("../input/day6/input.txt");
    let output = include_str!("../input/day6/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("6-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}

#[test]
fn test7_1() {
    let input = include_str!("../input/day7/input.txt");
    let output = include_str!("../input/day7/out-1.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("7-1").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}
#[test]
fn test7_2() {
    let input = include_str!("../input/day7/input.txt");
    let output = include_str!("../input/day7/out-2.txt");
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("7-2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(output);
}
