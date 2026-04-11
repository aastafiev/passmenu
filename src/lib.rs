use expanduser::expanduser;
use glob::glob;
use std::{collections::HashMap, env, error, io, io::prelude::*, process};

pub type BoxResult<T> = Result<T, Box<dyn error::Error>>;

fn run_rofi(rofi_args: &HashMap<String, Option<String>>, content: &Vec<String>) -> io::Result<process::Output> {
    let args: Vec<&String> = rofi_args
        .iter()
        .flat_map(|(arg, val)| match val {
            Some(v) => vec![arg, v],
            None => vec![arg],
        })
        .collect();
    let stdinp = content.join("\n");

    let mut child = process::Command::new("rofi")
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(stdinp.as_bytes()).expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;
    Ok(output)
}

pub fn passwords() -> BoxResult<Vec<String>> {
    let store_dir = expanduser(env::var("PASSWORD_STORE_DIR").unwrap_or("~/.password-store".into()))?;
    let mut res = vec![];
    for entry in glob(store_dir.join("**").join("*.gpg").to_str().unwrap())? {
        res.push(
            entry?
                .strip_prefix(&store_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".gpg", ""),
        );
    }
    Ok(res)
}

fn show_pass_details(pass_line: &str) -> BoxResult<Vec<String>> {
    let output = process::Command::new("pass")
        .args(&["show", pass_line])
        .output()
        .expect("Failed to run `pass show` command");

    let res = String::from_utf8(output.stdout)?
        .trim_end()
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()[1..]
        .to_vec();
    Ok(res)
}

fn copy_pass(pass_line: &str) -> io::Result<()> {
    process::Command::new("pass")
        .args(&["show", "-c", pass_line])
        .spawn()
        .expect("Failed to run `pass show -c` command");
    Ok(())
}

pub fn get_display_server() -> String {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return "wayland".to_string();
    }

    // 2. Check XDG_SESSION_TYPE
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        return session_type.to_lowercase(); // Usually "wayland" or "x11"
    }

    // 3. Fallback to DISPLAY (X11)
    if std::env::var("DISPLAY").is_ok() {
        return "x11".to_string();
    }

    "unknown".to_string()
}

fn copy(data: String, is_wayland: bool) -> io::Result<()> {
    let (program, args) = if is_wayland {
        ("wl-copy", vec![])
    } else {
        ("xsel", vec!["-b"])
    };

    let mut child = process::Command::new(program)
        .args(args)
        .stdin(process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(data.as_bytes()).expect("Failed to write to stdin");
    });
    child.wait()?;
    Ok(())
}

pub fn passmenu(
    rofi_args: HashMap<String, Option<String>>,
    pass_entries: &Vec<String>,
    is_wayland: bool,
) -> BoxResult<()> {
    let output = run_rofi(&rofi_args, &pass_entries)?;
    let mut pass_entity = String::new();
    if output.status.code() != Some(1) {
        pass_entity = String::from_utf8(output.stdout)?.trim_end().to_owned();
    }

    match output.status.code() {
        Some(code) if code == 10 => {
            let pass_details = show_pass_details(&pass_entity)?;
            let mut rofi_args_mesg = rofi_args.clone();
            if pass_details.is_empty() {
                rofi_args_mesg.insert("-mesg".to_owned(), Some("No pass details".to_owned()));
            } else {
                rofi_args_mesg.remove("-mesg");
            }

            let suboutput = run_rofi(&rofi_args_mesg, &pass_details)?;
            match suboutput.status.code() {
                Some(1 | 11) => {
                    rofi_args_mesg.insert("-select".to_owned(), Some(pass_entity));
                    rofi_args_mesg.remove("-mesg");
                    passmenu(rofi_args_mesg, &pass_entries, is_wayland)?;
                }
                _ => {}
            }

            let selected_line = String::from_utf8(suboutput.stdout)?.trim_end().to_owned();
            let parsed_parts = selected_line.split_once(": ");
            match parsed_parts {
                Some((_, data_to_copy)) => {
                    copy(data_to_copy.to_owned(), is_wayland)?;
                }
                None => {}
            }
        }
        Some(code) if code != 1 => {
            copy_pass(&pass_entity)?;
        }
        _ => {
            println!("Process terminated by signal");
        }
    }
    Ok(())
}
