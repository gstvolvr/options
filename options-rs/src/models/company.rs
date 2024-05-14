use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Company {
		pub symbol: String,
	  pub company_name: String,
	  pub exchange: String,
	  pub industry: String,
	  pub website: String,
	  pub description: String,
	  pub c_e_o: String,
	  pub security_name: String,
	  pub issue_type: String,
	  pub sector: String,
	  primary_sic_code: String,
	  employees: String,
	  tags: String,
	  address: String,
	  address2: String,
	  state: String,
	  city: String,
	  zip: String,
	  country: String,
	  phone: String
}
