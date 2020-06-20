use mxml::mixin;

mixin!(tooltip(message),
  <? +"data-toggle"="tooltip" +"data-placement"="top" +title={{message}}/>
);

#[test]
fn simple_tooltip() {
   assert_eq!(
     tooltip("hey!").mixin(
        r#"<button type="button" class="btn btn-secondary">tooltip will be mixed in</button>"#
     ),
     r#"<button type="button" class="btn btn-secondary" data-toggle="tooltip" data-placement="top" title="hey!">tooltip will be mixed in</button>"#
   );
}
