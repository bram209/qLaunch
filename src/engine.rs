use applications::Application;
use applications;



#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub exec: String
}

pub trait SearchEngine {
    fn search(&self, query: &str) -> Box<Iterator<Item = SearchResult>>;
//    fn execute_result(&self, result: &SearchResult) -> ExecutionResult;
}

pub struct ApplicationSearcher {
    applications: Vec<Application>
}

impl ApplicationSearcher {
    pub fn new() -> ApplicationSearcher {
        ApplicationSearcher {
            applications: applications::read_applications()
        }
    }
}

impl SearchEngine for ApplicationSearcher {
    fn search(&self, query: &str) -> Box<Iterator<Item = SearchResult>> {
        let mut results: Vec<SearchResult> = vec![];
        let query = &query.to_lowercase();
        for  app in &self.applications {
            let app = app.clone();
            if app.name.to_lowercase().contains(query) {
                results.push(SearchResult {
                    name: app.name,
                    exec: app.exec
                });
            }
        }
        Box::new(results.into_iter())
    }

//    fn execute_result(&self, result: &SearchResult) -> ExecutionResult {
//        unimplemented!()
//    }
}

pub fn edit_distance(a: &str, b: &str) -> i32 {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<i32> = vec![0; len_b + 1];
    let mut matrix: Vec<Vec<i32>> = vec![row; len_a + 1];

    // initialize string a
    for i in 0..len_a {
        matrix[i + 1][0] = matrix[i][0] + 1;
    }

    // initialize string b
    for i in 0..len_b {
        matrix[0][i + 1] = matrix[0][i] + 1;
    }

    // calculate matrix
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let alternatives = [
                // deletion
                matrix[i][j + 1] + 1,
                // insertion
                matrix[i + 1][j] + 1,
                // match or substitution
                matrix[i][j] + if ca == cb { 0 } else { 1 }];
            matrix[i + 1][j + 1] = *alternatives.iter().min().unwrap();
        }
    }

    matrix[len_a][len_b]
}
