use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use dialoguer::Select;
use lipsum::lipsum_words_with_rng;
use rand::thread_rng;
use std::{
    io::{self, Write},
    time::Instant,
};

const APP_TITLE: &str = r"                                                                                                                                                                                                                                                                                                     
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEEYYYYYYY       YYYYYYYFFFFFFFFFFFFFFFFFFFFFF      AAA                 SSSSSSSSSSSSSSS TTTTTTTTTTTTTTTTTTTTTTT
K:::::::K    K:::::KE::::::::::::::::::::EY:::::Y       Y:::::YF::::::::::::::::::::F     A:::A              SS:::::::::::::::ST:::::::::::::::::::::T
K:::::::K    K:::::KE::::::::::::::::::::EY:::::Y       Y:::::YF::::::::::::::::::::F    A:::::A            S:::::SSSSSS::::::ST:::::::::::::::::::::T
K:::::::K   K::::::KEE::::::EEEEEEEEE::::EY::::::Y     Y::::::YFF::::::FFFFFFFFF::::F   A:::::::A           S:::::S     SSSSSSST:::::TT:::::::TT:::::T
KK::::::K  K:::::KKK  E:::::E       EEEEEEYYY:::::Y   Y:::::YYY  F:::::F       FFFFFF  A:::::::::A          S:::::S            TTTTTT  T:::::T  TTTTTT
  K:::::K K:::::K     E:::::E                Y:::::Y Y:::::Y     F:::::F              A:::::A:::::A         S:::::S                    T:::::T        
  K::::::K:::::K      E::::::EEEEEEEEEE       Y:::::Y:::::Y      F::::::FFFFFFFFFF   A:::::A A:::::A         S::::SSSS                 T:::::T        
  K:::::::::::K       E:::::::::::::::E        Y:::::::::Y       F:::::::::::::::F  A:::::A   A:::::A         SS::::::SSSSS            T:::::T        
  K:::::::::::K       E:::::::::::::::E         Y:::::::Y        F:::::::::::::::F A:::::A     A:::::A          SSS::::::::SS          T:::::T        
  K::::::K:::::K      E::::::EEEEEEEEEE          Y:::::Y         F::::::FFFFFFFFFFA:::::AAAAAAAAA:::::A            SSSSSS::::S         T:::::T        
  K:::::K K:::::K     E:::::E                    Y:::::Y         F:::::F         A:::::::::::::::::::::A                S:::::S        T:::::T        
KK::::::K  K:::::KKK  E:::::E       EEEEEE       Y:::::Y         F:::::F        A:::::AAAAAAAAAAAAA:::::A               S:::::S        T:::::T        
K:::::::K   K::::::KEE::::::EEEEEEEE:::::E       Y:::::Y       FF:::::::FF     A:::::A             A:::::A  SSSSSSS     S:::::S      TT:::::::TT      
K:::::::K    K:::::KE::::::::::::::::::::E    YYYY:::::YYYY    F::::::::FF    A:::::A               A:::::A S::::::SSSSSS:::::S      T:::::::::T      
K:::::::K    K:::::KE::::::::::::::::::::E    Y:::::::::::Y    F::::::::FF   A:::::A                 A:::::AS:::::::::::::::SS       T:::::::::T      
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEE    YYYYYYYYYYYYY    FFFFFFFFFFF  AAAAAAA                   AAAAAAASSSSSSSSSSSSSSS         TTTTTTTTTTT                                                                                                                                                             
";

#[derive(Clone)]
struct GameConfig {
    text_length: usize,
    description: &'static str,
}

impl GameConfig {
    const SHORT: Self = Self {
        text_length: 3,
        description: "Short text (3 words)",
    };
    const MEDIUM: Self = Self {
        text_length: 8,
        description: "Medium text (8 words)",
    };
    const LONG: Self = Self {
        text_length: 15,
        description: "Long text (15 words)",
    };
    const EXIT: Self = Self {
        text_length: 0,
        description: "Exit game",
    };
}

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();
    let result = run_app();
    disable_raw_mode().unwrap();
    result
}

fn run_app() -> io::Result<()> {
    print_app_title();
    display_menu()
}

fn print_app_title() {
    println!("{}", APP_TITLE);
}

fn display_menu() -> io::Result<()> {
    let options = [
        GameConfig::SHORT,
        GameConfig::MEDIUM,
        GameConfig::LONG,
        GameConfig::EXIT,
    ];
    let descriptions: Vec<&str> = options.iter().map(|c| c.description).collect();

    let selection = Select::new()
        .with_prompt("\n\nPlease select a game type")
        .items(&descriptions)
        .default(0)
        .interact_opt()
        .unwrap();

    match selection {
        Some(3) => Ok(leave_app()),
        Some(index) => start_game(options[index].text_length),
        None => display_menu(),
    }
}

fn start_game(word_count: usize) -> io::Result<()> {
    let target_text = generate_random_text(word_count);
    println!("\n\n{}\n\n", target_text);
    read_input_with_timer(&target_text)
}

fn generate_random_text(word_count: usize) -> String {
    lipsum_words_with_rng(thread_rng(), word_count)
}

fn read_input_with_timer(target_text: &str) -> io::Result<()> {
    let mut input = String::new();
    let mut timer_started = false;
    let mut start_time = Instant::now();

    println!("Start typing to begin... (Press Enter to finish, Press ESC to quit.)\n\n");

    loop {
        if let Ok(Event::Key(event)) = read() {
            match event.code {
                KeyCode::Char(c) if event.is_press() => {
                    if !timer_started {
                        start_time = Instant::now();
                        timer_started = true;
                    }

                    input.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        print!("\x08");
                        io::stdout().flush().unwrap();
                    }
                }
                KeyCode::Enter => {
                    if !timer_started {
                        continue;
                    }

                    let duration = start_time.elapsed();
                    show_results(&input, target_text, duration);
                    return display_menu();
                }
                KeyCode::Esc => leave_app(),
                _ => {}
            }
        }
    }
}

fn leave_app() {
    println!("Bye!");
    std::process::exit(0)
}

fn show_results(input: &str, target_text: &str, duration: std::time::Duration) {
    let duration_ms = duration.as_millis();
    let duration_secs = duration.as_secs_f32();

    if input == target_text {
        let cps = if duration_secs > 0.0 {
            target_text.len() as f32 / duration_secs
        } else {
            0.0
        };

        println!("\n\nGG! You win in {}ms", duration_ms);
        println!("Character/sec: {:.1}", cps);
    } else {
        println!("\n\nYou lost in {}ms", duration_ms);
    }
}
