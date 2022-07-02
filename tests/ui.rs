use test_util::Environment;

#[test]
fn self_install() {
    let env = Environment::new(env!("CARGO_BIN_EXE_aftman"));

    let output = env.run(&["self-install"]);

    insta::assert_snapshot!("self-install stdout", output.stdout);
    insta::assert_snapshot!("self-install stderr", output.stderr);
}
