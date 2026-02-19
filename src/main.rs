use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const GUIDE_INSTALL_SNIPPET: &str = r#"# Cargo.toml
[dependencies]
iced = "0.14"

# run an official example
cargo run --package counter"#;
const GUIDE_BUNDLING_SNIPPET: &str = r#"iced::application(State::new, State::update, State::view)
    .title(State::title)
    .window_size((1200.0, 760.0))
    .centered()
    .run()"#;
const TUTORIAL_STEP1_SNIPPET: &str = r#"#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}"#;
const TUTORIAL_STEP2_SNIPPET: &str = r#"fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}"#;
const TUTORIAL_STEP3_SNIPPET: &str = r#"use iced::Center;
use iced::widget::{button, column, text, Column};

fn view(counter: &Counter) -> Column<'_, Message> {
    column![
        button("Increment").on_press(Message::Increment),
        text(counter.value).size(50),
        button("Decrement").on_press(Message::Decrement)
    ]
    .padding(20)
    .align_x(Center)
}"#;
const TUTORIAL_STEP4_SNIPPET: &str = r#"pub fn main() -> iced::Result {
    iced::run(update, view)

    // or:
    // iced::application(new, update, view)
    //     .title(title)
    //     .theme(theme)
    //     .subscription(subscription)
    //     .run()
}"#;
const GUIDE_SETUP_SNIPPET: &str = r#"use iced::widget::{button, column, text};

#[derive(Default)]
struct Counter { value: i64 }

#[derive(Debug, Clone, Copy)]
enum Message { Increment, Decrement }

fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}"#;
const GUIDE_TOOLING_SNIPPET: &str = r#"# native
cargo run --package tour

# web (from the example folder)
trunk serve"#;

const WIDGET_MODULES: &[(&str, &str, &str)] = &[
    ("button", "Buttons perform actions when pressed.", "reference/doc/iced/widget/index.html"),
    ("checkbox", "Binary choice control.", "reference/doc/iced/widget/index.html"),
    (
        "combo_box",
        "Searchable dropdown for selecting one option.",
        "reference/doc/iced/widget/index.html",
    ),
    ("container", "Aligns and wraps one child widget.", "reference/doc/iced/widget/index.html"),
    ("float", "Lets elements float over surrounding content.", "reference/doc/iced/widget/index.html"),
    ("grid", "Responsive grid distribution.", "reference/doc/iced/widget/index.html"),
    ("keyed", "Key-aware widgets to preserve continuity.", "reference/doc/iced/widget/index.html"),
    ("operation", "Programmatic widget state operations.", "reference/doc/iced/widget/index.html"),
    ("overlay", "Interactive layers above other widgets.", "reference/doc/iced/widget/index.html"),
    (
        "pane_grid",
        "User-rearrangeable split panes.",
        "reference/doc/iced/widget/index.html",
    ),
    ("pick_list", "Single-select dropdown list.", "reference/doc/iced/widget/index.html"),
    (
        "progress_bar",
        "Progress visualization for long-running operations.",
        "reference/doc/iced/widget/index.html",
    ),
    ("radio", "Single choice from multiple options.", "reference/doc/iced/widget/index.html"),
    ("row", "Horizontal layout container.", "reference/doc/iced/widget/index.html"),
    ("rule", "Horizontal or vertical divider.", "reference/doc/iced/widget/index.html"),
    (
        "scrollable",
        "Scrollable region for large content.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "sensor",
        "Signals when content enters/leaves view.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "shader",
        "Custom shader widget for the wgpu backend.",
        "reference/doc/iced/widget/index.html",
    ),
    ("slider", "Horizontal value slider.", "reference/doc/iced/widget/index.html"),
    ("space", "Explicit spacing element.", "reference/doc/iced/widget/index.html"),
    ("table", "Tabular data display.", "reference/doc/iced/widget/index.html"),
    ("text", "Text rendering and interaction.", "reference/doc/iced/widget/index.html"),
    (
        "text_editor",
        "Multi-line text editor input.",
        "reference/doc/iced/widget/index.html",
    ),
    ("text_input", "Single-line text input.", "reference/doc/iced/widget/index.html"),
    ("theme", "Built-in widget themes/styles.", "reference/doc/iced/widget/index.html"),
    ("toggler", "Switch-style binary input.", "reference/doc/iced/widget/index.html"),
    ("tooltip", "Hover hints and helper overlays.", "reference/doc/iced/widget/index.html"),
    (
        "vertical_slider",
        "Vertical value slider.",
        "reference/doc/iced/widget/index.html",
    ),
];

const WIDGET_BUILDERS: &[(&str, &str, &str)] = &[
    ("button", "Create a Button with content.", "reference/doc/iced/widget/index.html"),
    ("checkbox", "Create a Checkbox.", "reference/doc/iced/widget/index.html"),
    ("combo_box", "Create a ComboBox.", "reference/doc/iced/widget/index.html"),
    ("container", "Create a Container with one child.", "reference/doc/iced/widget/index.html"),
    ("float", "Create a Float wrapper.", "reference/doc/iced/widget/index.html"),
    ("grid", "Create a Grid from an iterator.", "reference/doc/iced/widget/index.html"),
    ("hover", "Show overlay content on hover.", "reference/doc/iced/widget/index.html"),
    ("keyed_column", "Create keyed::Column from keyed elements.", "reference/doc/iced/widget/index.html"),
    ("mouse_area", "Capture mouse events in a region.", "reference/doc/iced/widget/index.html"),
    ("opaque", "Capture mouse presses inside child bounds.", "reference/doc/iced/widget/index.html"),
    ("pane_grid", "Create a PaneGrid from pane state + view fn.", "reference/doc/iced/widget/index.html"),
    ("pick_list", "Create a PickList.", "reference/doc/iced/widget/index.html"),
    ("pin", "Position child at fixed coordinates.", "reference/doc/iced/widget/index.html"),
    ("progress_bar", "Create a ProgressBar.", "reference/doc/iced/widget/index.html"),
    ("radio", "Create a Radio button.", "reference/doc/iced/widget/index.html"),
    ("responsive", "Build content from current dimensions.", "reference/doc/iced/widget/index.html"),
    ("rich_text", "Create rich text from spans.", "reference/doc/iced/widget/index.html"),
    ("row", "Create a Row from iterator.", "reference/doc/iced/widget/index.html"),
    ("scrollable", "Create a Scrollable with child content.", "reference/doc/iced/widget/index.html"),
    ("sensor", "Create a Sensor widget.", "reference/doc/iced/widget/index.html"),
    ("shader", "Create a Shader widget.", "reference/doc/iced/widget/index.html"),
    ("slider", "Create a Slider.", "reference/doc/iced/widget/index.html"),
    ("space", "Create empty Space.", "reference/doc/iced/widget/index.html"),
    ("span", "Create text span for rich text.", "reference/doc/iced/widget/index.html"),
    ("stack", "Create layered Stack content.", "reference/doc/iced/widget/index.html"),
    ("table", "Create a Table from columns and rows.", "reference/doc/iced/widget/index.html"),
    ("text", "Create a Text widget.", "reference/doc/iced/widget/index.html"),
    ("text_editor", "Create a TextEditor.", "reference/doc/iced/widget/index.html"),
    ("text_input", "Create a TextInput.", "reference/doc/iced/widget/index.html"),
    ("themer", "Apply a Theme to descendants.", "reference/doc/iced/widget/index.html"),
    ("toggler", "Create a Toggler.", "reference/doc/iced/widget/index.html"),
    ("tooltip", "Create Tooltip around content.", "reference/doc/iced/widget/index.html"),
    ("value", "Create Text from any display value.", "reference/doc/iced/widget/index.html"),
    (
        "vertical_slider",
        "Create a VerticalSlider.",
        "reference/doc/iced/widget/index.html",
    ),
    ("center", "Container helper: fill and center both axes.", "reference/doc/iced/widget/index.html"),
    (
        "center_x",
        "Container helper: fill horizontally + center x.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "center_y",
        "Container helper: fill vertically + center y.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "bottom",
        "Container helper: fill vertically + bottom align.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "bottom_center",
        "Container helper: fill and align bottom center.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "bottom_right",
        "Container helper: fill and align bottom right.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "right",
        "Container helper: fill horizontally + right align.",
        "reference/doc/iced/widget/index.html",
    ),
    (
        "right_center",
        "Container helper: fill and align right center.",
        "reference/doc/iced/widget/index.html",
    ),
];

const CORE_API: &[(&str, &str, &str)] = &[
    ("run", "Run a basic app with default Settings.", "reference/doc/iced/index.html"),
    (
        "application",
        "Create an Application builder from boot/update/view logic.",
        "reference/doc/iced/index.html",
    ),
    (
        "daemon",
        "Create a background daemon with update/view logic.",
        "reference/doc/iced/index.html",
    ),
    ("exit", "Create a Task that exits the runtime.", "reference/doc/iced/index.html"),
    ("Element", "The generic widget type returned by view.", "reference/doc/iced/index.html"),
    (
        "Task",
        "Concurrent runtime actions for async work and side effects.",
        "reference/doc/iced/struct.Task.html",
    ),
    (
        "Subscription",
        "Declarative passive event stream requests.",
        "reference/doc/iced/struct.Subscription.html",
    ),
    (
        "Theme",
        "Built-in theme enum and customization entrypoint.",
        "reference/doc/iced/enum.Theme.html",
    ),
    (
        "Length",
        "Layout sizing strategy (Fill, Shrink, etc.).",
        "reference/doc/iced/enum.Length.html",
    ),
    (
        "Settings",
        "Program settings for app startup/runtime.",
        "reference/doc/iced/struct.Settings.html",
    ),
    (
        "window module",
        "Window settings, actions, events, and runtime tasks.",
        "reference/doc/iced/window/index.html",
    ),
    (
        "time module",
        "Time utilities and runtime time tasks.",
        "reference/doc/iced/time/index.html",
    ),
    (
        "Application::window / window_size / title / theme / subscription",
        "Key builder methods to configure runtime behavior.",
        "reference/doc/iced/application/struct.Application.html",
    ),
    (
        "Task::perform / batch / map / chain / abortable",
        "Core async orchestration tools.",
        "reference/doc/iced/struct.Task.html",
    ),
    (
        "Subscription::none / run / run_with / map / batch",
        "Subscription composition and stream wiring.",
        "reference/doc/iced/struct.Subscription.html",
    ),
];

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(SiteLayout)]
    #[route("/")]
    Guide {},
    #[route("/reference")]
    Reference {},
    #[route("/tutorial")]
    Tutorial {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn SiteLayout() -> Element {
    rsx! {
        div { class: "page-bg" }
        div { class: "shell",
            header { class: "topbar",
                div { class: "brand",
                    div { class: "brand-mark", "iced" }
                    div { class: "brand-copy",
                        h1 { "Iced Docs" }
                        p { "Approachable docs, traceable to local rustdoc and official examples." }
                    }
                }
                nav { class: "route-nav",
                    Link { class: "route-link", to: Route::Guide {}, "Guide" }
                    Link { class: "route-link", to: Route::Reference {}, "Reference" }
                    Link { class: "route-link", to: Route::Tutorial {}, "Tutorial" }
                }
            }
            main { class: "content",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn Guide() -> Element {
    rsx! {
        div { class: "doc-page",
            section { class: "hero",
                h2 { "Guide" }
                p {
                    "Installation, setup, tooling, bundling, and distribution in one place, aligned with Iced 0.14 patterns "
                    span { class: "footnote-inline", "[G1][G2]" }
                }
            }

            section { class: "card",
                h3 { "1. Installation" }
                p { "Start with Rust + Cargo, then add iced dependencies in your crate. For quick orientation, the crate docs recommend starting with `iced::run(update, view)` and the official examples run through Cargo." }
                p { class: "footnote-inline", "[G1][G2]" }
                pre { class: "code",
                    code { "{GUIDE_INSTALL_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "2. Setup" }
                p { "The core app model is state + message enum + update + view. This pattern is shown in both the crate pocket guide and `counter`/`tour` examples." }
                p { class: "footnote-inline", "[G1][G3][G4]" }
                pre { class: "code",
                    code { "{GUIDE_SETUP_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "3. Tooling" }
                p { "Native app flow in official examples: `cargo run --package <example>`. For web-capable examples (like `tour` and `todos`), the docs use Trunk (`trunk serve`)." }
                p { class: "footnote-inline", "[G2][G5][G6]" }
                pre { class: "code",
                    code { "{GUIDE_TOOLING_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "4. Bundling" }
                p { "Iced exposes app/window configuration through `Application` builder methods and `window::Settings` APIs. This is where you define size, title, position, decorations, icon handling, and other packaging-facing defaults." }
                p { class: "footnote-inline", "[G7][G8]" }
                pre { class: "code",
                    code { "{GUIDE_BUNDLING_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "5. Distribution" }
                p { "Iced supports both native and web delivery paths in official examples. A practical distribution workflow is to keep one `update/view` codepath and produce target-specific artifacts with your Rust/web build pipeline." }
                p { class: "footnote-inline", "[G2][G5][G6][G8]" }
                ul {
                    li { "Native: package your compiled binary with your platform installer tooling." }
                    li { "Web: package generated static assets from your web build toolchain." }
                    li { "Use `window`/`Application` settings early so shipped behavior matches development." }
                }
            }

            section { class: "card footnotes",
                h3 { "Guide Footnotes" }
                ol {
                    li { "[G1] reference/doc/iced/index.html" }
                    li { "[G2] reference/examples/README.md" }
                    li { "[G3] reference/examples/counter/src/main.rs" }
                    li { "[G4] reference/examples/tour/src/main.rs" }
                    li { "[G5] reference/examples/tour/README.md" }
                    li { "[G6] reference/examples/todos/README.md" }
                    li { "[G7] reference/doc/iced/application/struct.Application.html" }
                    li { "[G8] reference/doc/iced/window/index.html" }
                }
            }
        }
    }
}

#[component]
fn Reference() -> Element {
    rsx! {
        div { class: "doc-page",
            section { class: "hero",
                h2 { "Reference" }
                p { "Detailed, approachable reference for Iced widgets/elements and important runtime APIs." }
                p { class: "footnote-inline", "[R1][R2][R3][R4][R5][R6]" }
            }

            section { class: "card",
                h3 { "Core Runtime API" }
                p { "These are the high-leverage entry points you will use in most real projects." }
                div { class: "grid-list",
                    for entry in CORE_API.iter() {
                        ReferenceItem {
                            name: entry.0.to_string(),
                            summary: entry.1.to_string(),
                            source: entry.2.to_string(),
                        }
                    }
                }
            }

            section { class: "card",
                h3 { "All Widget Modules (iced::widget)" }
                p { "Module-level coverage of all built-in widget domains exposed in `iced::widget`." }
                div { class: "grid-list",
                    for entry in WIDGET_MODULES.iter() {
                        ReferenceItem {
                            name: entry.0.to_string(),
                            summary: entry.1.to_string(),
                            source: entry.2.to_string(),
                        }
                    }
                }
            }

            section { class: "card",
                h3 { "Widget Constructors and Helpers" }
                p { "Builder/helper functions for creating widgets and layout wrappers directly." }
                div { class: "grid-list",
                    for entry in WIDGET_BUILDERS.iter() {
                        ReferenceItem {
                            name: entry.0.to_string(),
                            summary: entry.1.to_string(),
                            source: entry.2.to_string(),
                        }
                    }
                }
            }

            section { class: "card footnotes",
                h3 { "Reference Footnotes" }
                ol {
                    li { "[R1] reference/doc/iced/index.html" }
                    li { "[R2] reference/doc/iced/widget/index.html" }
                    li { "[R3] reference/doc/iced/application/struct.Application.html" }
                    li { "[R4] reference/doc/iced/window/index.html" }
                    li { "[R5] reference/doc/iced/struct.Task.html" }
                    li { "[R6] reference/doc/iced/struct.Subscription.html" }
                }
            }
        }
    }
}

#[component]
fn Tutorial() -> Element {
    rsx! {
        div { class: "doc-page",
            section { class: "hero",
                h2 { "Tutorial" }
                p { "A short end-to-end flow: state, messages, UI, async work, and app boot." }
                p { class: "footnote-inline", "[T1][T2][T3][T4][T5]" }
            }

            section { class: "card",
                h3 { "Step 1: Define state and messages" }
                p { "Use a plain Rust struct for app state and an enum for user/runtime messages." }
                pre { class: "code",
                    code { "{TUTORIAL_STEP1_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "Step 2: Implement update" }
                p { "`update` mutates state and can optionally return `Task<Message>` for async work." }
                pre { class: "code",
                    code { "{TUTORIAL_STEP2_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "Step 3: Implement view" }
                p { "`view` creates widgets and wires messages from interactions." }
                pre { class: "code",
                    code { "{TUTORIAL_STEP3_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "Step 4: Boot and run" }
                p { "Start simple with `iced::run`. Move to `iced::application(...)` when you need title/theme/window/subscription hooks." }
                pre { class: "code",
                    code { "{TUTORIAL_STEP4_SNIPPET}" }
                }
            }

            section { class: "card",
                h3 { "Step 5: Add async and passive events" }
                ul {
                    li { "Use `Task::perform` to run futures and feed output into `Message`." }
                    li { "Use `subscription` + `Subscription` for passive streams (time, window events, websocket-like workers)." }
                    li { "Map child tasks/subscriptions/messages with `map` as the app grows into multiple screens." }
                }
            }

            section { class: "card footnotes",
                h3 { "Tutorial Footnotes" }
                ol {
                    li { "[T1] reference/doc/iced/index.html" }
                    li { "[T2] reference/examples/counter/src/main.rs" }
                    li { "[T3] reference/examples/tour/src/main.rs" }
                    li { "[T4] reference/doc/iced/struct.Task.html" }
                    li { "[T5] reference/doc/iced/struct.Subscription.html" }
                }
            }
        }
    }
}

#[component]
fn ReferenceItem(name: String, summary: String, source: String) -> Element {
    rsx! {
        article { class: "ref-item",
            h4 { "{name}" }
            p { "{summary}" }
            p { class: "source", "Source: {source}" }
        }
    }
}
