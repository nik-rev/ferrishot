#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::pedantic)]
#![allow(clippy::restriction)]
#![allow(clippy::nursery)]

//! Additional widget for the Iced GUI library.
//!
//! # Examples
//!
//! * `badge` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `card` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `color_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `context_menu` (Author: wiiznokes <wiiznokes2@gmail.com>)
//! * `date_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `drop_down` (Author: wiiznokes <wiiznokes2@gmail.com>)
//! * `grid` (Author: Alexander van Saase <avsaase@gmail.com>)
//! * `menu`
//! * `number_input` (Author: leang27 <52003343+leang27@users.noreply.github.com>)
//! * `selection_list` (Author: Héctor Ramón Jiménez <hector0193@gmail.com> and Andrew Wheeler <genusistimelord@gmail.com>)
//! * `side_bar` (Author: Kaiden42 <gitlab@tinysn.com> and Rizzen Yazston)
//! * `slide_bar` (Author: Andrew Wheeler <genusistimelord@gmail.com>)
//! * `spinner` (Author: Iohann Rabeson <irabeson42@gmail.com>)
//! * `tab_bar` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `tabs` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `time_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `typed_input` (Author: Ultraxime <36888699+Ultraxime@users.noreply.github.com>)
//! * `widget_id_return` (Author: Andrew Wheeler <genusistimelord@gmail.com>)
//! * `wrap` (Author: owntime <yrainbxqc@gmail.com>)
#![deny(missing_docs)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,

    // Restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
)]
#![allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::borrowed_box,
    clippy::missing_const_for_fn,
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::missing_docs_in_private_items,
    clippy::unit_arg,
    clippy::trivially_copy_pass_by_ref,
    clippy::let_unit_value
)]

pub mod widget;

pub mod core;
pub mod style;
#[cfg(any())]
pub use iced::Element;
#[cfg(any())]
pub use iced_fonts;

// Widgets that we care about

pub use widget::number_input;
pub use widget::number_input::NumberInput;
pub use widget::typed_input::TypedInput;

#[cfg(any())]
/// Exports for all platforms that are not WASM32.
mod platform {
    #[allow(unused_imports)]
    #[cfg(any())]
    pub use crate::style;
    #[cfg(any())]
    pub use crate::widgets::helpers;

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::badge, badge::Badge};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::card, card::Card};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::color_picker, color_picker::ColorPicker};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {
        crate::widgets::{grid, grid_row},
        grid::{Grid, GridRow},
    };

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {
        crate::widgets::tab_bar,
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {
        crate::widgets::tabs,
        tabs::{TabBarPosition, Tabs},
    };

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::time_picker, time_picker::TimePicker};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::wrap, wrap::Wrap, wrap::direction};

    #[doc(no_inline)]
    pub use {};

    #[doc(no_inline)]
    #[cfg(any())]
    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::selection_list, selection_list::SelectionList};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::menu, menu::Menu, menu::MenuBar};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::quad, quad::Quad};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::spinner, spinner::Spinner};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use crate::widgets::SlideBar;

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::context_menu, context_menu::ContextMenu};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use {crate::widgets::drop_down, drop_down::DropDown};

    #[doc(no_inline)]
    #[cfg(any())]
    pub use crate::widgets::sidebar;
}
