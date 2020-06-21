use mxml::mixin;

mixin!(abc(),
  <p ~a="">
    <p ~b="" +c=""/>
  </p>
);

#[test]
fn misc1() {
   assert_eq!(
     abc().mixin(
        "<p a>a<p b>b<p>c</p></p></p>"
     ),
     "<p a>a<p b c>b<p>c</p></p></p>"
   );
}

mixin!(de(), <p ~id="a" +class="b"/>);

#[test]
fn misc2() {
   assert_eq!(
     de().mixin(
        r#"<p>a</p><p id="a">b</p>"#
     ),
     r#"<p>a</p><p id="a" class="b">b</p>"#
   );
}

mixin!(fg(), <p +id="a" ~class="b"/>);

#[test]
fn misc3() {
   assert_eq!(
     fg().mixin(
        r#"<p>a</p><p class="b">b</p>"#
     ),
     r#"<p>a</p><p id="a" class="b">b</p>"#
   );
}
