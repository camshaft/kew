use xshell::{cmd, Shell};

fn main() {
    let sh = Shell::new().unwrap();
    build(&sh);
}

fn build(sh: &Shell) {
    cmd!(sh, "npm install").run().unwrap();
    cmd!(sh, "npm run-script build").run().unwrap();

    cmd!(sh, "cargo test").run().unwrap();

    if cmd!(sh, "which mdbook").quiet().run().is_err() {
        cmd!(sh, "cargo install mdbook").run().unwrap();
    }

    let path = format!(
        "{}:{}",
        sh.current_dir().join("bin").display(),
        std::env::var("PATH").unwrap()
    );
    let _env = sh.push_env("PATH", path);
    cmd!(sh, "mdbook build").run().unwrap();
}
