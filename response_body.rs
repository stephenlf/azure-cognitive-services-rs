
use serde::Deserialize;
use url::OpaqueOrigin;
use super::parse_csv::Csv;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GetAnalyzeLayoutResult {
    pub status: Option<OkDetail>,
    created_date_time: Option<String>,
    last_updated_date_time: Option<String>,
    analyze_result: Option<Box<AnalyzeResult>>, 
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum OkDetail {
    NotStarted,
    Running,
    Failed,
    Succeeded,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AnalyzeResult {
    version: Option<String>,
    page_results: Option<Vec<PageResult>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
struct PageResult {
    page: Option<usize>,
    tables: Option<Vec<Table>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Table {
    rows: Option<usize>,
    columns: Option<usize>,
    cells: Option<Vec<Cells>>,
}


#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Cells {
    row_index: Option<usize>,
    column_index: Option<usize>,
    text: Option<String>,
    column_span: Option<usize>,
    is_header: Option<bool>,
}

impl GetAnalyzeLayoutResult {
    pub fn get_status(&self) -> &Option<OkDetail> {
        &self.status
    }
    pub fn as_csvs(&self) -> Vec<Csv> {
        self
            .analyze_result.as_ref().expect("Could not find analyzeResult field")
            .as_csvs()
    }
}

impl Table {
    pub fn as_csv(&self) -> Csv {   
        let mut csv = Csv::init(
            self.columns.expect("Could not find columns field"), 
            self.rows.expect("Could not find rows field"), 
            '\t');

        for cell in self.cells.as_ref().expect("Could not find cells field") {
            let column = cell.column_index.expect("Could not find columnIndex field");
            let row = cell.row_index.expect("Could not find rowIndex field");
            let content = cell.text.as_ref().expect("Could not find text field");
            csv.write_cell(column, row, content.to_string());
        }
        
        csv
    }
}

impl PageResult {
    fn as_csvs(&self) -> Vec<Csv> {
        let mut csvs: Vec<Csv> = Vec::new();

        let tables = self
            .tables.as_ref().expect("Could not find tables field");

        for table in tables {
            csvs.push(table.as_csv());
        }
        csvs
    }
}

impl AnalyzeResult {
    fn as_csvs(&self) -> Vec<Csv> {
        let mut csvs: Vec<Csv> = Vec::new();

        let pages = self
            .page_results.as_ref().expect("Could not find pageResults field");

        for page in pages {
            for csv in page.as_csvs() {
                csvs.push(csv);
            }
        }

        csvs
    }
}

#[cfg(test)]
mod response_body_tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;
    use serde_json;

    fn load_response() -> GetAnalyzeLayoutResult {
        let raw_response = fs::File::open(PathBuf::from("__tempfolder__/example_response.json")).unwrap();
        let response: super::GetAnalyzeLayoutResult = serde_json::de::from_reader(raw_response).unwrap();
        response
    }

    #[test]
    fn test_load_response() {
        println!("{:?}",load_response());
    }

    #[test]
    fn test_as_csv() {
        let response = load_response();
        let csvs = response.as_csvs();
        for csv in csvs {
            println!("{}\n----------------\n",csv.pretty());
        }
    }
}