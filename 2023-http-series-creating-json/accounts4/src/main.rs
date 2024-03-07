use uuid;
use serde::Serialize;
use std::collections::HashMap;
// use serde_json::json;

#[derive(Debug)]
struct Account {
    id: uuid::Uuid,
    user_name: String,
    last_login_at: Option<String>,
}

impl Account {
    fn to_json(&self) -> String {
        let mut out: HashMap<&'static str, String> = HashMap::new();
        out.insert("id", self.id.to_string());
        out.inser("user_name", self.user_name.clone());
        if let Some(ref login) = self.last_login_at {
            out.insert("last_login_at",  login.to_string());
        }

        serde_json::to_string(&out).unwrap()

        // json!({
        //     "id": self.id.to_string(),
        //     "user_name": self.user_name,
        //     // uh oh..
        // }).to_string()
        // format!(r#"{{ "id": "{:?}", "user_name": {:?} }}"#, self.id, self.user_name)
    }
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "tim".to_string(),
        last_login_at: Some("2023-07-01".to_string()),
    };
    println!("{}", acct.to_json())
}