use xilem_web::{document_body, elements::html, interfaces::Element, App, DomView};

fn update(state: &mut bool) -> impl DomView<bool> {
    html::div((
        html::button("click me!").on_click(|state: &mut bool, _| *state = !*state),
        html::br(()),
        if *state {
            html::div("div!").boxed()
        } else {
            html::span("span!").boxed()
        },
    ))
}

fn main() {
    App::new(document_body(), false, update).run();
}
