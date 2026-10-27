// ☕ Coffee API: a tiny fake HTTP API for practising Chapter 6 (enums and pattern matching).
//
// There's no real network here. You type lines that look like HTTP requests
// (`GET /orders/1`) and the program answers like an API would (`200 #1 latte large ...`).
//
// Search for `todo!` to find your missions. The README explains each one.

// Unfinished missions leave unused variables around; this silences the noise.
#![allow(unused)]

use std::io::{self, BufRead, Write};

// ============================================================================
//  Domain: what the coffee shop sells
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Drink {
    Espresso,
    Latte,
    Tea,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

// An enum whose variants hold *different* data. Only `Ready` knows a counter number.
#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderStatus {
    Pending,
    Ready { counter: u8 },
    Cancelled,
}

#[derive(Debug)]
struct Order {
    id: u32,
    drink: Drink,
    size: Size,
    status: OrderStatus,
}

// ============================================================================
//  HTTP-ish layer: requests come in as a Route, answers go out as a Response
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Method {
    Get,
    Post,
    Delete,
}

// Every endpoint our API supports. Each variant carries exactly the data that endpoint needs.
#[derive(Debug, PartialEq)]
enum Route {
    Health,                                   // GET    /health
    Stats,                                    // GET    /stats
    ListOrders,                               // GET    /orders
    GetOrder(u32),                            // GET    /orders/{id}
    CreateOrder { drink: Drink, size: Size }, // POST   /orders {drink} {size}
    CancelOrder(u32),                         // DELETE /orders/{id}
    MarkReady { id: u32, counter: u8 },       // POST   /orders/{id}/ready {counter}
}

#[derive(Debug, PartialEq)]
enum Response {
    Ok(String),          // 200
    Created { id: u32 }, // 201
    NoContent,           // 204
    BadRequest(String),  // 400
    NotFound,            // 404
    Conflict(String),    // 409
}

// ============================================================================
//  Worked example: read this before starting Mission 2
// ============================================================================

impl Method {
    // Turns text into an enum. Returning `Option` is how we say "this might not work":
    // `Some(method)` if we recognise the text, `None` if we don't.
    fn parse(s: &str) -> Option<Method> {
        match s {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "DELETE" => Some(Method::Delete),
            _ => None, // `_` matches anything left over. Without it this match won't compile.
        }
    }
}

// ============================================================================
//  Missions 1 & 2: Drink and Size
// ============================================================================

impl Drink {
    // MISSION 1: price in cents. Espresso = 250, Latte = 400, Tea = 300
    fn price_cents(&self) -> u32 {
        match self {
            Drink::Latte => 400,
            Drink::Tea => 300,
            Drink::Espresso => 250,
        }
    }

    // MISSION 2: "espresso" / "latte" / "tea" -> Some(..), anything else -> None
    fn parse(s: &str) -> Option<Drink> {
        match s.trim() {
            "latte" => Some(Drink::Latte),
            "tea" => Some(Drink::Tea),
            "espresso" => Some(Drink::Espresso),
            _ => None,
        }
    }
}

impl Size {
    // MISSION 1: surcharge in cents. Small = 0, Medium = 50, Large = 100
    fn extra_cents(&self) -> u32 {
        match self {
            Size::Large => 100,
            Size::Medium => 50,
            Size::Small => 0,
        }
    }

    // MISSION 2: "small" / "medium" / "large" -> Some(..), anything else -> None
    fn parse(s: &str) -> Option<Size> {
        match s.trim() {
            "large" => Some(Size::Large),
            "medium" => Some(Size::Medium),
            "small" => Some(Size::Small),
            _ => None,
        }
    }
}

impl Order {
    fn total_cents(&self) -> u32 {
        self.drink.price_cents() + self.size.extra_cents()
    }

    // "#1 Latte Large $5.00 (pending)"
    fn summary(&self) -> String {
        let total = self.total_cents();
        format!(
            "#{} {:?} {:?} ${}.{:02} ({})",
            self.id,
            self.drink,
            self.size,
            total / 100,
            total % 100,
            self.status.describe()
        )
    }
}

// ============================================================================
//  Missions 3 & 4: turning a Response into something printable
// ============================================================================

impl Response {
    // MISSION 3: the HTTP status code (the numbers are in the comments on the enum above).
    fn status_code(&self) -> u16 {
        match self {
            Response::Ok(msg) => 200,
            Response::Created { id } => 201,
            Response::NoContent => 204,
            Response::BadRequest(msg) => 400,
            Response::NotFound => 404,
            Response::Conflict(msg) => 409,
        }
    }

    // MISSION 4: the response body. The README lists what each variant should say.
    fn body(&self) -> String {
        match self {
            Response::Ok(msg) => msg.to_string(),
            Response::Created { id } => format!("order {id} created"),
            Response::NoContent => String::new(),
            Response::BadRequest(msg) => format!("bad request: {msg}"),
            Response::NotFound => String::from("not found"),
            Response::Conflict(msg) => format!("conflict: {msg}"),
        }
    }
}

impl OrderStatus {
    // MISSION 4: "pending", "ready at counter 3", "cancelled"
    fn describe(&self) -> String {
        match self {
            OrderStatus::Ready { counter } => format!("ready at counter {counter}"),
            OrderStatus::Pending => String::from("pending"),
            OrderStatus::Cancelled => String::from("cancelled"),
        }
    }
}

// ============================================================================
//  Missions 5 to 8: the shop (our "database" plus request handlers)
// ============================================================================

struct Shop {
    orders: Vec<Order>,
    next_id: u32,
}

impl Shop {
    fn new() -> Shop {
        Shop {
            orders: Vec::new(),
            next_id: 1,
        }
    }

    fn create_order(&mut self, drink: Drink, size: Size) -> Response {
        let id = self.next_id;
        self.next_id += 1;
        self.orders.push(Order {
            id,
            drink,
            size,
            status: OrderStatus::Pending,
        });
        Response::Created { id }
    }

    fn list_orders(&self) -> Response {
        if self.orders.is_empty() {
            return Response::Ok(String::from("no orders yet"));
        }
        let mut lines = Vec::new();
        for order in &self.orders {
            lines.push(order.summary());
        }
        Response::Ok(lines.join("\n    "))
    }

    // Like `find_order` but gives you a *mutable* reference so you can change the order.
    fn find_order_mut(&mut self, id: u32) -> Option<&mut Order> {
        for order in &mut self.orders {
            if order.id == id {
                return Some(order);
            }
        }
        None
    }

    // Worked example: read this before Mission 6.
    fn mark_ready(&mut self, id: u32, counter: u8) -> Response {
        // `let ... else`: if the pattern doesn't match, run the else block (which must leave the function).
        let Some(order) = self.find_order_mut(id) else {
            return Response::NotFound;
        };

        match order.status {
            OrderStatus::Pending => {
                order.status = OrderStatus::Ready { counter };
                Response::Ok(format!("order {id} is ready at counter {counter}"))
            }
            // Pull the value out of the variant and give it a new name (`c`) in one step
            OrderStatus::Ready { counter: c } => {
                Response::Conflict(format!("order {id} is already ready at counter {c}"))
            }
            OrderStatus::Cancelled => Response::Conflict(format!("order {id} was cancelled")),
        }
    }

    fn stats(&self) -> Response {
        Response::Ok(format!(
            "{} orders, {} ready for pickup",
            self.orders.len(),
            self.ready_count()
        ))
    }

    // MISSION 5: return the order with this id, or None.
    fn find_order(&self, id: u32) -> Option<&Order> {
        todo!("Mission 5")
    }

    // MISSION 5: Ok(summary) if the order exists, NotFound otherwise.
    fn get_order(&self, id: u32) -> Response {
        todo!("Mission 5")
    }

    // MISSION 6: cancel an order. See the README for the rules.
    fn cancel_order(&mut self, id: u32) -> Response {
        todo!("Mission 6")
    }

    // MISSION 7: the router. Send each Route to the method that handles it.
    fn handle(&mut self, route: Route) -> Response {
        todo!("Mission 7")
    }

    // MISSION 8: how many orders are Ready? Use `if let`.
    fn ready_count(&self) -> usize {
        todo!("Mission 8")
    }
}

// ============================================================================
//  Parsing: provided, but worth a look once you're done
// ============================================================================

// Turns "POST /orders latte large" into Route::CreateOrder { drink: Latte, size: Large }.
//
// About `?` on an Option: `x?` means "if x is None, return None from this function right now;
// otherwise unwrap the Some". The book covers it in Chapter 9; here it saves a lot of nested matches.
fn parse_request(line: &str) -> Option<Route> {
    let mut words = line.split_whitespace();
    let method = Method::parse(words.next()?)?;
    let path = words.next()?;
    let arg1 = words.next(); // Option<&str>: the first word after the path, if there is one
    let arg2 = words.next();

    // "/orders/3/ready" -> ["orders", "3", "ready"]
    let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

    // Matching on a tuple *and* a slice at once. This is where pattern matching gets fun.
    match (method, segments.as_slice()) {
        (Method::Get, ["health"]) => Some(Route::Health),
        (Method::Get, ["stats"]) => Some(Route::Stats),
        (Method::Get, ["orders"]) => Some(Route::ListOrders),
        (Method::Get, ["orders", id]) => Some(Route::GetOrder(id.parse().ok()?)),
        (Method::Post, ["orders"]) => {
            let drink = Drink::parse(arg1?)?;
            let size = Size::parse(arg2?)?;
            Some(Route::CreateOrder { drink, size })
        }
        (Method::Delete, ["orders", id]) => Some(Route::CancelOrder(id.parse().ok()?)),
        (Method::Post, ["orders", id, "ready"]) => Some(Route::MarkReady {
            id: id.parse().ok()?,
            counter: arg1?.parse().ok()?,
        }),
        _ => None,
    }
}

// ============================================================================
//  The "server": reads requests from your keyboard (or a file) and prints responses
// ============================================================================

fn main() {
    let mut shop = Shop::new();
    println!("☕ Coffee API is running. Type a request like `GET /health` (Ctrl+D to quit).");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break; // end of input
        }
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let response = match parse_request(line) {
            Some(route) => shop.handle(route),
            None => Response::BadRequest(format!("could not understand `{line}`")),
        };
        println!("{} {}", response.status_code(), response.body());
    }
}

// ============================================================================
//  Tests: `cargo test mission_1` runs only Mission 1's tests, and so on.
//  You don't need to understand this part yet (that's Chapter 11).
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_1_drink_prices() {
        assert_eq!(Drink::Espresso.price_cents(), 250);
        assert_eq!(Drink::Latte.price_cents(), 400);
        assert_eq!(Drink::Tea.price_cents(), 300);
    }

    #[test]
    fn mission_1_size_extras() {
        assert_eq!(Size::Small.extra_cents(), 0);
        assert_eq!(Size::Medium.extra_cents(), 50);
        assert_eq!(Size::Large.extra_cents(), 100);
    }

    #[test]
    fn mission_2_parse_drink() {
        assert_eq!(Drink::parse("espresso"), Some(Drink::Espresso));
        assert_eq!(Drink::parse("latte"), Some(Drink::Latte));
        assert_eq!(Drink::parse("tea"), Some(Drink::Tea));
        assert_eq!(Drink::parse("mojito"), None);
    }

    #[test]
    fn mission_2_parse_size() {
        assert_eq!(Size::parse("small"), Some(Size::Small));
        assert_eq!(Size::parse("medium"), Some(Size::Medium));
        assert_eq!(Size::parse("large"), Some(Size::Large));
        assert_eq!(Size::parse("venti"), None);
    }

    #[test]
    fn mission_3_status_codes() {
        assert_eq!(Response::Ok(String::from("hi")).status_code(), 200);
        assert_eq!(Response::Created { id: 7 }.status_code(), 201);
        assert_eq!(Response::NoContent.status_code(), 204);
        assert_eq!(Response::BadRequest(String::from("x")).status_code(), 400);
        assert_eq!(Response::NotFound.status_code(), 404);
        assert_eq!(Response::Conflict(String::from("x")).status_code(), 409);
    }

    #[test]
    fn mission_4_bodies() {
        assert_eq!(Response::Ok(String::from("hello")).body(), "hello");
        assert_eq!(Response::Created { id: 7 }.body(), "order 7 created");
        assert_eq!(Response::NoContent.body(), "");
        assert_eq!(
            Response::BadRequest(String::from("no milk")).body(),
            "bad request: no milk"
        );
        assert_eq!(Response::NotFound.body(), "not found");
        assert_eq!(
            Response::Conflict(String::from("too late")).body(),
            "conflict: too late"
        );
    }

    #[test]
    fn mission_4_describe_status() {
        assert_eq!(OrderStatus::Pending.describe(), "pending");
        assert_eq!(
            OrderStatus::Ready { counter: 3 }.describe(),
            "ready at counter 3"
        );
        assert_eq!(OrderStatus::Cancelled.describe(), "cancelled");
    }

    #[test]
    fn mission_5_find_and_get_order() {
        let mut shop = Shop::new();
        shop.create_order(Drink::Latte, Size::Large);

        assert!(shop.find_order(1).is_some());
        assert!(shop.find_order(99).is_none());
        assert_eq!(
            shop.get_order(1),
            Response::Ok(String::from("#1 Latte Large $5.00 (pending)"))
        );
        assert_eq!(shop.get_order(99), Response::NotFound);
    }

    #[test]
    fn mission_6_cancel_order() {
        let mut shop = Shop::new();
        shop.create_order(Drink::Tea, Size::Small); // id 1
        shop.create_order(Drink::Espresso, Size::Small); // id 2
        shop.mark_ready(2, 4);

        assert_eq!(shop.cancel_order(1), Response::NoContent);
        assert_eq!(shop.find_order(1).unwrap().status, OrderStatus::Cancelled);
        assert_eq!(shop.cancel_order(99), Response::NotFound);
        // Cancelling twice, or cancelling a ready order, is a conflict
        assert_eq!(shop.cancel_order(1).status_code(), 409);
        assert_eq!(shop.cancel_order(2).status_code(), 409);
    }

    #[test]
    fn mission_7_router() {
        let mut shop = Shop::new();
        assert_eq!(shop.handle(Route::Health), Response::Ok(String::from("ok")));
        assert_eq!(
            shop.handle(Route::CreateOrder {
                drink: Drink::Latte,
                size: Size::Medium
            }),
            Response::Created { id: 1 }
        );
        assert_eq!(shop.handle(Route::GetOrder(1)).status_code(), 200);
        assert_eq!(
            shop.handle(Route::MarkReady { id: 1, counter: 2 })
                .status_code(),
            200
        );
        assert_eq!(shop.handle(Route::CancelOrder(1)).status_code(), 409);
        assert_eq!(shop.handle(Route::ListOrders).status_code(), 200);
        assert_eq!(shop.handle(Route::Stats).status_code(), 200);
    }

    #[test]
    fn mission_8_ready_count() {
        let mut shop = Shop::new();
        assert_eq!(shop.ready_count(), 0);
        shop.create_order(Drink::Tea, Size::Small);
        shop.create_order(Drink::Tea, Size::Small);
        shop.create_order(Drink::Tea, Size::Small);
        shop.mark_ready(1, 1);
        shop.mark_ready(3, 2);
        assert_eq!(shop.ready_count(), 2);
    }

    #[test]
    fn full_api_session() {
        let mut shop = Shop::new();
        let mut call = |line: &str| {
            let response = match parse_request(line) {
                Some(route) => shop.handle(route),
                None => Response::BadRequest(String::from("?")),
            };
            response.status_code()
        };
        assert_eq!(call("GET /health"), 200);
        assert_eq!(call("POST /orders latte large"), 201);
        assert_eq!(call("POST /orders mojito large"), 400);
        assert_eq!(call("GET /orders/1"), 200);
        assert_eq!(call("GET /orders/42"), 404);
        assert_eq!(call("POST /orders/1/ready 3"), 200);
        assert_eq!(call("DELETE /orders/1"), 409);
        assert_eq!(call("PATCH /orders/1"), 400);
    }
}
