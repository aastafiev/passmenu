use passmenu::{passmenu, passwords, BoxResult};
use std::collections::HashMap;

fn main() -> BoxResult<()> {
    let rofi_args = HashMap::from([
        ("-kb-accept-alt".to_owned(), Some("Alt+1".to_owned())),
        ("-kb-custom-1".to_owned(), Some("Shift+Return".to_owned())),
        ("-kb-mode-previous".to_owned(), Some("Alt+2".to_owned())),
        ("-kb-custom-2".to_owned(), Some("Control+ISO_Left_Tab".to_owned())),
        ("-dmenu".to_owned(), None),
        ("-no-fixed-num-lines".to_owned(), None),
        ("-p".to_owned(), Some(">>>".to_owned())),
        ("-i".to_owned(), None),
    ]);
    let pass_entries = passwords()?;

    passmenu(rofi_args, &pass_entries)?;
    Ok(())
}
