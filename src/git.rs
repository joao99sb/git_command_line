use std::process::Command;

pub fn git_add() -> Result<(), ()> {
    let output = Command::new("git").arg("add").arg(".").output().unwrap();

    if output.status.success() {
        println!("Files successfully added to the stage!");
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        if stderr_str.contains("GIT_DISCOVERY_ACROSS_FILESYSTEM") {
            eprintln!("There is no initialized git repository");
            std::process::exit(1)
        } else {
            eprintln!("Unespected error in add: {stderr_str}");
            std::process::exit(1)
        }
    }

    Ok(())
}
pub fn git_commit(message: &str) -> Result<(), ()> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git commit successfully");
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        eprintln!("Unespected error in commit: {stderr_str}");
        std::process::exit(1)
    }
    Ok(())
}

pub fn git_push() -> Result<(), ()> {
    let output = Command::new("git").arg("push").output().unwrap();

    if output.status.success() {
        println!("Git pushed successfully");
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        eprintln!("Unespected error in commit: {stderr_str}");
        std::process::exit(1)
    }
    Ok(())
}
