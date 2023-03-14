use rand::Rng;
use std::fmt;
use std::io;

// User structure
struct Question {
    question: String,
    answers: [String; 4],
    correct_answer: u8,
    audience: String,
    phone: String,
    money: u32,
}

// Jokers
enum Joker {
    J50_50,
    Audience,
    Call,
}

// Response types
enum Response {
    Correct,
    Incorrect,
    Quit,
}

// Handle asking a question from user
fn ask_question(
    question: &Question,
    helps: &mut u8,
    jokers: &mut Vec<Joker>,
    name: &String,
) -> io::Result<Response> {
    // If there is any Jokers available, show Joker option
    let joker_selection = if has_available_jokers(&jokers) {
        "J) Joker\n"
    } else {
        ""
    };
    // Show question
    println!(
        "OK {}. The ${} question:\n{}\n1) {}\n2) {}\n3) {}\n4) {}\n{}Q) Quit",
        name,
        question.money,
        question.question,
        question.answers[0],
        question.answers[1],
        question.answers[2],
        question.answers[3],
        joker_selection
    );

    // Get user answer
    let mut answer: String = String::new();
    get_user_input(&mut answer, is_valid_answer, jokers)?;

    // Flag for asking again (if Joker selected)
    let mut ask_again = true;

    // Use Jokers
    match answer.to_uppercase().trim() {
        // Joker
        "J" => {
            println!("Joker selected");
            println!("Which Joker do you want to use?");
            // Display Jokers to user
            show_available_jokers(&jokers);
            // Get a Joker selection from user
            let selected_joker = select_joker(jokers);
            // Apply Joker
            match selected_joker {
                Ok(Joker::J50_50) => {
                    // Remove two of invalid options
                    println!("OK, 50/50 Joker selected");
                    apply_joker_50_50(question);
                }
                Ok(Joker::Audience) => {
                    // Show audience help
                    println!("OK, Audience Joker selected");
                    println!("Audience selection is as follows:");
                    println!("{}", question.audience);
                }
                Ok(Joker::Call) => {
                    // Show call help
                    println!("OK, Call Joker selected");
                    println!("The person behind the phone says:");
                    println!("{}", question.phone);
                }
                _ => {
                    // This section should not be reached
                    println!("Invalid Joker selection.");
                }
            }
        }
        // Quit
        "Q" => return Ok(Response::Quit),
        _ =>
        /* Going for answer check */
        {
            ask_again = false;
        }
    };

    // If one of the Jokers used, ask again for user answer
    if ask_again {
        println!("Now select your answer:");
        get_user_input(&mut answer, is_valid_answer_no_joker, jokers)?;
    }

    return match answer.to_uppercase().trim() {
        // Main options
        "1" | "2" | "3" | "4" => {
            let answer_index: u8 = answer.trim().parse::<u8>().unwrap() - 1;

            if answer_index == question.correct_answer {
                // Correct answer
                Ok(Response::Correct)
            } else {
                // Incorrect answer

                // Help user (if available)
                if *helps > 0 {
                    println!("Are you really sure? You can change your mind if you want:");
                    *helps = *helps - 1;
                    get_user_input(&mut answer, is_valid_answer_no_joker, jokers)?;
                    return match answer.to_uppercase().trim() {
                        "1" | "2" | "3" | "4" => {
                            // Calculate index
                            let answer_index: u8 = answer.trim().parse::<u8>().unwrap() - 1;

                            if answer_index == question.correct_answer {
                                // Correct answer
                                Ok(Response::Correct)
                            } else {
                                // Wrong answer
                                Ok(Response::Incorrect)
                            }
                        }
                        "Q" => Ok(Response::Quit),
                        // This section should not be reached
                        _ => Ok(Response::Incorrect),
                    };
                }
                // No helps remaining
                Ok(Response::Incorrect)
            }
        }
        // Quit game
        "Q" => Ok(Response::Quit),
        // This section should not be reached
        _ => Ok(Response::Incorrect),
    };
}

// Gets input from user and validates it using provided closure
fn get_user_input(
    answer: &mut String,
    validator: fn(&str, &Vec<Joker>) -> bool,
    jokers: &Vec<Joker>,
) -> io::Result<()> {
    // Validate answer
    loop {
        // Get user answer
        answer.clear();
        io::stdin().read_line(answer)?;
        println!("Your answer: {}", answer.trim());
        if validator(answer.to_uppercase().trim(), jokers) {
            return Ok(());
        } else {
            println!("Your selection is invalid, please select one of the options.");
        }
    }
}

// Checks whether the provided answer is correct or not
fn is_valid_answer(answer: &str, jokers: &Vec<Joker>) -> bool {
    match answer {
        // Main options
        "1" | "2" | "3" | "4" => true,
        // Joker
        "J" => has_available_jokers(&jokers),
        // Quit
        "Q" => true,
        // Handle the rest of cases
        _ => false,
    }
}

// Checks whether the provided answer is correct or not
fn is_valid_answer_no_joker(answer: &str, _jokers: &Vec<Joker>) -> bool {
    match answer {
        // Main options
        "1" | "2" | "3" | "4" => true,
        // Quit
        "Q" => true,
        // Handle the rest of cases
        _ => false,
    }
}

// Validates selection of Jokers
fn is_valid_joker_answer(answer: &str, jokers: &Vec<Joker>) -> bool {
    match answer {
        // First Joker
        "1" => !jokers.is_empty(),
        // Second Joker
        "2" => jokers.len() >= 2,
        // Third Joker
        "3" => jokers.len() >= 3,
        // Handle the rest of cases
        _ => false,
    }
}

// Checks to see if there is any Jokers left or not
fn has_available_jokers(jokers: &Vec<Joker>) -> bool {
    !jokers.is_empty()
}

// Print available jokers
fn show_available_jokers(jokers: &Vec<Joker>) {
    for (i, joker) in jokers.iter().enumerate() {
        println!("{}) {}", i + 1, joker);
    }
}

// Selects a Joker by user input
fn select_joker(jokers: &mut Vec<Joker>) -> io::Result<Joker> {
    let mut answer: String = String::new();
    get_user_input(&mut answer, is_valid_joker_answer, jokers)?;
    // Detect Joker index
    let index = answer.trim().parse::<usize>().unwrap() - 1;
    // Remove and return Joker object
    Ok(jokers.remove(index))
}

// Removes two of the incorrect options
fn apply_joker_50_50(question: &Question) {
    let mut rng = rand::thread_rng();
    let mut first: Option<u8> = None;
    let second: Option<u8>;

    // Select two random numbers which are not the correct answer
    loop {
        let number: u8 = rng.gen_range(0..4);

        if number == question.correct_answer {
            // Avoid removing correct answer
            continue;
        }

        if first == None {
            // First removal
            first = Some(number);
        } else {
            // Second removal
            second = Some(number);
            break;
        }
    }
    let first_index = first.unwrap() as usize;
    let second_index = second.unwrap() as usize;
    // Show removed options
    println!(
        "These two options are incorrect: {}){} and {}){}",
        first_index + 1,
        question.answers[first_index],
        second_index + 1,
        question.answers[second_index]
    );
}

// Joker formatter
impl fmt::Display for Joker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Joker::J50_50 => write!(f, "50/50"),
            Joker::Audience => write!(f, "Audience"),
            Joker::Call => write!(f, "Call"),
        }
    }
}

// Main function
fn main() -> io::Result<()> {
    // Questions container
    let questions: [Question; 15] = [
        Question {
            question: String::from("What is the biggest currency in Europe?"),
            answers: [
                String::from("Afro"),
                String::from("Asio"),
                String::from("Australio"),
                String::from("Euro"),
            ],
            correct_answer: 3,
            audience: String::from("1: 0%, 2: 2%, 3: 0%, 4: 98%"),
            phone: String::from("Haha, are you kidding me? The answer is 4!"),
            money: 100,
        },
        Question {
            question: String::from("Which of these kills its victims by constriction?"),
            answers: [
                String::from("Andalucia"),
                String::from("Anaconda"),
                String::from("Andypandy"),
                String::from("Annerobinson"),
            ],
            correct_answer: 1,
            audience: String::from("1: 2%, 2: 95%, 3: 2%, 4: 1%"),
            phone: String::from("Come on! Im sure the answer is 2!"),
            money: 200,
        },
        Question {
            question: String::from("In the UK, VAT stands for value-added ...?"),
            answers: [
                String::from("Transaction"),
                String::from("Total"),
                String::from("Tax"),
                String::from("Trauma"),
            ],
            correct_answer: 2,
            audience: String::from("1: 15%, 2: 10%, 3: 75%, 4: 0%"),
            phone: String::from("Oh, umm, isn't it 3? I think it is 3. Yes, try 3!"),
            money: 300,
        },
        Question {
            question: String::from("What might an electrician lay?"),
            answers: [
                String::from("Tables"),
                String::from("Gables"),
                String::from("Cables"),
                String::from("Stables"),
            ],
            correct_answer: 2,
            audience: String::from("1: 28%, 2: 3%, 3: 68%, 4: 1%"),
            phone: String::from("Well... an electrician uses cables, so 3 right?"),
            money: 500,
        },
        Question {
            question: String::from("How is a play on words commonly described?"),
            answers: [
                String::from("Pan"),
                String::from("Pin"),
                String::from("Pen"),
                String::from("Pun"),
            ],
            correct_answer: 3,
            audience: String::from("1: 21%, 2: 12%, 3: 19%, 4: 48%"),
            phone: String::from("Oh, a play on words... is it 1 or 4? I think 4."),
            money: 1000,
        },
        Question {
            question: String::from("Which color is used as a term to describe an illegal market for rare goods?"),
            answers: [
                String::from("Black"),
                String::from("Blue"),
                String::from("Red"),
                String::from("White"),
            ],
            correct_answer: 0,
            audience: String::from("1: 58%, 2: 4%, 3: 21%, 4: 17%"),
            phone: String::from("I know that black markets exists, I have heard of that. I don't know what they are though... go with 1!"),
            money: 2000,
        },
        Question {
            question: String::from("Which character was first played by Arnold Schwarzenegger in a 1984 film?"),
            answers: [
                String::from("The Demonstrator"),
                String::from("The Instigator"),
                String::from("The Investigator"),
                String::from("The Terminator"),
            ],
            correct_answer: 3,
            audience: String::from("1: 4%, 2: 3%, 3: 11%, 4: 82%"),
            phone: String::from("It is his most famous movie! The answer is 4!"),
            money: 4000,
        },
        Question {
            question: String::from("In which country would you expect to be greeted with the word 'bonjour'?"),
            answers: [
                String::from("France"),
                String::from("Italy"),
                String::from("Spain"),
                String::from("Wales"),
            ],
            correct_answer: 0,
            audience: String::from("1: 72%, 2: 12%, 3: 14%, 4: 2%"),
            phone: String::from("Isn't that french? So 1 should be the answer, right?"),
            money: 8000,
        },
        Question {
            question: String::from("What name is given to the person who traditionally attends the groom on his wedding day?"),
            answers: [
                String::from("Best man"),
                String::from("Top man"),
                String::from("Old man"),
                String::from("Poor man"),
            ],
            correct_answer: 0,
            audience: String::from("1: 48%, 2: 38%, 3: 14%, 4: 0%"),
            phone: String::from("It is either 1 or 2, I am not sure though..."),
            money: 16000,
        },
        Question {
            question: String::from("People who are in a similar unfavourable situation are said to be 'all in the same ...'?"),
            answers: [
                String::from("Train"),
                String::from("Plane"),
                String::from("Boat"),
                String::from("Tube"),
            ],
            correct_answer: 2,
            audience: String::from("1: 15%, 2: 8%, 3: 77%, 4: 0%"),
            phone: String::from("All in the same boat I think: try 3!"),
            money: 32000,
        },
        Question {
            question: String::from("According to the old adage, how many lives does a cat have?"),
            answers: [
                String::from("Five"),
                String::from("Seven"),
                String::from("Nine"),
                String::from("Ten"),
            ],
            correct_answer: 2,
            audience: String::from("1: 17%, 2: 42%, 3: 39%, 4: 2%"),
            phone: String::from("Ohh... I don't know if it is 2 or 3. Definitely not 4."),
            money: 64000,
        },
        Question {
            question: String::from("How many countries use the Euro as their currency?"),
            answers: [
                String::from("12"),
                String::from("16"),
                String::from("19"),
                String::from("28"),
            ],
            correct_answer: 2,
            audience: String::from("1: 19%, 2: 26%, 3: 28%, 4: 27%"),
            phone: String::from("I am terribly sorry but I have no idea."),
            money: 125000,
        },
        Question {
            question: String::from("How many different time zones exist within Russia?"),
            answers: [
                String::from("8"),
                String::from("7"),
                String::from("6"),
                String::from("5"),
            ],
            correct_answer: 0,
            audience: String::from("1: 38%, 2: 26%, 3: 21%, 4: 15%"),
            phone: String::from("Russia?! How would I know?! The country is huge, must be a lot."),
            money: 250000,
        },
        Question {
            question: String::from("California has almost the same population as..."),
            answers: [
                String::from("The United Kingdom"),
                String::from("Spain"),
                String::from("Italy"),
                String::from("Poland"),
            ],
            correct_answer: 3,
            audience: String::from("1: 9%, 2: 31%, 3: 11%, 4: 49%"),
            phone: String::from("I have no idea. Which country is the smallest? The smallest one is probably correct."),
            money: 500000,
        },
        Question {
            question: String::from("THE FINAL QUESTION!!!!! How many Game Boys were sold world wide?"),
            answers: [
                String::from("over 50 Million"),
                String::from("over 100 Million"),
                String::from("over 500 Million"),
                String::from("over one Billion"),
            ],
            correct_answer: 1,
            audience: String::from("1: 32%, 2: 29%, 3: 28%, 4: 11%"),
            phone: String::from("I have absolutely no clue! But wait, one billion cannot be right, can it? How many people live on the earth?!"),
            money: 1000000,
        },
    ];

    let mut won: bool = false;
    let mut helps: u8 = 2;
    let jokers: &mut Vec<Joker> = &mut vec![Joker::J50_50, Joker::Audience, Joker::Call];
    let mut name = String::new();

    println!("Hello and welcome to Be A Millionaire game!");
    println!("You will be asked 15 questions and if you answer all of them, you will win $1M!");
    // Get user name
    println!("What is your name?");
    io::stdin().read_line(&mut name)?;
    name = String::from(name.trim());

    // Loop for all questions
    for question in questions.iter() {
        // Ask questions one by one
        match ask_question(question, &mut helps, jokers, &name) {
            Ok(Response::Correct) => {
                println!("Congratulations {}! Your answer was correct!", name);
                // Check to see if user won the game
                if question.money == 1000000 {
                    won = true;
                }
            }
            Ok(Response::Incorrect) => {
                // Failed to answer question
                // Print failing message
                println!("Sorry {} your answer was wrong!!! We are really sorry.\nBut, you will get ${}!",
                         name, question.money);
                // Get out of loop and program
                break;
            }
            Ok(Response::Quit) => {
                println!("OK, you quit the game with ${}.\nThank you for your participation in our game!",
                         question.money);
                break;
            }
            _ => {
                println!("Some errors occurred. Can not continue game.");
                break;
            }
        }
    }

    // Check to see if user won the game greet with special message!
    if won {
        // User won the grand prize
        println!("YOU WIN THE GRAND PRIZE {}!!!!!!!!!!!!!!!!!!", name);
        println!("$1,000,000 !!!!!!!!!!!!");
        println!("That was AMAZING !!!");
        println!("Thank you for participating in our game! Good luck!");
    }

    // Game ended
    Ok(())
}
