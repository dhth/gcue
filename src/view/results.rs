use serde_json::Value;
use tabled::builder::Builder;
use tabled::settings::style::Style;

pub fn get_results(results: &Value) -> Option<String> {
    let results_array = match results {
        Value::Array(arr) => Some(arr),
        Value::Object(obj) => match obj.get("results") {
            Some(Value::Array(arr)) => Some(arr),
            _ => None,
        },
        _ => None,
    };

    let results_array = results_array?;

    if results_array.is_empty() {
        return None;
    }

    let mut builder = Builder::default();

    if let Some(Value::Object(first)) = results_array.first() {
        let headers: Vec<String> = first.keys().cloned().collect();
        builder.push_record(&headers);

        for result in results_array {
            if let Value::Object(row) = result {
                let cells: Vec<String> = headers
                    .iter()
                    .map(|h| {
                        row.get(h)
                            .map(|v| match v {
                                Value::String(s) => s.clone(),
                                Value::Null => "null".to_string(),
                                _ => v.to_string(),
                            })
                            .unwrap_or_else(|| "".to_string())
                    })
                    .collect();
                builder.push_record(cells);
            }
        }
    }

    let mut table = builder.build();

    table.with(Style::psql());

    Some(table.to_string())
}
