struct Laptops{
    brand:String,
    price:i32,
    quantity:i32
}
impl Laptops{
    fn total_cost(&self, order_quantity:i32) -> i32{
       self.price * order_quantity
    }
}

fn main() {
    let cos1 = Laptops{
        brand:String::from("HP"),
        price: 650_000,
        quantity: 10,

    };
    let cos2 = Laptops{
        brand:String::from("IBM"),
        price: 755_000,
        quantity: 6,
        
    };
    let cos3 = Laptops{
        brand:String::from("Toshiba"),
        price: 550_000,
        quantity: 10,
        
    };
    let cos4 = Laptops{
        brand:String::from("Dell"),
        price: 850_000,
        quantity: 4,
        
    };
    let mut inventory :Vec<Laptops> = Vec::new();

    inventory.push(cos1);
    inventory.push(cos2);
    inventory.push(cos3);
    inventory.push(cos4);
    
    let mut total_price = 0;

    for i in &inventory{
        let order_quantity = 3;
        let remaining_quantity = i.quantity - order_quantity;
        let  cost  = i.total_cost(order_quantity);
        println!(
            "You ordered {} of {} laptops and your total cost is: {}",
           order_quantity, i.brand, cost
        );
        println!("You have {} units of {} left", remaining_quantity, i.brand);
        total_price+=cost;
    }
    println!("Total cost for ordering 3 pieces of each laptop: {}", total_price);
    // println!("{:?}", inventory);
}