use serde::{Deserialize, Serialize};
use crate::models::options::Options;

#[derive(Debug, Deserialize, Serialize)]
pub struct Returns {
    #[serde(flatten)]
    pub option: Options,
    pub mid: f64,
    pub net: f64,
    pub premium: f64,
    pub insurance: f64,
    pub return_after_1_div: Option<f64>,
    pub return_after_2_div: Option<f64>,
    pub return_after_3_div: Option<f64>,
    pub return_after_4_div: Option<f64>,
    pub return_after_5_div: Option<f64>,
    pub return_after_6_div: Option<f64>
}
