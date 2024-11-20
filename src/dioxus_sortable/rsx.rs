#![allow(non_snake_case)]
#![allow(dead_code)]

use dioxus::prelude::*;

use crate::dioxus_sortable::{Direction, SignalSorter, Sortable, SortBy};


/// See [`Th`].
#[derive(Props, PartialEq, Clone)]
pub struct ThProps<F: Copy + PartialEq + 'static> {
    sorter: SignalSorter<F>,
    field: F,
    children: Element,
}

/// Convenience helper. Builds a `<th>` element with a click handler that calls [`UseSorter::toggle_field`]. Renders the current state using [`ThStatus`].
pub fn Th<F: Copy + Sortable>(props: ThProps<F>) -> Element {
    let ThProps { mut sorter, field, children } = props;
    rsx! {
        th {
            onclick: move |_| sorter.write().toggle_field(field),
            {children}
            ThStatus {
                sorter: sorter,
                field: field,
            }
        }
    }
}

/// See [`ThStatus`].
#[derive(Props, PartialEq, Clone)]
pub struct ThStatusProps<F: Copy + PartialEq + 'static> {
    sorter: SignalSorter<F>,
    field: F,
}

/// Convenience helper. Renders the [`Sortable`] value for a given [`UseSorter`] and field.
///  - If the field is unsortable then render an empty string.
///  - If the field is sortable in one direction then render an arrow pointing in that direction.
///  - If the field is sortable in both directions then render an arrow pointing in the active direction, or a double-headed arrow if the field is inactive.
///
/// Active fields will be shown in bold (i.e., the current field being sorted by). Inactive fields will be greyed out.
pub fn ThStatus<F: Copy + Sortable>(props: ThStatusProps<F>) -> Element {
    let ThStatusProps { sorter, field } = props;
    let (active_field, active_dir) = sorter.read().get_state();
    let active = active_field == field;

    match field.sort_by() {
        None => rsx!(""),
        Some(sort_by) => {
            use Direction::*;
            use SortBy::*;
            match sort_by {
                Fixed(Ascending) => rsx!(ThSpan { active: active, "▲" }),
                Fixed(Descending) => rsx!(ThSpan { active: active, "▼" }),

                Reversible(_) => rsx!(
                ThSpan {
                    active: active,
                    match (active, active_dir) {
                        (true, Ascending) => "▲",
                        (true, Descending) => "▼",
                        (false, _) => "↕",
                    }
                }),
            }
        }
    }
}

/// See [`ThSpan`].
#[derive(Props, PartialEq, Clone)]
struct ThSpan {
    active: bool,
    children: Element,
}

/// Convenience helper. Renders an active or inactive gielement.
fn ThSpan(props: ThSpan) -> Element {
    let ThSpan { active, children } = props;

    let colour = if active { "#555" } else { "#ccc" };
    rsx! {
        span {
            style: "color: {colour};",
            span { dangerous_inner_html: "&nbsp;", }
            {children}
        }
    }
}
