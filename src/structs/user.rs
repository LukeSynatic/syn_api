use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    // _id: {
    // "$oid": "6476f8efb10d78cf654413f6"
    // },
    id: String,
    deactivated: bool,
    first_name: String,
    last_name: String,
    job_title: String,
    avatar: String,
    // birthday: {
    // $date": "2022-12-19T13:35:38.421Z"
    // },
    street_address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
    phone_number: String,
    email: String,
    favorite_color: String,
    account_balance: u64
}