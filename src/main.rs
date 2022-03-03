mod language;
mod note;
mod poem;

use clipboard_win::set_clipboard_string;
use language::Language;
use markov::Chain;
use note::Note;
use poem::Poem;
use std::fs::{read_dir, read_to_string, DirEntry};
use std::io::{stdin, stdout, Write};
use std::path::Path;

const NOTES_DIR: &str = "notes";

fn get_language() -> Language {
    loop {
        println!("In which language do you want your poem?:");
        println!("\t1) Portuguese");
        println!("\t2) English");
        print!("\nYour choice: ");
        stdout().flush().unwrap();

        let mut lang = String::new();

        stdin().read_line(&mut lang).expect("Failed to read line");

        match lang.trim().parse::<u8>() {
            Ok(num) => match Language::from(num) {
                Language::None => {
                    println!("Please select one of the given options");
                    continue;
                }
                lang => break lang,
            },
            Err(_) => {
                println!("Please select one of the given options");
                continue;
            }
        }
    }
}

fn load_chain(lang: &Language) -> Option<Chain<String>> {
    println!("Loading {} chain...", lang);

    match Chain::load(Path::new(
        format!("./{}", lang.to_string().to_lowercase()).as_str(),
    )) {
        Ok(chain) => {
            println!("Chain loaded!");
            Some(chain)
        }
        Err(_) => {
            eprintln!("Could not load specified chain");
            None
        }
    }
}

fn get_notes(lang: &Language) -> Vec<Note> {
    println!("Getting notes...");

    let mut notes: Vec<Note> = Vec::new();

    for file in read_dir(NOTES_DIR).unwrap() {
        let entry: DirEntry = file.unwrap();

        let note: Note =
            serde_json::from_str(read_to_string(entry.path()).unwrap().as_str()).unwrap();

        if note.is(&lang) {
            notes.push(note);
        }
    }

    println!("Notes fetched!");

    notes
}

fn regen_chain(lang: &Language) -> Chain<String> {
    println!("Regenerating {} chain...", lang);

    let notes: Vec<Note> = get_notes(lang);

    let mut chain: Chain<String> = Chain::new();

    for note in notes {
        chain.feed_str(note.content().as_str());
    }

    match &chain.save("welele") {
        Err(_) => eprintln!("Failed to save chain"),
        _ => {}
    };

    println!("Chain regenerated!");

    chain
}

fn get_chain(lang: &Language) -> Chain<String> {
    print!("Do you want to regenerate the chain? (y/n): ");
    stdout().flush().unwrap();

    let mut regen: String = String::new();
    stdin().read_line(&mut regen).expect("Failed to read line");

    let regenerate_chain: bool = match regen.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("I don't get what you tried to type, I'll just regenerate that for you");
            true
        }
    };

    match regenerate_chain {
        false => match load_chain(lang) {
            Some(chain) => chain,
            None => regen_chain(lang),
        },
        true => regen_chain(lang),
    }
}

fn generate(lang: Language) -> Poem {
    if let Language::None = lang {
        std::process::exit(exitcode::DATAERR);
    }

    println!("Getting Markov chain...");

    let chain: Chain<String> = get_chain(&lang);

    println!("Generating your poem...");

    Poem(chain.generate_str())
}

fn copy_to_clipboard(poem: Poem) -> () {
    print!("Do you want to copy the poem to the clipboard? (y/n): ");
    stdout().flush().unwrap();

    let mut copy: String = String::new();
    stdin().read_line(&mut copy).expect("Failed to read line");

    let copy_to_clipboard: bool = match copy.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("I don't get what you tried to type, I won't copy it");
            false
        }
    };

    match copy_to_clipboard {
        true => match set_clipboard_string(poem.as_str()) {
            Err(_) => eprintln!("Failed to copy to clipboard"),
            _ => {
                println!("Copied to clipboard!");
            }
        },
        false => {
            println!("Carry on, then!")
        }
    }
}

fn wait_to_exit() -> () {
    println!("\nPress any key to exit");
    let mut _var = String::new();

    stdin().read_line(&mut _var).expect("Failed to read line");
}

fn main() {
    let poem: Poem = generate(get_language());

    println!("{}", poem);

    copy_to_clipboard(poem);

    wait_to_exit();
}
