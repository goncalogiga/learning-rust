# ☕ Coffee API: Chapter 6 practice

A tiny coffee shop **API** where you write the missing pieces. You send it requests that look like
HTTP, and it answers with status codes, like a real backend:

```
> POST /orders latte large
201 order 1 created
> GET /orders/1
200 #1 Latte Large $5.00 (pending)
> DELETE /orders/42
404 not found
```

There's no real network, just your keyboard. Everything that makes this work is an **enum** plus a
**match**:

| Book section | Where you'll use it |
|---|---|
| 6.1 Defining an enum | `Route`, `Response`, `OrderStatus` (variants that carry data) |
| 6.1 `Option<T>` | parsing user input, looking up an order that might not exist |
| 6.2 `match` | prices, status codes, the request router |
| 6.3 `if let` / `let...else` | counting ready orders, cancelling orders |

## How to work through it

Everything is in `src/main.rs`. Each mission is a `todo!("Mission N")` that you replace with real code.

1. Read the mission below. Each one takes about 5 to 10 minutes.
2. Write the code.
3. Run only that mission's tests:
   ```bash
   cargo test mission_1
   ```
4. Once they pass (green), you're done with that mission. Move on to the next one.

Do the missions **in order**, because later ones use earlier ones. If a test panics with
`not yet implemented: Mission 3`, the code you're testing calls a mission you haven't done yet.

When you finish everything, `cargo test` should pass all 12 tests, and you can play with your API:

```bash
cargo run                  # type requests by hand, Ctrl+D to quit
cargo run < requests.txt   # replay a whole morning at the coffee shop
```

### Endpoints

| Request | Meaning |
|---|---|
| `GET /health` | is the server alive? |
| `GET /stats` | number of orders, and how many are ready |
| `GET /orders` | list every order |
| `GET /orders/{id}` | one order |
| `POST /orders {drink} {size}` | place an order, e.g. `POST /orders tea small` |
| `POST /orders/{id}/ready {counter}` | the barista finished it, e.g. `POST /orders/1/ready 3` |
| `DELETE /orders/{id}` | cancel an order |

---

## Mission 1: Prices (basic `match`)

**Book: 6.2, "The match Control Flow Construct"**

Fill in `Drink::price_cents` and `Size::extra_cents`.

| Drink | Price |   | Size | Extra |
|---|---|---|---|---|
| Espresso | 250 | | Small | 0 |
| Latte | 400 | | Medium | 50 |
| Tea | 300 | | Large | 100 |

👉 Before you start, try this: write the `match` with only **two** of the three arms and run `cargo build`.
Read the error. This is *exhaustiveness*: Rust won't let you forget a case. That's the most important
idea in this chapter.

```bash
cargo test mission_1
```

<details><summary>Hint</summary>

```rust
match self {
    Drink::Espresso => 250,
    // ...
}
```
</details>

## Mission 2: Parsing input (returning `Option`)

**Book: 6.1, "The Option Enum and Its Advantages Over Null Values"**

A real API gets text from strangers on the internet, and that text is often garbage. Fill in
`Drink::parse` and `Size::parse` so that `"latte"` becomes `Some(Drink::Latte)` and `"mojito"` becomes `None`.

There's a worked example right above them: `Method::parse`. Copy its pattern.

Ask yourself: why return `Option<Drink>` instead of just `Drink`? What would you return for `"mojito"` otherwise?

```bash
cargo test mission_2
```

## Mission 3: Status codes (variants with data you don't care about)

**Book: 6.2, "Patterns That Bind to Values"**

Fill in `Response::status_code`. The codes are in the comments next to `enum Response`.

Some variants carry data (`Ok(String)`, `Created { id }`), but here you only need the *variant*, not
what's inside it. You have to write the pattern so that it ignores the data:

- tuple-style variant: `Response::Ok(_)`
- struct-style variant: `Response::Created { .. }`

```bash
cargo test mission_3
```

## Mission 4: Response bodies (pulling data out of variants)

**Book: 6.2, "Patterns That Bind to Values"**

Now you *do* need the data. Fill in `Response::body`:

| Variant | Body |
|---|---|
| `Ok(message)` | the message itself |
| `Created { id: 7 }` | `order 7 created` |
| `NoContent` | empty string |
| `BadRequest(reason)` | `bad request: <reason>` |
| `NotFound` | `not found` |
| `Conflict(reason)` | `conflict: <reason>` |

Then fill in `OrderStatus::describe`, which returns `pending`, `ready at counter 3`, or `cancelled`.

```bash
cargo test mission_4
```

<details><summary>Hint</summary>

`Response::Created { id } => format!("order {id} created")`. Writing `id` in the pattern creates
a variable. For `Ok(message)`, `message` is a `&String`, so use `message.clone()` to return an owned `String`.
</details>

## Mission 5: Looking things up (`Option<&T>` in practice)

**Book: 6.2, "Matching with Option\<T\>"**

Real APIs constantly ask, "does this thing exist?"

1. `Shop::find_order`: loop over `self.orders` and return `Some(order)` when the id matches. If the
   loop finishes without a match, return `None`. (`find_order_mut` does almost the same thing. Look at it.)
2. `Shop::get_order`: `match` on the result of `find_order`. Return `Response::Ok(order.summary())`
   for `Some`, and `Response::NotFound` for `None`.

```bash
cargo test mission_5
```

## Mission 6: Cancelling an order (`let...else` + `match`)

**Book: 6.3, "Staying on the 'Happy Path' with let...else"**

First read the worked example `Shop::mark_ready`. Then write `Shop::cancel_order` using the same shape:

| Situation | Response |
|---|---|
| order doesn't exist | `NotFound` |
| order is `Pending` | change its status to `Cancelled`, return `NoContent` |
| order is `Ready { .. }` | `Conflict("order {id} is already made, too late to cancel")` |
| order is `Cancelled` | `Conflict("order {id} is already cancelled")` |

These are real business rules written as a `match`. Every case is handled, and the compiler checks that for you.

```bash
cargo test mission_6
```

## Mission 7: The router (the heart of every web server)

Write `Shop::handle`: a single `match route { ... }` that sends each `Route` variant to the right method.

- `Route::Health` returns `Response::Ok(String::from("ok"))`
- everything else calls a method that already exists: `stats`, `list_orders`, `get_order`,
  `create_order`, `cancel_order`, `mark_ready`

You need to pull data out of the variants, e.g. `Route::GetOrder(id) => self.get_order(id)`.

Web frameworks like Axum and Actix do exactly this; they just generate the `match` for you.

```bash
cargo test mission_7
```

## Mission 8: Counting ready orders (`if let`)

**Book: 6.3, "Concise Control Flow with if let"**

Write `Shop::ready_count`. Loop over the orders and count the ones whose status is `Ready`.
You only care about *one* variant, so a full `match` with a `_ => {}` arm would be clunky. Use `if let`:

```rust
if let OrderStatus::Ready { .. } = order.status {
    // ...
}
```

```bash
cargo test mission_8
```

🎉 Now run `cargo test` (everything should pass) and `cargo run < requests.txt`.

---

## Bonus missions: let the compiler guide you

These are where enums really pay off. Change an enum, run `cargo build`, and **the compiler gives you a
to-do list** of every place you need to update.

**Bonus A: a new drink.** Add `Cappuccino` (450 cents) to `enum Drink`. Run `cargo build` and fix every
error until it compiles. Then make `POST /orders cappuccino large` work. (Hint: one place *won't*
show up as a compiler error. Why not? Look for the `_` arm.)

**Bonus B: picked up.** Add an `OrderStatus::PickedUp` variant and a new endpoint
`POST /orders/{id}/pickup`. Only a `Ready` order can be picked up. You'll need to touch `Route`,
`parse_request`, `Shop::handle`, and write a `Shop::pick_up` method. Again, `cargo build` shows you
every `match` that needs a new arm.

**Bonus C: read the parser.** Look at `parse_request` at the bottom of `main.rs`. It matches on a
*tuple* and a *slice* at the same time: `(Method::Get, ["orders", id])`. Can you add
`GET /orders/{id}/price`, which returns just the price?
