use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
"#;

// We are going to define a few type aliases to make our life easier.

// A worry value is a 64-bit integer.
type WorryValue = u64;
// A monkey ID is a usize (as it will be used to index a vec).
type MonkeyId = usize;
// An operation is a function that takes a worry value and outputs a new one.
// We need a lifetime here because we'll be borrowing the input string.
type Operation<'a> = Box<dyn Fn(WorryValue) -> WorryValue + 'a>;
// A divisibility test is a function that takes a worry value and a divisor,
// and returns the corresponding monkey ID alongside with a worry value.
type DivisibilityTest<'a> = Box<dyn Fn(WorryValue, u64) -> (MonkeyId, WorryValue) + 'a>;

// This function parses an operation from a string.
fn parse_operation(input: &str) -> Operation {
    // We'll need to get the tokens of the operation. We can skip the first
    // three: "Operation:", "new" and "=", as they are the same across all
    // monkeys.
    let mut tokens = input.split_ascii_whitespace().skip(3);

    // Now let's get each token one by one.
    let left = tokens.next().unwrap();
    let op = tokens.next().unwrap();
    let right = tokens.next().unwrap();

    // And now, we define what the body of an "operation" is.
    Box::new(move |old| {
        // The left operand is either the input if it's the string "old", or
        // a string literal otherwise.
        let left = match left {
            "old" => old,
            num => num.parse().unwrap(),
        };

        // The same for the right operand.
        let right = match right {
            "old" => old,
            num => num.parse().unwrap(),
        };

        // What's the operation to be performed here?
        match op {
            "+" => left + right,
            "*" => left * right,
            // We should always have either a + or a *, so we can panic here.
            _ => unreachable!(),
        }
    })
}

// This function parses a divisibility test. We get as input the three lines
// forming the test, and we return both the divisibility test function and the
// divisor used inside of it. We'll need it later.
fn parse_divisibility_test<'a>(
    test: &'a str,
    if_true: &'a str,
    if_false: &'a str,
) -> (DivisibilityTest<'a>, WorryValue) {
    // The divisor is the 3rd token on the "test" line.
    let divisor = test
        .split_ascii_whitespace()
        .nth(3)
        .unwrap()
        .parse::<WorryValue>()
        .unwrap();

    // The monkey ID for the "true" branch is the 5th token on the "if_true" line.
    let true_target = if_true
        .split_ascii_whitespace()
        .nth(5)
        .unwrap()
        .parse::<MonkeyId>()
        .unwrap();

    // The monkey ID for the "false" branch is the 5th token on the
    //"if_false" line.
    let false_target = if_false
        .split_ascii_whitespace()
        .nth(5)
        .unwrap()
        .parse::<MonkeyId>()
        .unwrap();

    (
        // Here we define the body of a divisibility test. Given a worry value
        // and a divisor...
        Box::new(move |value, divisor| {
            // If the value is divisible by the divisor:
            if (value % divisor) == 0 {
                // Return the "true" target and the value;
                (true_target, value)
            } else {
                // Otherwise, return the "false" target and the value.
                (false_target, value)
            }
        }),
        // Don't forget to return the actual divisor too.
        divisor,
    )
}

// This function parses a monkey from the string. We also get as input whether
// the monkey will get bored after inspecting an item; this will be true for the
// first part, and false for the second part.
fn parse_monkey(input: &str, becomes_bored: bool) -> Monkey {
    // Let's get the different lines from the input:
    let [
        // The monkey's ID, which we don't care about.
        _,
        // The list of starting items. 
        starting_items,
        // The operation performed on each inspection.
        operation, 
        // The divisibility test.
        test,
        // The target if the divisibility test returns true. 
        if_true, 
        // The target if the divisibility test returns false.
        if_false
    ] =
        // We get all of this from the input, divided into lines. We know that 
        // we'll get exactly 6 lines from the input, so we can use `try_into()` 
        // and unwrap here.
        input.lines().collect::<Vec<&str>>().try_into().unwrap();

    // Let's parse the items: taking the items string...
    let items = starting_items
        // ...we split it by whitespace and commas...
        .split(|c: char| c.is_ascii_whitespace() || c == ',')
        // ...filter out the ones that aren't numbers...
        .filter_map(|token| token.parse::<WorryValue>().ok())
        // ...and collect them all into a vector.
        .collect::<Vec<_>>();

    // We also need to parse the operation, the divisibility test and the 
    // divisor, using the previous functions.
    let operation = parse_operation(operation);
    let (divisibility_test, divisor) = parse_divisibility_test(test, if_true, if_false);

    Monkey {
        items,
        operation,
        becomes_bored,
        divisor,
        divisibility_test,
        inspection_count: 0,
    }
}

/// An item-throwing, mischief-loving monkey.
struct Monkey<'a> {
    /// The list of items the monkey currently holds. It will inspect them in 
    /// order.
    items: Vec<WorryValue>,
    /// The operation to perform on an item's worry number when it's being 
    /// inspected.
    operation: Operation<'a>,
    /// Whether or not the monkey gets bored after inspecting an item, which 
    /// means the worry value of an item gets divided by 3 after being inspected.
    becomes_bored: bool,
    // The divisor used in the divisibility test.
    divisor: u64,
    /// The divisibility test, used to determine to which other monkey this 
    /// monkey will throw the currently held item to.
    divisibility_test: DivisibilityTest<'a>,
    /// The number of times this monkey has inspected an item.
    inspection_count: usize,
}

impl<'a> Monkey<'a> {
    /// Perform a turn with the monkey, returning an iterator over the items it 
    /// throws, alongside their respective targets.
    fn turn(
        &mut self,
        global_modulo: WorryValue,
    ) -> impl Iterator<Item = (MonkeyId, WorryValue)> + '_ {
        // Firstly, increase the inspection count by the number of items it has,
        // since the monkey inspects all of its items.
        self.inspection_count += self.items.len();

        // Then, taking every single item from the monkey's inventory, in order 
        // of inspection...
        self.items
            .drain(..)
            // ...perform the worry value operation...
            .map(&self.operation)
            // ...divide the worry value by 3 if the monkey gets bored...
            .map(|v| if self.becomes_bored { v / 3 } else { v })
            // ...modulo the value with the global modulo, which is the product 
            // of the divisors of all the monkeys
            .map(move |v| v % global_modulo)
            // ...and perform the divisibility test, yielding the worry value 
            // and its target.
            .map(|v| (self.divisibility_test)(v, self.divisor))
    }
}

// Perform a given number of rounds of monkey business, by parsing the input and 
// running turns on the monkeys. We also get the boolean that says whether 
// monkeys get bored or not.
fn perform_rounds(input: &str, rounds: usize, becomes_bored: bool) -> MonkeyId {
    // Split the input by double line breaks and parse each resulting string as 
    // a monkey. We store them as Options because during a round, we will need 
    // to mutate both the current monkey and another monkey. We can't statically 
    // prove that those two monkeys will never be the same, so in order to 
    // satisfy borrow checking, during each turn, we will `take()` the current 
    // monkey and store it back when we're done.
    let mut monkeys = input
        .split("\n\n")
        .map(|input| Some(parse_monkey(input, becomes_bored)))
        .collect::<Vec<_>>();

    // Calculate the global modulo applied to worry values, which is the product 
    // of the divisors of each monkey. This works because of ✨ maths ✨: you 
    // can clamp down the worry value down to the product of the divisors so 
    // that you won't keep multiplying everything up to overflowing, and you 
    // don't mess up the division tests.
    let global_modulo = monkeys
        .iter()
        .map(|m| m.as_ref().unwrap().divisor)
        .product::<WorryValue>();

    // For each round:
    for _ in 0..rounds {
        // Iterate over the monkeys.
        for idx in 0..monkeys.len() {
            // Take the current monkey out of the array, but keep a None in its 
            // place, so that re-inserting it is O(1). This way, we can mutate 
            // both the monkey and the array.
            let mut monkey = monkeys[idx].take().unwrap();

            // Perform the turn of the monkey, and for each item thrown...
            for (recipient, item) in monkey.turn(global_modulo) {
                // ...add it to the recipient monkey's item list, at the end.
                monkeys[recipient].as_mut().unwrap().items.push(item);
            }

            // Put the monkey back in its place.
            monkeys[idx] = Some(monkey);
        }
    }

    // Now it's time to calculate the level of monkey business going on. For 
    // each monkey...
    monkeys
        .iter()
        // ...take the number of time it has inspected an item...
        .map(|m| m.as_ref().unwrap().inspection_count)
        // ...sort these counts...
        .sorted()
        // ...reverse the iterator so that we consider the biggest ones first...
        .rev()
        // ...take the first two, which are the biggest two by virtue of the 
        // sort...
        .take(2)
        // ...and multiply them together.
        .product::<usize>()
}

fn part_1(input: &str) {
    // In the first part, we perform 20 rounds, and the monkeys get bored.

    let monkey_business = perform_rounds(input, 20, true);

    println!("Part 1: {monkey_business}");
}

fn part_2(input: &str) {
    // In the second part, we perform 10,000 rounds and the monkeys do not get 
    // bored anymore.

    let monkey_business = perform_rounds(input, 10000, false);

    println!("Part 2: {monkey_business}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
