use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum ItemType {
    Book,
    Dvd,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    item_type: ItemType,
    price: u32,
    amount: u32,
}

impl Item {
    fn total_price(&self) -> u32 {
        self.amount * self.price
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    id: String,
    items: Vec<Item>,
}

impl Order {
    pub fn create() -> Order {
        let items = vec![
            Item {
                item_type: ItemType::Book,
                price: 199,
                amount: 5,
            },
            Item {
                item_type: ItemType::Dvd,
                price: 599,
                amount: 2,
            },
        ];
        Order {
            id: String::from("order-id"),
            items,
        }
    }

    pub fn total_price(&self) -> u32 {
        self.items.iter().map(|i| i.total_price()).sum()
    }
}
