use crate::types;

use std::process::{Command, Stdio};

use futures::{Future, Stream};
use tokio_process::{Child, CommandExt};

pub fn spawn_cmd(cmd: &types::Cmd, stdin: Option<Stdio>, stdout: Option<Stdio>) {
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

    let child = cmd_proc.spawn_async();

    // Make sure our child succeeded in spawning and process the result
    let future = child
        .expect("failed to spawn")
        .map(|status| println!("exit status: {}", status))
        .map_err(|e| panic!("failed to wait for exit: {}", e));

    // Send the future to the tokio runtime for execution
    tokio::run(future)
}

pub fn interpret_config(config: &types::Config) {

    if pipe
    let stdin = Some(Stdio::inherit());

    for cmd in &config.depends {
        // execute_command(&cmd, stdin.clone(), stdout.clone());
        spawn_cmd(&cmd, None, None);
    }
}
