use anyhow::{anyhow, Result};
use std::process::Command;

const SSH_USER: &str = "manager";

pub fn execute_ssh(host: &str, command: &str) -> Result<String> {
    let target = format!("{SSH_USER}@{host}");

    let remote_cmd = command
        .replace("\r\n", "\n")
        .replace('\r', "")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" && ");

    let mut cmd = Command::new("ssh");
    cmd.arg("-o").arg("BatchMode=yes")
       .arg("-o").arg("StrictHostKeyChecking=no")
       .arg(&target)
       .arg(&remote_cmd);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }

    let output = cmd.output()
        .map_err(|e| anyhow!("SSH Execution Failed: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(format!("{stdout}{stderr}"))
}
