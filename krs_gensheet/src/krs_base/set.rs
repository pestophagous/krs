#[derive(Clone)]
pub struct Item {
    pub unique_id: String,
    pub answer: String,
    pub tags: Vec<String>,
    pub prompt: String,
}

pub struct Set {
    ordered_items: Vec<Item>,
}

impl Set {
    pub fn new() -> Set {
        let mut the_set = Set {
            ordered_items: Vec::new(),
        };

        let fake_item = Item {
            unique_id: "a844695".to_owned(),
            answer: "x=1, x=-1".to_owned(),
            tags: [
                "gcf".to_owned(),
                "factoring".to_owned(),
                "solve-for-x".to_owned(),
            ]
            .into(),
            prompt: "$$ 4x^2 - 4 = 0 $$".to_owned(),
        };
        the_set.ordered_items.push(fake_item);

        the_set
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.ordered_items.clone()
    }
}
