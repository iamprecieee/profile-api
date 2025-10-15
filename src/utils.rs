use crate::{dtos::CatFactResponse, DEFAULT_CAT_FACT};

pub async fn get_random_cat_fact(cat_fact_api: String) -> String {
    let response = reqwest::get(format!("{}", cat_fact_api)).await;

    match response {
        Ok(res) => {
            let json_result = res.json::<CatFactResponse>().await;
            match json_result {
                Ok(cat_fact) => cat_fact.fact,
                Err(_) => String::from(DEFAULT_CAT_FACT),
            }
        }
        Err(_) => String::from(DEFAULT_CAT_FACT),
    }
}
