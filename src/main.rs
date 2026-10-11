use std::path::Path;
use std::{env, fs, io};

const LIMIT: &str = "8D ?? ?? 66 3B ?? 7C ?? 0F B7 ?? 89 ?? ?? 66 8B ?? 8B ?? ?? 83 ?? ?? 66 3B ?? 7C ?? 0F B7 ?? ?? E8 03 00 00";
const MARGINS: [usize; 2] = [2, 22];

fn patch(path: &Path) -> io::Result<Option<&'static str>> {
    let mut game = fs::read(path)?;
    let limit: Vec<Option<u8>> = LIMIT
        .split(' ')
        .map(|byte| u8::from_str_radix(byte, 16).ok())
        .collect();
    let fits = |start: &usize| {
        let mut pairs = game[*start..].iter().zip(&limit);
        pairs.all(|(byte, wanted)| wanted.is_none_or(|wanted| wanted == *byte))
    };
    let mut found = (0..game.len().saturating_sub(limit.len())).filter(fits);
    let (Some(start), None) = (found.next(), found.next()) else {
        return Ok(None);
    };
    let [width, height] = MARGINS.map(|margin| start + margin);
    match (game[width] as i8, game[height] as i8) {
        (0, 0) => Ok(Some("[*] already patched")),
        (-100, -80) => {
            fs::copy(path, path.with_extension("exe.bak"))?;
            game[width] = 0;
            game[height] = 0;
            fs::write(path, game)?;
            Ok(Some("[+] patched"))
        }
        _ => Ok(None),
    }
}

fn run() -> io::Result<bool> {
    let mut found = false;
    for file in fs::read_dir(env::current_exe()?.with_file_name(""))? {
        let path = file?.path();
        if !path
            .extension()
            .is_some_and(|kind| kind.eq_ignore_ascii_case("exe"))
        {
            continue;
        }
        if let Some(result) = patch(&path)? {
            println!(
                "{result} {}",
                path.file_name().unwrap_or_default().display()
            );
            found = true;
        }
    }
    Ok(found)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => println!("[!] no game here. put this file in the game folder"),
        Err(error) => println!("[-] {error}\n[*] close the game or try as administrator"),
    }
    println!("[*] press enter to close");
    let _ = io::stdin().read_line(&mut String::new());
}
