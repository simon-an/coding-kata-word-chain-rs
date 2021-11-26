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

    let solver = Solver::default();

    let res = solver.solve(start, end)?;
    println!("from {} to {} in {} words", start, end, res);

    Ok(())
}

struct Solver {
    wordlist: Vec<&'static str>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            wordlist: load_wordlist(),
        }
    }
}
impl Solver {
    fn solve(&self, start: &str, end: &str) -> Result<u64, &'static str> {
        // implement this

        Ok(0)
    }
}

fn load_wordlist() -> Vec<&'static str> {
    include_str!("../resources/wordlist.txt")
        .split("\n")
        .collect()
}

#[cfg(test)]
mod tests {

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
        assert_eq!(2, res)
    }
    #[test]
    fn test_from_lead_to_gold() {
        let start = "lead";
        let end = "gold";

        let solver = super::Solver::default();

        let res = solver.solve(start, end).unwrap();
        assert_eq!(2, res)
    }
    #[test]
    fn test_from_ruby_to_code() {
        let start = "ruby";
        let end = "code";
        let solver = super::Solver::default();

        let res = solver.solve(start, end).unwrap();
        assert_eq!(4, res)
    }
}
