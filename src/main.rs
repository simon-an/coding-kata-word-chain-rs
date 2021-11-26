use std::time::Instant;

use clap::App;

fn main() -> Result<(), &'static str> {
    let matches = App::new("word chain")
        .version("1.0")
        .author("Simon P <simon-an@github.com>")
        .arg("<Start>            'First word")
        .arg("<End>              'Last word'")
        .get_matches();

    let start = matches.value_of("Start").unwrap();
    let end = matches.value_of("End").unwrap();

    assert_eq!(start.len(), end.len());

    let solver = Solver::default();

    let res = solver.solve(start, end)?;
    println!(
        "from {} to {} in {} words: {:?}",
        start,
        end,
        res.len(),
        res
    );

    Ok(())
}

struct Solver {
    wordlist: Vec<String>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            wordlist: load_wordlist(),
        }
    }
}
impl Solver {
    fn solve(&self, start: &str, end: &str) -> Result<Vec<String>, &'static str> {
        // implement this
        let time = Instant::now();
        let mut result: Vec<String> = vec![];

        println!("solved in {}s", time.elapsed().as_secs_f64());
        Ok(result)
    }
    fn find_next_word(
        &self,
        results: &Vec<String>,
        end: &str,
        len: usize,
        blacklist: &Vec<String>,
    ) -> Option<Vec<String>> {

        Some(vec![])
        
    }
}

fn load_wordlist() -> Vec<String> {
    let s = include_str!("../resources/wordlist.txt");
    let s = newline_converter::dos2unix(s);
    let s = s.into_owned();
    let vec: Vec<String> = s.split("\n").map(|s: &str| s.to_string()).collect();
    vec
}

#[cfg(test)]
mod tests {
    use crate::load_wordlist;

    #[test]
    fn test_word_list() {
        let list = load_wordlist();
        assert!(list.contains(&"ruby".to_string()));
        assert!(list.contains(&"rubs".to_string()));
        assert!(list.contains(&"robs".to_string()));
        assert!(list.contains(&"rode".to_string()));
    }

    #[test]
    fn test_find_next_word() {
        let solver = super::Solver::default();
        let res = solver
            .find_next_word(&vec!["ruby".to_string()], "robs", 4, &vec![])
            .unwrap();
        assert_eq!(vec!["ruby".to_string(), "rubs".to_string()], res);
    }
    #[test]
    fn test_find_next_word_2() {
        let solver = super::Solver::default();
        let res = solver
            .find_next_word(
                &vec!["ruby".to_string(), "rube".to_string()],
                "code",
                4,
                &vec!["cube".to_string()],
            )
            .unwrap();
        assert_eq!(
            vec!["ruby".to_string(), "rube".to_string(), "robe".to_string()],
            res
        );
        let res = solver
            .find_next_word(
                &vec!["ruby".to_string(), "rube".to_string(), "robe".to_string()],
                "code",
                4,
                &vec!["cube".to_string()],
            )
            .unwrap();
        assert_eq!(
            vec![
                "ruby".to_string(),
                "rube".to_string(),
                "robe".to_string(),
                "rode".to_string()
            ],
            res
        );
    }

    #[test]
    fn test_load_world_list() {
        assert_eq!(338882, super::load_wordlist().len());
    }
    #[test]
    fn test_from_cat_to_dog() {
        let start = "cat";
        let end = "dog";

        let solver = super::Solver::default();

        let res = solver.solve(start, end).unwrap();

        // assert_eq!(4, res.len());
        assert_eq!(
            vec![
                "cat".to_string(),
                "cot".to_string(),
                "cog".to_string(),
                "dog".to_string()
            ],
            res
        );
    }
    #[test]
    fn test_from_lead_to_gold() {
        let start = "lead";
        let end = "gold";

        let solver = super::Solver::default();

        let res = solver.solve(start, end).unwrap();
        assert_eq!(
            vec![
                "lead".to_string(),
                "load".to_string(),
                "goad".to_string(),
                "gold".to_string()
            ],
            res
        )
    }
    #[test]
    fn test_from_ruby_to_code() {
        let start = "ruby";
        let end = "code";
        let solver = super::Solver::default();

        let res = solver.solve(start, end).unwrap();
        assert_eq!(5, res.len());
        assert_eq!(
            vec![
                "ruby".to_string(),
                "rube".to_string(),
                "robe".to_string(),
                "rode".to_string(),
                "code".to_string()
            ],
            res
        );
    }
}
