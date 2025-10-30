use automate_refuse_de_nier::tools::parsing::parse_grammar_file;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestTempDir {
    path: PathBuf,
}

impl TestTempDir {
    fn new(prefix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let dir = env::temp_dir().join(format!("{}_{}", prefix, unique));
        fs::create_dir(&dir).expect("create temp dir");
        Self { path: dir }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestTempDir {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[test]
fn parse_invalid_line_reports_format_error() {
    let temp_dir = TestTempDir::new("ardn_badformat");
    let grammar_path = temp_dir.path().join("bad_format.gmr");

    {
        let mut file = File::create(&grammar_path).expect("create temp grammar");
        writeln!(file, "Invalid grammar line without separator").expect("write invalid line");
    }

    let err = parse_grammar_file(&grammar_path).expect_err("expected parse failure");
    assert!(
        err.contains("unrecognized line"),
        "unexpected error message: {err}"
    );
}

#[test]
fn parse_directory_path_surfaces_io_error() {
    let temp_dir = TestTempDir::new("ardn_dir");
    let err = parse_grammar_file(temp_dir.path()).expect_err("directories are not valid grammars");
    assert!(
        err.contains("Failed to read grammar file"),
        "unexpected error message: {err}"
    );
}

#[cfg(unix)]
#[test]
fn parse_unreadable_file_surfaces_permission_error() {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TestTempDir::new("ardn_perm");
    let grammar_path = temp_dir.path().join("restricted.gmr");

    {
        let mut file = File::create(&grammar_path).expect("create temp grammar");
        writeln!(file, "a, Attack").expect("write minimal mapping");
    }

    fs::set_permissions(&grammar_path, Permissions::from_mode(0o000))
        .expect("restrict permissions");

    if File::open(&grammar_path).is_ok() {
        eprintln!(
            "Skipping permission error test: filesystem still allows reading restricted file."
        );
        let _ = fs::set_permissions(&grammar_path, Permissions::from_mode(0o600));
        return;
    }

    let err = parse_grammar_file(&grammar_path).expect_err("expected permission failure");
    assert!(
        err.contains("Failed to read grammar file"),
        "unexpected error message: {err}"
    );

    // Restore permissions so the temp file can be cleaned up without panicking.
    let _ = fs::set_permissions(&grammar_path, Permissions::from_mode(0o600));
}
