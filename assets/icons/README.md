# Icons

To add a new icon:

- Add the `Icon.svg` file in this directory.
- Modify the `load_icons!` macro invocation in `src/icons.rs` to include the `Icon`.

Use the `Icon` in ferrishot with `crate::icon!(Icon)`, which will expand to an `iced::widget::Svg` on use.
