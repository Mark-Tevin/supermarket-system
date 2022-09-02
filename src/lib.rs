use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::Promise;
use near_sdk::{env, near_bindgen};


#[warn(dead_code)]
fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

// smart contract helper method convert string to enum
fn get_category(category: String) -> Categories {
    if category.eq(&"food".to_owned()) {
        Categories::FOOD
    } else if category.eq(&"house_hold".to_owned()) {
        Categories::HOUSEHOLD
    } else if category.eq(&"PreparedFood".to_owned()) {
        Categories::PreparedFood
    } else if category.eq(&"groceries".to_owned()) {
        Categories::GROCERIES
    } else if category.eq(&"toileteries".to_owned()) {
        Categories::TOILETRIES
    } else if category.eq(&"snacks".to_owned()) {
        Categories::SNACKS
    } else {
        Categories::OTHERS
    }
}

// superamket system
//   the supermaket contains products, the properties of a product are as followes
//     -> name
//     -> price
//     -> catgory
//     -> serial number
//     -> quantity
//     -> date_bought


// Categories are  groups into whichh products can be placed
#[derive(BorshDeserialize, BorshSerialize)]
enum Categories {
    FOOD,
    HOUSEHOLD,
    PreparedFood,
    GROCERIES,
    SNACKS,
    BreadAndBreadSpreads,
    SkinCareProducts,
    TOILETRIES,
    OTHERS,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Products {
    name: String,
    price: u128,
    category: Categories,
    quantity: i16,
    date_bought: u64,
}

// a map with product_id  => product details {captured above ie price name....}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Supermaket {
    products: UnorderedMap<u8, Products>,
    
}

impl Default for Supermaket {
    fn default() -> Self {
        Supermaket {
            products: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

#[near_bindgen]
impl Supermaket {
    // supermaket buys products from wholes / manufacturers
    pub fn buy(&mut self, name: String, price: u128, category: String, quantity: i16, date: u64) {
        let random = env::random_seed();

        let pr = Products {
            name: name,
            price: price,
            category: get_category(category),
            quantity: quantity,
            date_bought: date,
        };

        self.products.insert(&random[0], &pr);
    }

    #[payable]
    pub fn sell(&mut self, product_id: u8, quantity: i16) -> String {
        let current_user = env::signer_account_id();
        let deposit = env::attached_deposit();

        let product = self.products.get(&product_id);

        match product {
            Some(pr) => {
                let total_cost = pr.price * quantity as u128;

                assert!(deposit >= total_cost, "attached deposit not enough");

                Promise::new(current_user).transfer(total_cost);

                "ok".to_string()
            }
            None => {
                env::panic_str("The product id does not exist");
            }
        }
    }

    // pub fn get_product(&self) -> &near_sdk::collections::Vector<Products> {
    //     self.products.values_as_vector()
    // }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{ VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    pub fn buy() {
        let mut app = Supermaket::default();
        app.buy(
            "tissue".to_string(),
            one_near(),
            "toileteries".to_string(),
            5,
            env::block_timestamp(),
        );

        assert_eq!(app.products.len(), 1)
    }

    #[test]
    pub fn sell() {
        let user = AccountId::new_unchecked("tevin.testnet".to_string());
        let mut _context = get_context(user.clone());
        let bal = one_near() * 10;
    _context.attached_deposit(bal);
        _context.account_balance(bal);
        testing_env!(_context.build());

        let mut app = Supermaket::default();
        app.buy(
            "tissue".to_string(),
            one_near(),
            "toileteries".to_string(),
            5,
            env::block_timestamp(),
        );        
        assert_eq!(app.products.len(), 1);

        let tmp= app.products.keys_as_vector().get(0);

        let result = app.sell(tmp.unwrap(), 1);

        assert_eq!("ok".to_string(), result)
    }
}
