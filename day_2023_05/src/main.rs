// if you're reading this version of the day 5 solution: i'm sorry

use itertools::Itertools;

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

fn part_1(input: &str) {
    let (_, initial_seeds) = input.lines().next().unwrap().split_once(':').unwrap();
    let initial_seeds = initial_seeds
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let result = input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let mut line = line
                        .split_ascii_whitespace()
                        .map(|v| v.parse::<u64>().unwrap());

                    (
                        line.next().unwrap(),
                        line.next().unwrap(),
                        line.next().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .fold(initial_seeds, |mut locations, map| {
            for l in &mut locations {
                *l = map
                    .iter()
                    .find_map(|(destination_start, source_start, length)| {
                        if (*source_start..=*source_start + length).contains(l) {
                            Some(*l - source_start + *destination_start)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(*l);
            }

            locations
        })
        .into_iter()
        .min()
        .unwrap();

    println!("{result}");
}

fn part_2(input: &str) {
    let (_, initial_seeds) = input.lines().next().unwrap().split_once(':').unwrap();
    let initial_seeds = initial_seeds
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .tuples()
        .collect::<Vec<(u64, u64)>>();

    let result = input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|v| v.parse::<u64>().unwrap())
                        .tuples()
                        .next()
                        .unwrap()
                })
                .collect::<Vec<(u64, u64, u64)>>()
        })
        .fold(initial_seeds, |locations, maps| {
            println!("\n== next step ==\n");

            locations.into_iter().flat_map(|(start, length)| {
                let (untranslated, transalted) = maps.iter()
                    .filter(|(_, source_start, map_length)| {
                        println!("checking overlap between ({start}, {length}) and ({source_start}, {map_length})");
                        (start..=start + length).contains(source_start)
                            || (start..=start + length).contains(&(source_start + map_length)) 
                            || ((*source_start <= start) && (source_start + map_length) >= (start + length))
                    })
                    .fold(
                        (vec![(start, length)], vec![]),
                        |(ranges, translated), (destination_start, source_start, map_length)| {
                            println!("mapping ranges {ranges:?} with map ({destination_start}, {source_start}, {map_length}).");
                            ranges
                                .into_iter()
                                .map(|(start, length)| {
                                    print!("  ({start}, {length}) with ({destination_start}, {source_start}, {map_length}): ");
                                    if start <= *source_start
                                        && (start + length) >= (source_start + map_length)
                                    {
                                        println!("completely containing.");
                                        // complete overlap
                                        (vec![(start, *source_start - start - 1),(
                                            source_start + map_length + 1,
                                            (start + length) - (source_start + map_length + 1),
                                        )], vec![(*destination_start, *map_length),])
                                      
                                    } else if *source_start <= start && (source_start + map_length) >= start + length {
                                        println!("completely contained.");
                                        (vec![], vec![(destination_start + (start - source_start), length)])
                                    
                                    } else if (start..=start + length).contains(source_start) {
                                        println!("right overlap.");
                                        // overlap right
                                        (vec![
                                            (start, *source_start - start - 1),
                                            
                                        ], vec![(
                                            *destination_start,
                                            (start + length) - *source_start,
                                        ),])
                                    } else if (start..=start + length)
                                        .contains(&(source_start + map_length))
                                    {
                                        println!("left overlap.");
                                        // overlap left
                                        (vec![
                                            
                                            (
                                                source_start + map_length + 1,
                                                (start + length) - (source_start + map_length + 1),
                                            ),
                                        ], vec![(
                                            destination_start + (start - source_start),
                                            source_start + map_length - start,
                                        ),])
                                    } else {
                                        println!("no overlap.");
                                        (vec![], vec![(start, length)])
                                    }
                                })
                                .inspect(|(unfinished, finished)| println!("-> resulting ranges: ({unfinished:?}, {finished:?})"))
                                .fold((vec![], translated), |(mut a, mut b), (c, d)| {
                                    a.extend(c);
                                    b.extend(d);
                                    (a, b)
                                })
                        },
                    );
                    untranslated.into_iter().chain(transalted)
            }).collect()

        })
        .into_iter()
        .min_by_key(|(start, _)| *start)
        .unwrap()
        .0;

    println!("{result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
