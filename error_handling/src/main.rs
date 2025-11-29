use gpui::{
    App, Application, Bounds, Context, Entity, Global, SharedString, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

// struct _HelloWorld {
//     text: SharedString,
// }

// impl Render for _HelloWorld {
//     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//         div()
//             .flex()
//             .flex_col()
//             .gap_3()
//             .bg(rgb(0xFFA500))
//             .size(px(800.0))
//             .justify_center()
//             .items_center()
//             .shadow_lg()
//             .border_1()
//             .border_color(rgb(0x0000ff))
//             .text_xl()
//             .text_color(rgb(0xffffff))
//             .child(format!("Hello, {}!", &self.text))
//             .child(
//                 div()
//                     .flex()
//                     .gap_2()
//                     .child(div().size_8().bg(gpui::red()).on_any_mouse_down(
//                         |_, _window: &mut Window, _app: &mut App| {
//                             println!("Clicked red");
//                         },
//                     ))
//                     .child(div().size_8().bg(gpui::green()))
//                     .child(div().size_8().bg(gpui::blue()))
//                     .child(div().size_8().bg(gpui::yellow()))
//                     .child(div().size_8().bg(gpui::black()))
//                     .child(div().size_8().bg(gpui::white())),
//             )
//     }
// }

// struct _Button {
//     text: SharedString,
//     color: u32,
// }

// impl _Button {
//     fn change_color(&mut self, _: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
//         self.color += 10000;
//         println!("{}", self.color);
//         cx.notify();
//     }
// }

// impl Render for _Button {
//     fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         // let handle =

//         return div()
//             .bg(rgb(self.color))
//             .child(format!("{}", &self.text))
//             .h_full()
//             .w_full()
//             .flex()
//             .justify_center()
//             .items_center()
//             .on_mouse_down(gpui::MouseButton::Left, cx.listener(_Button::change_color));
//     }
// }

struct GlobalSelection {
    selection: SharedString,
}
impl gpui::EventEmitter<()> for GlobalSelection {}

struct Files {
    files: Vec<SharedString>,
    selection: Entity<GlobalSelection>,
}

impl Files {
    fn new(
        files: Vec<SharedString>,
        selection: Entity<GlobalSelection>,
        cx: &mut Context<Self>,
    ) -> Self {
        // Subscribe to selection changes
        cx.subscribe(&selection, |this, _entity, _event, cx| {
            cx.notify();
        })
        .detach();

        Self { files, selection }
    }
}

impl Render for Files {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selection = self.selection.read(cx).selection.clone();

        return div()
            .flex()
            .justify_around()
            .items_center()
            .w_full()
            .h_full()
            .child(
                div()
                    .bg(rgb(0x181818))
                    .flex()
                    .flex_col()
                    .children(self.files.iter().map(|f| {
                        cx.new(|_| FileEntry {
                            name: f.clone(),
                            selection: self.selection.clone(),
                        })
                    }))
                    .h_full()
                    .w_1_4(),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .bg(rgb(0x1f1f1f))
                    .child(
                        div()
                            .child(format!("{}", selection))
                            .text_color(gpui::white()),
                    )
                    .w_full()
                    .h_full(),
            );
    }
}

struct FileEntry {
    name: SharedString,
    selection: Entity<GlobalSelection>,
}

impl FileEntry {
    fn select_me(
        &mut self,
        _: &gpui::MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection.update(cx, |state, cx| {
            state.selection = self.name.clone();
            println!("selected: {}", self.name);
            cx.notify();
        });
    }
}

impl Render for FileEntry {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .child(self.name.clone())
            .text_color(gpui::white())
            .text_left()
            .px_3()
            .hover(|style_refinement| style_refinement.bg(rgb(0x2b2b2b)).text_color(rgb(0xffffff)))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(FileEntry::select_me));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Application::new().run(|cx: &mut App| {
    //     let bounds = Bounds::centered(None, size(px(800 as f32), px(500 as f32)), cx);

    //     let options = WindowOptions {
    //         window_bounds: Some(WindowBounds::Windowed((bounds))),
    //         ..Default::default()
    //     };

    //     cx.open_window(options, |_, cx| {
    //         cx.new(|_| HelloWorld {
    //             text: "Omar Emad".into(),
    //         })
    //     })
    //     .unwrap();
    // });

    let filenames: Vec<SharedString> = std::fs::read_dir("./")?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| {
                    e.path()
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned()) // <-- make owned String
                })
                .map(SharedString::from) // now wrap in SharedString safely
        })
        .collect();

    let default_selected: SharedString = filenames[0].clone();

    // println!("{:?}", filenames);

    Application::new().run(|cx: &mut App| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800 as f32), px(500 as f32)),
                cx,
            ))),
            ..Default::default()
        };

        let reservation = cx.reserve_entity();
        let selection_entity = cx.insert_entity(reservation, |_| GlobalSelection {
            selection: default_selected,
        });
        let selection_clone = selection_entity.clone();

        let build_root_view = |_window: &mut Window, cx: &mut App| {
            cx.new(|_| Files {
                files: filenames,
                selection: selection_clone,
            })
        };

        cx.open_window(options, build_root_view).unwrap();
    });

    Ok(())
}
