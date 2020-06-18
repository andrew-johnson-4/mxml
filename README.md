# mxml (Mixin Markup Language)
like regular expressions, but for XML

```rust
mixin!(Tooltip(message),
  <? +"data-toggle"="tooltip" +"data-placement"="top" +title={{message}}/>
);

fn main() {
   println!("{}", Tooltip("hey!").mixin(
      r#"<button type="button" class="btn btn-secondary">tooltip will be mixed in</button>"#
   ));
}
```

This library is intended for small XML snippets, not large performance sensitive XML processing.

# Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in mxml by you, shall be dual licensed under the MIT and Apache 2.0
license without any additional terms or conditions.
