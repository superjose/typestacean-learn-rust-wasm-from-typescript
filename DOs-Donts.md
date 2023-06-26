# Ownership and Borrowing on Rust as a TypeScript developer

In TypeScript you can reassign variables, but in Rust you can't. This is because Rust has a concept called ownership and borrowing. Ownership and borrowing is a way to manage memory in Rust.
```typescript
// Valid TypeScript code:

const numberOfLegs = 4;
const specialTableLegs = numberOfLegs;

console.log("Number of legs", numberOfLegs);
console.log("Special Table Legs", specialTableLegs);
```

```rust
// Invalid Rust Code
let number_of_legs = 2;
let special_table_legs = number_of_legs;

// Invalid, because number_of_legs has moved to special_table_legs
println!("Number of legs {}", number_of_legs);
println!("Special Table Legs {}", special_table_legs);
```


# Mutation in TypeScript vs Rust
```typescript
const a: string[] = [];
a.push('hello');
```

Incorrect:
```rust
let a: Vec<String> = vec![];
// Invalid, because a is immutable
a.push("hello".to_string());
```

Correct:
```rust
let mut a: Vec<String> = vec![];
a.push("hello".to_string());
```

# Enums in TypeScript vs Rust
```typescript
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
```
    
```rust
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

# Structs in TypeScript vs Rust
```typescript
type Person = {
  name: string;
  age: number;
};
```

```rust
pub struct Person {
    pub name: String,
    pub age: u8,
}
```