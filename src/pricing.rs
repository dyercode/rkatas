// A store has the following pricing policy:
//
// Buy less than $100 worth of merchandise and pay the full price.
// Buy $100 or more, but less than $1000, and get 10% off.
// Buy $1000 or more, and get 15% off.
// According to the laws governing the store, the following taxes apply:
//
// Food: no tax.
// Alcohol: 7.5% sales tax + 8% "sin" tax.
// All other merchandise: 7.5% sales tax.
// Discounts are calculated on sticker prices; taxes are calculated on discounted prices.
//
// Using TDD, write a method that will accept purchased items of food, alcohol, and other, and will return the price the customer should be charged.

const TEN_PERCENT_OFF_FLOOR: f64 = 100.0;
const SALES_TAX_PERCENT: f64 = 0.075;
const SIN_TAX_PERCENT: f64 = 0.08;
const FIFTEEN_PERCENT: f64 = 0.15;
const TEN_PERCENT: f64 = 0.1;

fn discount(item: Item) -> Item {
    let discounted_price = if (item.price >= 1000.00) {
        item.price * (1.0 - FIFTEEN_PERCENT)
    } else if (item.price >= TEN_PERCENT_OFF_FLOOR) {
        item.price * (1.0 - TEN_PERCENT)
    } else {
        item.price
    };

    Item { price: discounted_price, product_type: item.product_type }
}

fn pricing(item: Item) -> f64 {
    calculate_tax(discount(item))
}

fn calculate_tax(item: Item) -> f64 {
    match item.product_type {
        ProductType::Food => item.price,
        ProductType::Alcohol => item.price * (1.0 + SALES_TAX_PERCENT + SIN_TAX_PERCENT),
        _ => item.price * (1.0 + SALES_TAX_PERCENT)
    }
}


#[test]
fn pay_full_price_under_100() {
    assert_eq!(pricing(Item { price: 99.99, product_type: ProductType::Food }), 99.99)
}

#[test]
fn ten_percent_off_over_100() {
    assert_eq!(pricing(Item { price: 110.00, product_type: ProductType::Food }), 99.00)
}

#[test]
fn ten_percent_off_on_100_exactly() {
    assert_eq!(pricing(Item { price: 100.00, product_type: ProductType::Food }), 90.00)
}

#[test]
fn fifteen_percent_off_on_1000() {
    assert_eq!(pricing(Item { price: 1000.00, product_type: ProductType::Food }), 850.00)
}

#[test]
fn fifteen_percent_off_above_1000() {
    assert_eq!(pricing(Item { price: 1500.00, product_type: ProductType::Food }), 1275.00)
}

#[test]
fn other_items_have_7_5_percent_sales_tax() {
    assert_eq!(pricing(Item { price: 10.00, product_type: ProductType::Other }), 10.75)
}

#[test]
fn food_items_must_not_be_taxed() {
    assert_eq!(pricing(Item { price: 10.00, product_type: ProductType::Food }), 10.00)
}

#[test]
fn alcohol_must_have_a_sin_tax() {
    assert_eq!(pricing(Item { price: 10.00, product_type: ProductType::Alcohol }), 11.55)
}

#[test]
fn discounts_on_sticker_tax_on_discounted_price() {
    assert_eq!(pricing(Item { price: 100.00, product_type: ProductType::Other }), 96.75)
}


struct Item {
    price: f64,
    product_type: ProductType,
}

enum ProductType {
    Other,
    Food,
    Alcohol,
}

