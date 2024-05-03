fn git_output(args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .output()
        .unwrap()
}

const CLANG_FORMAT: &str = r#"C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\Llvm\x64\bin\clang-format"#;
const FILE_EXTENSIONS: &[&str] = &[".c", ".h", ".cpp", ".hpp", ".mm", ".m"];

fn main() {
    let sha = String::from_utf8(git_output(&["rev-parse", "head"]).stdout).unwrap();
    let sha = sha.trim();
    println!("formatting commit {sha}");
    let files = String::from_utf8(
        git_output(&["diff-tree", "--no-commit-id", "--name-only", &sha, "-r"]).stdout,
    )
    .unwrap();
    for file in files.lines() {
        if !FILE_EXTENSIONS.iter().any(|ext| file.ends_with(ext)) {
            println!("ignoring {file}");
            continue;
        }
        println!("formatting {file}");
        std::process::Command::new(CLANG_FORMAT)
            .args(&["-i", file.trim()])
            .status()
            .unwrap();
        git_output(&["add", file.trim()]);
    }
    let changes =
        String::from_utf8(git_output(&["status", "-uno", "--porcelain=v1"]).stdout).unwrap();
    if !changes.is_empty() {
        println!("Committing changes");
        git_output(&["commit", "--amend", "--no-edit"]);
    } else {
        println!("No changes, leaving commit as is");
    }
}
