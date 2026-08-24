# Reference
## Stores
<details><summary><code>client.stores.<a href="/src/api/resources/stores/client.rs">list</a>() -> Result&lt;Vec&lt;ListStoresResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns every store the authenticated user has access to, including summary counts for locations, cards, customers, and posters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.stores.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stores.<a href="/src/api/resources/stores/client.rs">get</a>(id: i64) -> Result&lt;GetStoresResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns detailed information for a single store, including summary counts for its associated resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.stores.get(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `i64` — Store ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stores.<a href="/src/api/resources/stores/client.rs">update</a>(id: i64, request: UpdateStoresRequest) -> Result&lt;UpdateStoresResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the store's name or store_name. Use `store_name` for the public-facing name displayed to customers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .stores
        .update(
            1,
            &UpdateStoresRequest {
                account: UpdateStoresRequestAccount {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `i64` — Store ID
    
</dd>
</dl>

<dl>
<dd>

**account:** `UpdateStoresRequestAccount` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Cards
<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">list</a>(account_id: i64, scope: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;ListCardsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns loyalty card templates for the specified store. By default, only
active (unarchived) cards are returned. Use the `scope` parameter to include
archived cards.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .cards
        .list(
            1,
            &CardsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**scope:** `Option<String>` — Filter cards by archive status. Default: active only.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">create</a>(account_id: i64, request: CreateCardsRequest) -> Result&lt;CreateCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new loyalty stamp card template for the store. The card defines the
visual design (colours, icon, strip) and program rules (stamps required,
initial stamps).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .cards
        .create(
            1,
            &CreateCardsRequest {
                card: CreateCardsRequestCard {
                    name: "name".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**card:** `CreateCardsRequestCard` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">get</a>(account_id: i64, id: i64) -> Result&lt;GetCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single loyalty card template by ID, including reward and customer card counts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.cards.get(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Card ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">update</a>(account_id: i64, id: i64, request: UpdateCardsRequest) -> Result&lt;UpdateCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing loyalty card template. Only the provided attributes are changed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .cards
        .update(
            1,
            1,
            &UpdateCardsRequest {
                card: UpdateCardsRequestCard {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Card ID
    
</dd>
</dl>

<dl>
<dd>

**card:** `UpdateCardsRequestCard` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Customers
<details><summary><code>client.customers.<a href="/src/api/resources/customers/client.rs">list</a>(account_id: i64, search: Option&lt;Option&lt;String&gt;&gt;, source: Option&lt;Option&lt;String&gt;&gt;, external_id: Option&lt;Option&lt;String&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;, items: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListCustomersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of customers for the store. Use the `search` parameter to filter
by name, email, phone, card code (barcode), or external reference ID. Alternatively, pass
`source` AND `external_id` together to perform an exact lookup by an external reference -
the response will contain at most one customer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .customers
        .list(
            1,
            &CustomersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**search:** `Option<String>` — Search query to filter customers by name, email, phone, card code (barcode), or external reference ID
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` — External system slug (e.g. `square`, `shopify`). When combined with `external_id`, performs an exact lookup.
    
</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<String>` — External system's identifier for the customer. Must be combined with `source`.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page number (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<i64>` — Number of items per page
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customers.<a href="/src/api/resources/customers/client.rs">create</a>(account_id: i64, request: CreateCustomersRequest) -> Result&lt;CreateCustomersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new customer for the store. Requires `first_name` and at least one of `email` or `phone`.
Optionally enroll the customer in a loyalty card by passing `card_id`, and trigger delivery of
card links (email/SMS) by passing `send_card_links`. When a card with initial stamps is assigned,
those stamps are automatically applied as a welcome bonus.

Pass `metadata` to attach arbitrary key/value data, and `external_references` to link the
customer to records in other systems (e.g. Square, Shopify). External references are upserted
by `(source, external_id)` so this endpoint is safe to call with the same references twice.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .customers
        .create(
            1,
            &CreateCustomersRequest {
                customer: CreateCustomersRequestCustomer {
                    first_name: "first_name".to_string(),
                    ..Default::default()
                },
                card_id: None,
                send_card_links: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**card_id:** `Option<i64>` — Loyalty card ID to auto-enroll the customer in
    
</dd>
</dl>

<dl>
<dd>

**customer:** `CreateCustomersRequestCustomer` 
    
</dd>
</dl>

<dl>
<dd>

**send_card_links:** `Option<bool>` — When true, sends the card links to the customer via email/SMS after enrollment. Note: even without this flag, the response includes `apple_wallet_url` and `google_wallet_url` in each customer card object so you can deliver them yourself.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customers.<a href="/src/api/resources/customers/client.rs">get</a>(account_id: i64, id: i64) -> Result&lt;GetCustomersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns detailed information about a single customer, including all of their
enrolled loyalty cards with stamp progress and wallet pass URLs (`apple_wallet_url`
and `google_wallet_url`) for each card. Also includes `metadata` and
`external_references` so you can sync state with external systems.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.customers.get(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Customer ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customers.<a href="/src/api/resources/customers/client.rs">update</a>(account_id: i64, id: i64, request: UpdateCustomersRequest) -> Result&lt;UpdateCustomersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing customer's details. To add stamps or redeem rewards, use the
customer cards endpoints instead.

`metadata` is shallow-merged into the existing metadata. `external_references` are upserted
by `(source, external_id)` - to remove a reference, omit it from subsequent calls and use
a separate `DELETE` workflow (not yet exposed via API; manage in dashboard for now).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .customers
        .update(
            1,
            1,
            &UpdateCustomersRequest {
                customer: UpdateCustomersRequestCustomer {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Customer ID
    
</dd>
</dl>

<dl>
<dd>

**customer:** `UpdateCustomersRequestCustomer` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Customer Cards
<details><summary><code>client.customer_cards.<a href="/src/api/resources/customer_cards/client.rs">list</a>(account_id: i64, customer_id: i64) -> Result&lt;Vec&lt;ListCustomerCardsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns all loyalty cards enrolled for a specific customer, including stamp progress,
status, wallet pass installation state, and wallet pass URLs (`apple_wallet_url` and
`google_wallet_url`) that you can use to let customers add their loyalty card to
Apple Wallet or Google Wallet from your own app or website.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.customer_cards.list(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `i64` — Customer ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer_cards.<a href="/src/api/resources/customer_cards/client.rs">get</a>(account_id: i64, customer_id: i64, id: i64) -> Result&lt;GetCustomerCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns detailed information about a specific customer card, including stamp progress,
a list of rewards the customer has earned enough stamps to redeem, and wallet pass URLs
(`apple_wallet_url` and `google_wallet_url`) for adding the card to Apple Wallet or
Google Wallet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.customer_cards.get(1, 1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `i64` — Customer ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Customer card ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer_cards.<a href="/src/api/resources/customer_cards/client.rs">redeem</a>(account_id: i64, customer_id: i64, id: i64, request: RedeemCustomerCardsRequest) -> Result&lt;RedeemCustomerCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Redeems a reward for a customer, deducting the required stamps from their card.
The customer must have enough stamps on this card to cover the reward's cost.
Triggers wallet pass updates and push notifications.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .customer_cards
        .redeem(1, 1, 1, &RedeemCustomerCardsRequest { reward_id: 1 }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `i64` — Customer ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Customer card ID
    
</dd>
</dl>

<dl>
<dd>

**reward_id:** `i64` — Reward ID to redeem
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer_cards.<a href="/src/api/resources/customer_cards/client.rs">stamp</a>(account_id: i64, customer_id: i64, id: i64, request: StampCustomerCardsRequest) -> Result&lt;StampCustomerCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds stamps to a customer's loyalty card. Triggers ledger entries, wallet pass updates,
and push notifications. Pass `skip_notifications` to stamp silently.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .customer_cards
        .stamp(
            1,
            1,
            1,
            &StampCustomerCardsRequest {
                stamps: 1,
                skip_notifications: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `i64` — Customer ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Customer card ID
    
</dd>
</dl>

<dl>
<dd>

**skip_notifications:** `Option<bool>` — When true, stamp changes bypass notifications
    
</dd>
</dl>

<dl>
<dd>

**stamps:** `i64` — Number of stamps to add (e.g. 1, 3)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Locations
<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">list</a>(account_id: i64) -> Result&lt;Vec&lt;ListLocationsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns every physical location belonging to the specified store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.locations.list(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">create</a>(account_id: i64, request: CreateLocationsRequest) -> Result&lt;CreateLocationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new physical location for the store. The provided address is
automatically geocoded to latitude and longitude coordinates in the background.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .locations
        .create(
            1,
            &CreateLocationsRequest {
                location: CreateLocationsRequestLocation {
                    address: "address".to_string(),
                    name: "name".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**location:** `CreateLocationsRequestLocation` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">get</a>(account_id: i64, id: i64) -> Result&lt;GetLocationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single location by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.locations.get(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Location ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">delete</a>(account_id: i64, id: i64) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a location. This action cannot be undone.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.locations.delete(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Location ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">update</a>(account_id: i64, id: i64, request: UpdateLocationsRequest) -> Result&lt;UpdateLocationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing location. If the address is changed, it will be re-geocoded automatically.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .locations
        .update(
            1,
            1,
            &UpdateLocationsRequest {
                location: UpdateLocationsRequestLocation {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Parent store ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Location ID
    
</dd>
</dl>

<dl>
<dd>

**location:** `UpdateLocationsRequestLocation` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Posters
<details><summary><code>client.posters.<a href="/src/api/resources/posters/client.rs">list</a>(account_id: i64, card_id: Option&lt;Option&lt;i64&gt;&gt;, active: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;ListPostersResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns all posters for the store. Optionally filter by card or active status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .posters
        .list(
            1,
            &PostersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**card_id:** `Option<i64>` — Filter posters belonging to a specific card
    
</dd>
</dl>

<dl>
<dd>

**active:** `Option<String>` — When present, return only active posters
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.posters.<a href="/src/api/resources/posters/client.rs">create</a>(account_id: i64, request: CreatePostersRequest) -> Result&lt;CreatePostersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new printable QR code poster for customer signup. The poster will automatically
generate a unique public signup URL and QR code. The `card_id` is required on create to
associate the poster with a loyalty card.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .posters
        .create(
            1,
            &CreatePostersRequest {
                poster: CreatePostersRequestPoster {
                    card_id: 1,
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**poster:** `CreatePostersRequestPoster` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.posters.<a href="/src/api/resources/posters/client.rs">get</a>(account_id: i64, id: i64) -> Result&lt;GetPostersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single poster by ID, including generated signup and display URLs.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.posters.get(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Poster ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.posters.<a href="/src/api/resources/posters/client.rs">delete</a>(account_id: i64, id: i64) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a poster. The public signup URL will stop working.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.posters.delete(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Poster ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.posters.<a href="/src/api/resources/posters/client.rs">update</a>(account_id: i64, id: i64, request: UpdatePostersRequest) -> Result&lt;UpdatePostersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing poster. The `card_id` cannot be changed after creation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .posters
        .update(
            1,
            1,
            &UpdatePostersRequest {
                poster: UpdatePostersRequestPoster {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Poster ID
    
</dd>
</dl>

<dl>
<dd>

**poster:** `UpdatePostersRequestPoster` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Rewards
<details><summary><code>client.rewards.<a href="/src/api/resources/rewards/client.rs">list</a>(account_id: i64, card_id: Option&lt;Option&lt;i64&gt;&gt;, active: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;ListRewardsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns all rewards for the store. Optionally filter by card or active status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .rewards
        .list(
            1,
            &RewardsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**card_id:** `Option<i64>` — Filter rewards belonging to a specific card
    
</dd>
</dl>

<dl>
<dd>

**active:** `Option<String>` — When present, return only active rewards
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.rewards.<a href="/src/api/resources/rewards/client.rs">create</a>(account_id: i64, request: CreateRewardsRequest) -> Result&lt;CreateRewardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new reward for a loyalty card. The card must belong to the same store.
The `card_id` is required on create but cannot be changed afterwards.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .rewards
        .create(
            1,
            &CreateRewardsRequest {
                reward: CreateRewardsRequestReward {
                    card_id: 1,
                    name: "name".to_string(),
                    stamps_required: 1,
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**reward:** `CreateRewardsRequestReward` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.rewards.<a href="/src/api/resources/rewards/client.rs">get</a>(account_id: i64, id: i64) -> Result&lt;GetRewardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single reward by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.rewards.get(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Reward ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.rewards.<a href="/src/api/resources/rewards/client.rs">delete</a>(account_id: i64, id: i64) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a reward. This cannot be undone.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.rewards.delete(1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Reward ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.rewards.<a href="/src/api/resources/rewards/client.rs">update</a>(account_id: i64, id: i64, request: UpdateRewardsRequest) -> Result&lt;UpdateRewardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing reward. The `card_id` cannot be changed after creation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client
        .rewards
        .update(
            1,
            1,
            &UpdateRewardsRequest {
                reward: UpdateRewardsRequestReward {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `i64` — Store (account) ID
    
</dd>
</dl>

<dl>
<dd>

**id:** `i64` — Reward ID
    
</dd>
</dl>

<dl>
<dd>

**reward:** `UpdateRewardsRequestReward` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Status
<details><summary><code>client.status.<a href="/src/api/resources/status/client.rs">check</a>() -> Result&lt;CheckStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the status of the API. No authentication required.

Every response from this API, including this one, carries `RateLimit-Limit`,
`RateLimit-Remaining`, `RateLimit-Reset` and `RateLimit-Policy`. Exceeding
the limit returns 429 with `Retry-After` in seconds.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use leal::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LealClient::new(config).expect("Failed to build client");
    client.status.check(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

