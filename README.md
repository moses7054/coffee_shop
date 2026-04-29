# Coffee Shop - Idiomatic Rust Patterns

A simple example demonstrating strongly-typed API design using Rust's type system to enforce correctness at compile time.

## Concepts Used

1. **Typestate Pattern** - Using phantom types to encode state machines in the type system
2. **Builder Pattern** - Compile-time validation of required fields
3. **Trait-Based Capabilities** - Role-based permissions through traits
4. **Rich Enums** - Domain modeling with enums that carry data and behavior

## File Structure

### `src/enums.rs`
Defines the domain model using enums and structs:
- `CoffeeType`, `Size`, `Milk`, `SyrupFlavor`, `Extra` - Basic enums with associated pricing logic
- `OrderDetails` - Main struct holding order information with a `total_price_cents()` method

### `src/builder.rs`
Implements a typestate builder pattern for creating orders:
- Uses `PhantomData<S>` to track builder state (`NeedsCoffee` → `NeedsSize` → `ReadyToBuild`)
- Enforces at compile time that coffee and size must be set before building
- Optional customizations (milk, extras) only available after required fields are set

### `src/typestate.rs`
Implements order lifecycle as a state machine:
- `Order<S>` uses phantom types to track order state
- States: `Pending` → `Customized` → `Paid` → `Preparing` → `Ready` → `Served`
- Each state transition consumes the old state and returns a new one
- Prevents invalid operations (e.g., can't serve an unpaid order)

### `src/capabilities.rs`
Defines role-based permissions using traits:
- `CanOrder`, `CanCharge`, `CanPrepare` - Traits representing capabilities
- `Customer`, `Cashier`, `Barista`, `Manager` - Role structs implementing different trait combinations
- Demonstrates generic functions with trait bounds (`run_bar_workflow<P: CanPrepare>`)

## Running the Example

```bash
cargo run
```

This executes the workflow in `capabilities::invoke()`, demonstrating a complete order lifecycle from placement to serving.
