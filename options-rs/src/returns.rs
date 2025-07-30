use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Returns {
    pub symbol: String,
    pub company_name: String,
    pub industry: String,
    pub last: f64,
    pub net: f64,
    pub strike_price: String,
    pub expiration_date: String,
    pub insurance: f64,
    pub premium: f64,
    pub dividend_quarterly_amount: f64,
    pub dividend_ex_date: String,
    pub return_after_1_div: f64,
    pub return_after_2_div: f64,
    pub return_after_3_div: f64,
    pub return_after_4_div: f64,
    pub return_after_5_div: f64,
    pub return_after_last_div: f64,
    pub bid: f64,
    pub mid: f64,
    pub ask: f64,
    pub previous_date: String,
}

impl Returns {
    pub fn to_firestore_document(&self, project_id: &str) -> Value {
        let doc_id = format!("{}_{}_{}",
                             self.symbol, self.expiration_date, self.strike_price
        );
        let serialized = serde_json::to_value(self).unwrap();
        let mut fields = Map::new();

        if let Value::Object(obj) = serialized {
            for (key, value) in obj {
                let key_clone = key.clone();
                fields.insert(
                    key,
                    match value {
                        Value::String(s) => {
                            if key_clone == "expiration_date" {
                                json!({"timestampValue": format!("{}T00:00:00Z", s)})
                            } else {
                                json!({"stringValue": s})
                            }
                        },
                        Value::Number(n) => {
                            if let Some(f) = n.as_f64() {
                                json!({"doubleValue": f})
                            } else {
                                json!({"doubleValue": 0.0})
                            }
                        },
                        _ => json!({"nullValue": null}),
                    }
                );
            }
        }
        json!({
            "name": format!("projects/{}/databases/(default)/documents/options_returns/{}", project_id, doc_id),
            "fields": fields
        })
    }

    pub fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.symbol.clone(),
            self.company_name.clone(),
            self.industry.clone(),
            self.last.to_string(),
            self.net.to_string(),
            self.strike_price.clone(),
            self.expiration_date.clone(),
            self.insurance.to_string(),
            self.premium.to_string(),
            self.dividend_quarterly_amount.to_string(),
            self.dividend_ex_date.clone(),
            self.return_after_1_div.to_string(),
            self.return_after_2_div.to_string(),
            self.return_after_3_div.to_string(),
            self.return_after_4_div.to_string(),
            self.return_after_5_div.to_string(),
            self.return_after_last_div.to_string(),
            self.bid.to_string(),
            self.mid.to_string(),
            self.ask.to_string(),
            self.previous_date.clone(),
        ]
    }
}
