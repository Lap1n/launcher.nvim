use std::io;

use nvim_oxi::{self as oxi};

use oxi::libuv::AsyncHandle;

use process_stream::{Process, ProcessExt, StreamExt};
use tokio::sync::mpsc::UnboundedSender;

use super::ConfigurationConfig;

type SenderMessage = UnboundedSender<(usize, String)>;

#[tokio::main]
pub async fn launch_process(
    config: ConfigurationConfig,
    handle: AsyncHandle,
    sender: SenderMessage,
) -> io::Result<()> {
    let mut commands = vec![config.run_type, config.program];
    commands.extend(config.args);
    let mut ls_home: Process = commands.into();
    let mut stream = ls_home.spawn_and_stream()?;

    let mut count = 0;

    while let Some(output) = stream.next().await {
        if output.is_exit() {
            sender.send((count, "[End]".to_string())).unwrap();
            break;
        } else {
            sender.send((count, format!("{output}"))).unwrap();
        }
        handle.send().unwrap();
        count += 1;
    }
    Ok(())
}
