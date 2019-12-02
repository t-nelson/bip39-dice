use bip39;
use bip39_dice::{dice_to_index, index_to_dice, solve_checkwords};
use clap::{App, Arg, ArgMatches, SubCommand};
use rand::{thread_rng, Rng};

fn join(strings: &[&str], delim: &str) -> String {
    let mut it = strings.iter();
    let mut out = it.next().unwrap().to_string();
    it.for_each(|string| {
        out += delim;
        out += string;
    });
    out
}

const LANG_DEFAULT: &str = "english";
const LANG_VALUES: [&str; 1] = [LANG_DEFAULT];

fn get_bip39_language(s: &str) -> bip39::Language {
    let s = s.to_ascii_lowercase();
    match s.as_str() {
        "english" => bip39::Language::English,
        _ => panic!("Unexpected language: {}", s),
    }
}

const MNEMONIC_LENGTH_DEFAULT: &str = "12";
const MNEMONIC_LENGTH_VALUES: [&str; 5] = [MNEMONIC_LENGTH_DEFAULT, "15", "18", "21", "24"];

fn main() {
    let matches = App::new("BIP39 Dice")
        .version("0.1")
        .author("Trent Nelson <trent@solana.com>")
        .about("Utility for generating BIP39 Seed Phrases with dice")
        .arg(Arg::with_name("language")
            .long("language")
            .help("Specifiy BIP39 wordlist language.")
            .takes_value(true)
            .possible_values(&LANG_VALUES)
            .default_value(LANG_DEFAULT)
        )
        .subcommand(SubCommand::with_name("convert-dice")
            .about("Convert a 5-dice roll to BIP39 word.\nNOTE: To prevent biasing the output, not all rolls map to words")
            .arg(Arg::with_name("dice_roll")
                .index(1)
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("dice-wordlist")
            .about("Print a dice-roll to BIP39 wordlist mapping")
        )
        .subcommand(SubCommand::with_name("solve-checkword")
            .about("Solve for the checkword options of a seed phrase base")
            .arg(Arg::with_name("length")
                .long("length")
                .help("Specify number of words in seed phrase")
                .takes_value(true)
                .possible_values(&MNEMONIC_LENGTH_VALUES)
                .default_value(MNEMONIC_LENGTH_DEFAULT)
            )
            .arg(Arg::with_name("random")
                .long("random")
                .help("Generate random seed phrase base")
                .takes_value(false)
            )
            .arg(Arg::with_name("seed_phrase_word")
                .index(1)
                .help("The base seed phrase to solve checkwords for")
                .multiple(true)
                .required_unless("random")
            )
        )
        .get_matches();

    let mnemonic_lang = get_bip39_language(matches.value_of("language").unwrap());
    let word_list = mnemonic_lang.wordlist();

    let default_matches = ArgMatches::default();
    match matches.subcommand() {
        ("convert-dice", matches) => {
            let matches = matches.unwrap_or(&default_matches);
            let dice_max = dice_to_index("66666");
            let bip39_dice_max = (dice_max / 2048) * 2048;
            let roll = dice_to_index(matches.value_of("dice_roll").unwrap());
            if roll < bip39_dice_max {
                println!("{}", word_list.get_word((roll % 2048).into()));
            } else {
                println!("Roll out of bounds! Re-roll and try again");
            }
        }
        ("dice-wordlist", _matches) => {
            let dice_max = dice_to_index("66666");
            let bip39_dice_max = (dice_max / 2048) * 2048;
            for i in 0u16..bip39_dice_max {
                let dice = index_to_dice(i);
                let word_index = i % 2048;
                let word = word_list.get_word(word_index.into());
                println!("{}  {}", dice, word);
            }
        }
        ("solve-checkword", matches) => {
            let matches = matches.unwrap_or(&default_matches);
            let mnemonic_type = bip39::MnemonicType::for_word_count(
                matches.value_of("length").unwrap().parse().unwrap(),
            )
            .unwrap();
            let words: Vec<&str> = if matches.is_present("random") {
                let mut rng = thread_rng();
                (0..mnemonic_type.word_count() - 1)
                    .map(|_| {
                        let r: u16 = rng.gen_range(0, 2048);
                        word_list.get_word(r.into())
                    })
                    .collect()
            } else {
                matches.values_of("seed_phrase_word").unwrap().collect()
            };
            let checkwords = solve_checkwords(&words, mnemonic_type);
            let seed_phrase_base = join(&words, " ");
            let seed_phrases: Vec<String> = checkwords
                .iter()
                .map(|checkword| join(&[&seed_phrase_base, checkword], " "))
                .collect();

            for seed_phrase in &seed_phrases {
                let ok = bip39::Mnemonic::validate(&seed_phrase, mnemonic_lang).is_ok();
                println!("{:5}  {}", ok, seed_phrase);
            }
        }
        (subcommand, _) => {
            if subcommand.is_empty() {
                eprintln!("A subcommand is required!\n");
            } else {
                eprintln!("Unexpected subcommand: {}\n", subcommand);
            };
            eprintln!("{}", matches.usage());
        }
    }
}
