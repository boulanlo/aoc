// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"A Y
B X
C Z
"#;

fn part_1(input: &str) {
    // We have a list of Rock, Paper, Scissors (RPS) plays as our
    // input, formatted as a list of two plays. The left play is our
    // adversary's, and the right one is ours.
    //
    // The goal of this part is to calculate the score as given in the
    // prompt.

    // Let's go over each round of RPS.
    let result = input
        .lines()
        .map(|line| {
            // In this part, the rule is:
            // - A and X are "rock"
            // - B and Y are "paper"
            // - C and Z are "scissors"

            // First, we need to get the individual letters from the
            // overall line. Unwrapping is OK because the format is
            // guaranteed.
            let (left, right) = line.split_once(' ').unwrap();

            // Let's convert those letters to integers. We could use an
            // enumerated type, but there is a fancy solution using only
            // integers and I want to do this anyways.
            let theirs: u32 = match left {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };

            let ours: u32 = match right {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => unreachable!(),
            };

            // Now, let's construct our score. We know that one part of it
            // is what we selected (we need the +1 because of our
            // 0-indexing):
            (ours + 1)
	    // We also need to calculate the score for the outcome of
	    // the RPS round: 0 for our loss, 3 for a draw, and 6 for
	    // our win.
		+ if ours == theirs {
		    // If our shape is the same as theirs, it's a draw.
		    3
		} else if ours == (theirs + 2) % 3 {
		    // If their shape is 2 values behind us (modulo 3),
		    // then we lose. For example, our 2 (scissors) would
		    // be beaten by their 0 (rock); our 1 (paper) would be
		    // beaten by their 2 (scissors).
		    0
		} else {
		    // If it's neither a draw nor a loss, then it's a win.
		    6
		}
        })
        // The final result is the sum of the score of each RPS round.
        .sum::<u32>();

    println!("Part 1: {result}")
}

fn part_2(input: &str) {
    // The second part is similar: instead of interpreting the second
    // column as our RPS shape, we should interpret it as the desired
    // outcome.

    let result = input
        .lines()
        .map(|line| {
            // In this part, the rule is:
            // - A is "rock"
            // - B is "paper"
            // - C is "scissors"
            // We also have the outcome list:
            // - X is a loss for us
            // - Y is a draw
            // - Z is a win for us

            let (left, right) = line.split_once(' ').unwrap();

            let theirs: u32 = match left {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };

            // For this part, we have to compute what shape we need to do in order to fulfill the given outcome.
            let (ours, outcome): (u32, u32) = match right {
                // A loss: our shape should be 1 less than theirs, and the RPS part of the score is 0.
                "X" => (if theirs == 0 { 2 } else { theirs - 1 }, 0),
                // A draw: our shape should be the same as theirs, and the RPS part of the score is 3.
                "Y" => (theirs, 3),
                // A win: our shape should be 1 more than theirs, and the RPS part of the score is 6.
                "Z" => ((theirs + 1) % 3, 6),
                _ => unreachable!(),
            };

            // Since we already know the outcome, we don't have to do
            // any fancy calculation: just add our shape's value (plus
            // one since we're zero-indexing) and the outcome's value.
            ours + 1 + outcome
        })
        .sum::<u32>();

    println!("Part 2: {result}")
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
