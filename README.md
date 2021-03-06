[![CircleCI](https://circleci.com/gh/benjaminjellis/wordle-solver/tree/master.svg?style=svg)](https://circleci.com/gh/benjaminjellis/wordle-solver/tree/master) ![MSRV](https://img.shields.io/badge/msrv-1.58.0-red)
# Wordle-Solver

Tool for suggesting words to solve wordle puzzles. 


## Usage 

This crate, by default, is a library the exposes a function to generate guesses based on the state of the puzzle. The crate also provides an optional feature 
'lambda' that allows you to deploy the function into an [AWS lambda](https://aws.amazon.com/lambda/) function

### Library usage

```rust
use wordle_solver::generate_guesses;

// state is the current correct placement of all known letters, if a spot is unknown it should be represented 
// by an underscore. E.g. you know the word looks like "C _ _ N E"
let state = String::from("C__NE");

// excluded_letters are the letters you have tried that are not in the word 
let excluded_letters = vec![
            "D".to_string(),
            "E".to_string(),
            "U".to_string(),
            "O".to_string(),
            "G".to_string(),
        ];
        
// unplaced_letters are letters that you know are in the word, but you don't know where they are in the word
let unplaced_letters = vec!["A".to_string()];

// excluded_placements are incorrect placements of letters that are in the word E.g. you know that A is in the word
// but it's not the second letter of the word 
let excluded_placements = vec!["_A__".to_string()];

let gussess = generate_guesses(
            state,
            excluded_letters,
            unplaced_letters,
            excluded_placements,
        )?;
println!("{:?}", gussess);
```


## Deployment 

This repo uses the [serverless framework](https://www.serverless.com/) to deploy the crate as a HTTP Rest API using AWS' API Gateway to route requests to a Lambda function

This can be done using the following commands (provided you have your AWS credentials configured correctly): 

```shell
npm install -D serverless
npm  install -D https://github.com/softprops/serverless-rust
npm install -D serverless-plugin-log-retention
npx serverless deploy
```

This deployment allows the tool to be used as a backend for a web extension called [auto-wrodle](https://github.com/BrownKnight/auto-wordle)


## Credits 

This project makes use of a list of worlde words which were sourced from [here](https://github.com/tabatkins/wordle-list)
