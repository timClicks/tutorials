use uuid;
use serde_json::json;
// use serde::Serialize;

#[derive(Debug)]
struct Account {
    id: uuid::Uuid,
    user_name: String,
}

impl Account {
    fn to_json(&self) -> String {
        json!({
            "id": self.id.to_string(),
            "user_name": self.user_name,
        }).to_string()
    }
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "tim".to_string(),
    };
    println!("{}", acct.to_json())
}