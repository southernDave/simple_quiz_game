use std::io;

fn main(){
    println!("Welcome to the quiz game");

    //declaring program variables
    let mut score = 0;
    let mut question_index = 0;

    let questions = vec!["what is the capital of indonesia?", 
                         "what is the capital of Portugal?",
                         "what is the capital of England?"
    ];
    let answers = vec!["Bali",
                      "Lisboa",
                      "London"
    ];

    loop {
    let mut answer = String::new();
    println!("{}", questions[question_index]);

    io::stdin().read_line(&mut answer).expect("please enter the correct value");
    
    if answer.trim().to_lowercase() == answers[question_index].to_lowercase(){
        println!("well done, this is correct");
        score += 1;
    } else {
        println!("please try again")
    }

    question_index += 1;
    if question_index >= questions.len() {
        question_index = 0;
        answer = String::new();
        println!("your score is: {}/{}", score, questions.len());
        println!("enter 'q' to quit");
        io::stdin().read_line(&mut answer).expect("please enter the correct value");
        match answer.trim() {
            "q" => break,
            _ => println!("game continues")
        }

        score = 0;

    }
    
    }

}