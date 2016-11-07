use applications::Application;
use applications;



#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub exec: String,
    pub icon_path: Option<String>
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
        let mut application_reader = applications::ApplicationReader::new();
        ApplicationSearcher {
            applications: application_reader.read_applications()
        }
    }
}

impl SearchEngine for ApplicationSearcher {
    fn search(&self, query: &str) -> Box<Iterator<Item = SearchResult>> {
        let mut results: Vec<SearchResult> = vec![];
        let query = &query.to_lowercase();
        for app in &self.applications {
            let app = app.clone();
            if app.name.to_lowercase().contains(query) {
                results.push(SearchResult {
                    name: app.name,
                    exec: app.exec,
                    icon_path: app.icon_path
                });
            }
        }
        Box::new(results.into_iter())
    }
}
