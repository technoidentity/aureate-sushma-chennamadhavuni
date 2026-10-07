use std::collections::HashSet;
use std::io;

fn main() {
    #[rustfmt::skip]
    let dictionary = vec![
            "CARGO", "MUTABLE", "IMMUTABLE", "VARIABLE", "MEMORY", "MOVE", "COPY", "CLONE","COMPILER", "STRUCT", "ENUM", "MATCH",
            "TRAIT","GENERIC", "CLOSURE", "LIFETIME","BORROW", "REFERENCE", "OWNER", "SLICE", "VECTOR", "STRING", "OPTION", "RESULT",
            "PANIC", "CRATE", "MODULE", "PUBLIC", "PRIVATE", "SCOPE", "GLOBAL", "STATIC","CONST", "POINTER", "THREAD", "MUTEX",
            "LOCK", "ASYNC", "AWAIT", "FUTURE","MACRO", "PRIMITIVE", "TUPLE", "ARRAY", "ITERATOR", "RECURSION", "HEAP", "STACK",
            "ALLOC", "SAFETY",
        ];

    let secret_index = 0;
    let target_word = dictionary[secret_index];

    let mut attempts_remaining: i32 = 3;
    let mut total_guesses_made: i32 = 0;
    let mut wrong_guesses_made: i32 = 0;
    let mut correct_guesses_made: i32 = 0;

    let mut guessed_letters: HashSet<char> = HashSet::new();

    println!("==================================================");
    println!("        WELCOME TO GUESS THE WORD!                ");
    println!(" TOPIC: Programming Concepts & Rust Fundamentals ");
    println!("==================================================");

    loop {
        let current_score = (correct_guesses_made * 4) - (wrong_guesses_made * 1);
        println!(
            "\nAttempts remaining: {} | Current Score: {} pts",
            attempts_remaining, current_score
        );
        display_progress(target_word, &guessed_letters);
        display_guessed_letters(&guessed_letters);

        if is_word_guessed(target_word, &guessed_letters) {
            let score = (correct_guesses_made * 4) - (wrong_guesses_made * 1);
            let final_score = if score < 0 { 0 } else { score };

            println!(
                "\n🎉 Congratulations! You guessed the word: {} 🎉",
                target_word
            );
            println!("--------------------------------------------------");
            println!("📊 GAME SUMMARY:");
            println!(" • Total guesses taken: {}", total_guesses_made);
            println!(" • Incorrect attempts: {}", wrong_guesses_made);
            println!(" • Attempts remaining: {}", attempts_remaining);
            println!(" • Final Score: {} points", final_score);
            println!("--------------------------------------------------");
            break;
        }

        if attempts_remaining <= 0 {
            println!("\n❌ Game Over! You've run out of attempts.");
            println!("The secret word was: {}", target_word);
            println!("--------------------------------------------------");
            println!("📊 GAME SUMMARY:");
            println!(" • Total guesses taken: {}", total_guesses_made);
            println!(" • Final Score: 0 points");
            println!("--------------------------------------------------");
            break;
        }
        println!("\nEnter a letter guess: ");

        let user_guess = match read_user_char() {
            Some(ch) => ch.to_ascii_uppercase(),
            None => {
                println!("Invalid input. Please enter a single letter.");
                continue;
            }
        };

        if guessed_letters.contains(&user_guess) {
            println!(
                "You already guessed '{}'! Try a different letter.",
                user_guess
            );
            continue;
        }

        guessed_letters.insert(user_guess);
        total_guesses_made += 1;

        if target_word.contains(user_guess) {
            correct_guesses_made += 1;
            attempts_remaining += 2;
            println!(
                "Good job! '{}' is in the word (+4 points & +2 attempts!).",
                user_guess
            );
        } else {
            wrong_guesses_made += 1;
            attempts_remaining -= 1;
            println!(
                "Sorry, '{}' is not in the word (-1 point & -1 attempt).",
                user_guess
            );
        }
        let current_score = (correct_guesses_made * 4) - (wrong_guesses_made * 1);
        println!("🎯 Current Score: {} points", current_score);
    }
}

fn display_progress(word: &str, guessed_letters: &HashSet<char>) {
    print!("Word: ");
    for ch in word.chars() {
        if guessed_letters.contains(&ch) {
            print!("{} ", ch);
        } else {
            print!("_ ");
        }
    }
    println!();
}

fn display_guessed_letters(guessed_letters: &HashSet<char>) {
    let mut sorted_guesses: Vec<&char> = guessed_letters.iter().collect();
    sorted_guesses.sort();

    print!("Guessed letters: ");
    for ch in sorted_guesses {
        print!("{} ", ch);
    }
    println!();
}

fn is_word_guessed(word: &str, guessed_letters: &HashSet<char>) -> bool {
    word.chars().all(|ch| guessed_letters.contains(&ch))
}

fn read_user_char() -> Option<char> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    if trimmed.len() == 1 {
        trimmed.chars().next()
    } else {
        None
    }
}
