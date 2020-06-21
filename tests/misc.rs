use mxml::mixin;

mixin!(abc(),
  <p ~a="">
    <p ~b="" +c=""/>
  </p>
);

#[test]
fn simple_tooltip() {
   assert_eq!(
     abc().mixin(
        "<p a>a<p b>b<p>c</p></p></p>"
     ),
     "<p a>a<p b c>b<p>c</p></p></p>"
   );
}
