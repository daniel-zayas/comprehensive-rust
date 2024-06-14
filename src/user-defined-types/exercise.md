---
minutes: 15
---

# Exercise: Car Events

We will create a data structure to represent an event in an car Electronic Unit Control (ECU)
system. It is up to you to define the types and functions to construct various
events. Use `#[derive(Debug)]` to allow the types to be formatted with `{:?}`.

This exercise only requires creating and populating data structures so that
`main` runs without errors. The next part of the course will cover getting data
out of these structures.

```rust,editable,should_panic
{{#include exercise.rs:event}}
    // TODO: add required variants
}



{{#include exercise.rs:Percentage}}

{{#include exercise.rs:Button}}

{{#include exercise.rs:Door}}



{{#include exercise.rs:car_door_opened}}
    todo!()
}

{{#include exercise.rs:car_door_closed}}
    todo!()
}

{{#include exercise.rs:car_stopped}}
    todo!()
}

{{#include exercise.rs:passenger_button_volumeup}}
    todo!()
}

{{#include exercise.rs:break_pedal_pressed}}
    todo!()
}

{{#include exercise.rs:main}}
```


## Additional exercices collective-rustlings

Do the following exercises in [`collective-rustlings`](https://codeberg.org/mo8it/collective-rustlings):

- `structs1`
- `structs2`
- `structs3`
- `enums1`
- `enums2`
- `enums3`
