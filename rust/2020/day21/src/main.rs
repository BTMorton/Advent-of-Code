use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[derive(Debug, PartialEq)]
struct Recipe {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    static TEST_INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    fn get_test_input() -> Vec<Recipe> {
        parse_input(TEST_INPUT)
    }

    #[test]
    fn should_parse_a_recipe_line() {
        assert_eq!(
            Recipe {
                ingredients: vec![
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string()
                ],
                allergens: vec!["dairy".to_string(), "fish".to_string()]
            },
            parse_recipe("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)")
        );
        assert_eq!(
            Recipe {
                ingredients: vec![
                    "sqjhc".to_string(),
                    "mxmxvkd".to_string(),
                    "sbzzf".to_string()
                ],
                allergens: vec!["fish".to_string()]
            },
            parse_recipe("sqjhc mxmxvkd sbzzf (contains fish)")
        );
    }

    #[test]
    fn should_find_ingredients_that_are_allergen_candidates() {
        assert_eq!(
            vec![
                ("dairy".to_string(), vec!["mxmxvkd".to_string()]),
                (
                    "soy".to_string(),
                    vec!["sqjhc".to_string(), "fvjkl".to_string()]
                ),
                (
                    "fish".to_string(),
                    vec!["mxmxvkd".to_string(), "sqjhc".to_string()]
                ),
            ]
            .into_iter()
            .collect::<HashMap<String, Vec<String>>>(),
            find_allergen_candidates(&get_test_input())
        );
    }

    #[test]
    fn should_count_how_many_times_ingredients_are_used() {
        assert_eq!(
            vec![
                ("mxmxvkd".to_string(), 3),
                ("kfcds".to_string(), 1),
                ("sqjhc".to_string(), 3),
                ("nhms".to_string(), 1),
                ("fvjkl".to_string(), 2),
                ("trh".to_string(), 1),
                ("sbzzf".to_string(), 2),
            ]
            .into_iter()
            .collect::<HashMap<String, usize>>(),
            count_ingredient_usage(&get_test_input())
        );
    }

    #[test]
    fn day21a_test() {
        assert_eq!(5, day21a(&get_test_input()))
    }

    #[test]
    fn day21b_test() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", day21b(&get_test_input()))
    }
}

fn count_ingredient_usage(recipes: &Vec<Recipe>) -> HashMap<String, usize> {
    let mut map = HashMap::<String, usize>::new();

    for recipe in recipes {
        for ingredient in recipe.ingredients.iter() {
            let added_count = map.entry(ingredient.clone()).or_insert(0);
            *added_count += 1;
        }
    }

    map
}

fn find_allergen_candidates(recipes: &Vec<Recipe>) -> HashMap<String, Vec<String>> {
    let mut candidates = HashMap::<String, Vec<String>>::new();

    for recipe in recipes {
        for allergen in recipe.allergens.iter() {
            let new_candidates = if !candidates.contains_key(allergen) {
                recipe.ingredients.clone()
            } else {
                candidates[allergen]
                    .iter()
                    .map(|ingredient| ingredient.clone())
                    .filter(|ingredient| recipe.ingredients.contains(ingredient))
                    .collect()
            };

            candidates.insert(allergen.clone(), new_candidates);
        }
    }

    candidates
}

fn parse_recipe(line: &str) -> Recipe {
    let mut parts = line.split("(contains");
    let ingredients = parts
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.trim().to_string())
        .collect();
    let allergens = parts
        .next()
        .unwrap()
        .replace(")", "")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    Recipe {
        ingredients: ingredients,
        allergens: allergens,
    }
}

fn parse_input(input: &str) -> Vec<Recipe> {
    input.lines().map(|line| parse_recipe(line)).collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day21a(recipes: &Vec<Recipe>) -> usize {
    let usage_counts = count_ingredient_usage(recipes);

    let allergen_opts: HashSet<String> = find_allergen_candidates(recipes)
        .values()
        .flat_map(|v| v.clone())
        .collect();

    usage_counts
        .iter()
        .filter(|&(ingredient, _)| !allergen_opts.contains(ingredient))
        .map(|(_, count)| count)
        .sum()
}

fn day21b(recipes: &Vec<Recipe>) -> String {
    let allergen_opts = find_allergen_candidates(recipes);
    let mut dangerous_ingredients = HashMap::<String, String>::new();

    while dangerous_ingredients.len() < allergen_opts.len() {
        for (allergen, opts) in allergen_opts.iter() {
            let filtered_opts: Vec<String> = opts
                .iter()
                .filter(|&ingredient| !dangerous_ingredients.contains_key(ingredient))
                .map(|ingredient| ingredient.clone())
                .collect();

            if filtered_opts.len() == 1 {
                dangerous_ingredients.insert(filtered_opts[0].clone(), allergen.clone());
            }
        }
    }

    let mut allergens: Vec<String> = allergen_opts.keys().map(|s| s.clone()).collect();
    allergens.sort();

    let sorted_ingredients: Vec<String> = allergens
        .into_iter()
        .map(|all| {
            dangerous_ingredients
                .iter()
                .find(|&(_, value)| value == &all)
                .unwrap()
                .0
                .clone()
        })
        .collect();

    sorted_ingredients.join(",")
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let recipes = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day21a(&recipes);
    println!(
        "Day 21A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day21b(&recipes);
    println!(
        "Day 21B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
