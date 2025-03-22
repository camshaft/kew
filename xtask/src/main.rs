use clap::Parser;
use xshell::{cmd, Shell};

mod compile;

#[derive(Debug, Parser)]
enum Arguments {
    Test(Test),
    Compile(compile::Compile),
}

fn main() {
    let sh = Shell::new().unwrap();
    Arguments::parse().run(&sh);
}

impl Arguments {
    fn run(&self, sh: &Shell) {
        match self {
            Arguments::Test(t) => t.run(sh),
            Arguments::Compile(t) => t.run(sh),
        }
    }
}

#[derive(Debug, Parser)]
struct Test;

impl Test {
    fn run(&self, sh: &Shell) {
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
}
