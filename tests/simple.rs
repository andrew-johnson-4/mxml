use mxml::mixin;

mixin!(tooltip(message),
  <button ~"type"="button" +"data-toggle"="tooltip" +"data-placement"="top" +title={{message}}/>
);

#[test]
fn simple_tooltip() {
   assert_eq!(
     tooltip("hey!").mixin(
        r#"<button type="button" class="btn btn-secondary">tooltip will be mixed in</button>"#
     ),
     r#"<button class="btn btn-secondary" data-placement="top" data-toggle="tooltip" title="hey!" type="button">tooltip will be mixed in</button>"#
   );
}
