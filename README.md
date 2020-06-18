# mxml: Mixin Markup Language

[![Crates.IO](https://img.shields.io/crates/v/mxml.svg)](https://crates.rs/crates/mxml)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/mxml)
[![Build Nightly](https://github.com/andrew-johnson-4/mxml/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/mxml)
[![Build](https://github.com/andrew-johnson-4/mxml/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/mxml)

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
