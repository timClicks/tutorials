use uuid;

#[derive(Debug)]
struct Account {
    id: uuid::Uuid,
    user_name: String,
}

impl Account {
    fn to_json(&self) -> String {
        format!(r#"{{ "id": "{:?}", "user_name": {:?} }}"#, self.id, self.user_name)
    }
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "tim".to_string(),
    };
    println!("{}", acct.to_json())
}