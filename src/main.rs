#![windows_subsystem = "windows"]

use std::{net::TcpStream, path::PathBuf, fs::{OpenOptions}};
use clipboard_win::{ set_clipboard, formats, Unicode, Clipboard };
use anyhow::{anyhow};

use msgbox::IconType;
use ssh2::Session;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long, value_parser)]
    server_url: String,

    #[clap(short = 'u', long, value_parser)]
    user: String,

    #[clap(short = 'k', long, value_parser)]
    private_key: PathBuf,

    #[clap(short = 'p', long, value_parser)]
    private_key_passphrase: Option<String>,

    #[clap(short = 't', long, value_parser)]
    target_directory: PathBuf,
    
    /// Image source path.
    /// When used with getgreenshot use "{0}"
    #[clap(short = 'i', long, value_parser)] // i for image
    source: PathBuf,

    #[clap(long, value_parser)]
    clipboard_url: Option<String>,
}

fn main() {
    let result = real_main();
    if let Err(err) = result {
        println!("Screenshot upload filed:\n{:?}", err);
        let _ = msgbox::create("Upload failed", &format!("Screenshot upload filed:\n{:?}", err), IconType::Error);
    }
}

// Attention: UriToClipboard should be false!
fn real_main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    if !args.source.is_file() {
        anyhow::bail!("Source image is not a file!")
    }

    let file_name = args.source.file_name().map_or("unknown_image_name".to_string(), |name| name.to_string_lossy().to_string());
    let mut file = OpenOptions::new()
        .read(true)
        .open(&args.source)?;

    let mut sess = Session::new()?;
    let tcp = TcpStream::connect(&args.server_url)?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    
    let available_methods = sess.auth_methods(&args.user)?.split(",").collect::<Vec<_>>();
    if !available_methods.contains(&"publickey") {
        anyhow::bail!("Public key authentication not enabled!")
    }

    sess.userauth_pubkey_file(&args.user, None, &args.private_key, args.private_key_passphrase.as_deref())?;

    // TODO: Detect if the file already exists.
    //       If so append a number until the file does not exists any more.

    let file_size = file.metadata()?.len();
    let mut file_remote = sess.scp_send(&args.target_directory.join(&file_name), 0o644, file_size, None)?;
    std::io::copy(&mut file, &mut file_remote)?;
    file_remote.send_eof()?;
    file_remote.wait_eof()?;
    file_remote.close()?;
    file_remote.wait_close()?;

    println!("File uploaded successfully.");
    if let Some(url) = &args.clipboard_url {
        let url = url.replace("[file_name]", &file_name);
        
        let _clip = Clipboard::new_attempts(10)
        .map_err(|err| anyhow!("failed to open clipboard: {:#?}", err))?;

        set_clipboard(formats::Unicode, &url)
            .map_err(|err| anyhow!("failed to copy URL to clipboard: {:#?}", err))?;
    }

    // TODO: Does not properly show...
    // Toast::new(Toast::POWERSHELL_APP_ID)
    //     .title("Screenshot uploaded")
    //     .text1("Screenshot successfully uploaded to ...")
    //     .sound(Some(Sound::Default))
    //     .duration(Duration::Short)
    //     .show()?;

    Ok(())
}
