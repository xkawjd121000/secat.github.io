use file_manager::{copy, delete, list, move_file};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_copy_move_delete() -> std::io::Result<()> {
    let dir = tempdir()?;
    let src = dir.path().join("src.txt");
    let mut file = File::create(&src)?;
    writeln!(file, "hello")?;

    let copy_path = dir.path().join("copy.txt");
    copy(&src, &copy_path)?;
    assert!(copy_path.exists());

    let move_path = dir.path().join("moved.txt");
    move_file(&copy_path, &move_path)?;
    assert!(!copy_path.exists());
    assert!(move_path.exists());

    delete(&src)?;
    assert!(!src.exists());
    delete(&move_path)?;
    assert!(!move_path.exists());
    Ok(())
}

#[test]
fn test_list() -> std::io::Result<()> {
    let dir = tempdir()?;
    File::create(dir.path().join("a.txt"))?;
    File::create(dir.path().join("b.txt"))?;
    let mut entries = list(dir.path())?;
    entries.sort();
    assert_eq!(entries.len(), 2);
    let names: Vec<_> = entries
        .into_iter()
        .map(|p| p.file_name().unwrap().to_owned())
        .collect();
    assert!(names.contains(&"a.txt".into()));
    assert!(names.contains(&"b.txt".into()));
    Ok(())
}
