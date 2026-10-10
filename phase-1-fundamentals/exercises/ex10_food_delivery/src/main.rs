#![allow(dead_code)]

#[derive(Debug)]
enum MealType {
    Burgers,
    Fufu,
    Waakye,
    Pizza,
}

#[derive(Debug)]
enum OrderStatus {
    Placed(MealType, f32),
    KitchenPreparing(i32),
    OutForDelivery(String),
    Delivered,
}

// Helper Functions 
fn order_placed(meal: MealType, price: f32) -> OrderStatus {
    OrderStatus::Placed(meal, price)
}

fn kitchen_preparing(minutes: i32) -> OrderStatus {
    OrderStatus::KitchenPreparing(minutes)
}

fn out_for_delivery(driver_name: String) -> OrderStatus {
    OrderStatus::OutForDelivery(driver_name)
}

fn order_delivered() -> OrderStatus {
    OrderStatus::Delivered
}

fn main() {
    println!("=======================================");
    println!("       FOOD TRACKER NOTIFICATIONS      ");
    println!("=======================================");

    // Step 1: Placed
    let status_1 = order_placed(MealType::Fufu, 125.50);
    print_notification(&status_1);

    // Step 2: Preparing
    let status_2 = kitchen_preparing(40);
    print_notification(&status_2);

    // Step 3: Out for delivery
    let driver = String::from("Albert Quaye");
    let status_3 = out_for_delivery(driver);
    print_notification(&status_3);
    
    // Step 4: Delivered
    let status_4 = order_delivered();
    print_notification(&status_4);
}

// Uses pattern matching to print beautiful alerts based on the order status
fn print_notification(status: &OrderStatus) {
    match status {
        OrderStatus::Placed(meal, price) => {
            println!("🎉 Order received! A delicious plate of {:?} has been placed for GHC {:.2}.", meal, price);
        }
        OrderStatus::KitchenPreparing(mins) => {
            println!("🍳 The chef is on it! Your meal is being prepared and will be ready in {} minutes.", mins);
        }
        OrderStatus::OutForDelivery(driver_name) => {
            println!("🛵 Out for delivery! Your rider, {}, is on his way to your location.", driver_name);
        }
        OrderStatus::Delivered => {
            println!("✅ Delivered! Your food has arrived. Enjoy your meal!");
        }
    }
}
