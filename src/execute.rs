use crate::types;

use std::os::unix::io::*;
use std::process::{Child, Command, Stdio};

use futures::future;
use futures::future::join_all;
use futures::prelude::*;
use futures::{Future, Stream};
use nix;
use nix::unistd::Pid;
// use tokio_process::*;
// use tokio_process::{Child, CommandExt};

pub fn spawn_cmd(cmd: &types::Cmd, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Child {
    let mut cmd_proc = Command::new(&cmd.exec);

    if let Some(ref args) = cmd.args {
        // flatten whitespace if necessary
        let args = args
            .iter()
            .flat_map(|s| s.split_whitespace())
            .collect::<Vec<&str>>();

        cmd_proc.args(args);
    }

    if let Some(ref envs) = cmd.env {
        cmd_proc.envs(envs);
    }

    if let Some(stdin) = stdin {
        cmd_proc.stdin(stdin);
    }

    if let Some(stdout) = stdout {
        cmd_proc.stdout(stdout);
    }

    // cmd_proc.spawn_async().expect(&format!("Failed to spawn process {}", cmd.exec))
    cmd_proc
        .spawn()
        .expect(&format!("Failed to spawn process {}", cmd.exec))
}

pub fn interpret_config(config: &types::Config) {
    let num_cmds = config.depends.len();

    let mut stdin = Some(Stdio::inherit());

    let mut handles = Vec::new();

    for (i, cmd) in config.depends.iter().enumerate() {
        let stdout = if i == num_cmds - 1 && config.pipe.is_some() {
            Some(Stdio::piped())
        } else {
            None
        };

        let is_stdout_none = stdout.is_none();
        let child = spawn_cmd(&cmd, stdin, stdout);
        handles.push(child.id());

        stdin = if is_stdout_none {
            None
        } else {
            Some(Stdio::from(child.stdout.unwrap()))
        };
    }

    if let Some(ref cmd) = config.pipe {
        let child = spawn_cmd(&cmd, stdin, None);
        handles.push(child.id());
    }

    handles.iter().for_each(|id| {
        let pid = Pid::from_raw(*id as i32);
        nix::sys::wait::waitpid(pid, None).expect("Error awaiting process");
    });
}
