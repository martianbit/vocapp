use std::path::Path;
use std::collections::HashMap;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::env;
use std::process::Command;
use rand::prelude::*;

const NEW_WORDS_PER_DAY: usize = 10;
const SHARE_DIR_PATH: &str = "/home/martianbit/.local/share/vocapp";
const STUDICT_FILEPATH: &str = "/tmp/studict.txt";

fn save_knowledge(knowledge: &Vec<u32>, filepath: &Path) {
    let mut file = File::create(filepath).unwrap();

    for level in knowledge {
        writeln!(file, "{}", level).unwrap();
    }
}

fn load_knowledge(filepath: &Path, dict_size: usize) -> Vec<u32> {
    let mut knowledge = Vec::<u32>::with_capacity(dict_size);

    if filepath.is_file() {
        let raw_knowledge = fs::read_to_string(filepath).unwrap();

        for line in raw_knowledge.lines() {
            knowledge.push(line.parse().unwrap());
        }
    }

    if knowledge.len() != dict_size {
        while knowledge.len() < dict_size {
            knowledge.push(0);
        }

        save_knowledge(&knowledge, filepath);
    }

    knowledge
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let share_dir_path = Path::new(SHARE_DIR_PATH);

    let data_dir_path = share_dir_path.join(&args[1]);
    let knowledge_filepath = data_dir_path.join("knowledge.txt");

    let dict_filepath = data_dir_path.join("dict.txt");
    if !dict_filepath.is_file() { panic!(); }

    let raw_dict = fs::read_to_string(dict_filepath).unwrap();

    let mut dict = HashMap::<&str, &str>::new();
    let mut dict_index = Vec::<&str>::new();

    for line in raw_dict.lines() {
        let mut pair = line.split(": ");

        let key = pair.next().unwrap();

        if dict.contains_key(key) {
            continue;
        }

        dict.insert(key, pair.next().unwrap());
        dict_index.push(key);
    }

    let mut knowledge = load_knowledge(&knowledge_filepath, dict.len());

    let chosen = knowledge
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == 0)
        .map(|(i, _)| i)
        .choose_multiple(&mut rand::thread_rng(), NEW_WORDS_PER_DAY);

    {
        let mut studict_file = File::create(STUDICT_FILEPATH).unwrap();

        for &i in &chosen {
            writeln!(studict_file, "{}: {}", dict_index[i], dict[dict_index[i]]).unwrap();
        }
    }

    let ecode = Command::new("studict")
        .arg(STUDICT_FILEPATH)
        .arg(NEW_WORDS_PER_DAY.to_string())
        .spawn().unwrap()
        .wait().unwrap();

    if !ecode.success() {
        return ();
    }

    for &i in &chosen {
        knowledge[i] += 1;
    }

    save_knowledge(&knowledge, &knowledge_filepath);
}

