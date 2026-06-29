use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

#[derive(Debug, Clone, Copy)]
enum Status {
    未着手,
    着手中,
    待ち,
    順延,
    完了,
}
struct Item {
    title: SharedString,
    status: Option<Status>,
}
struct Group {
    heading: SharedString,
    items: Vec<Item>,
}
struct Lane {
    title: SharedString,
    groups: Vec<Group>,
}
struct LogodexWindow {
    lanes: Vec<Lane>,
}

impl Render for LogodexWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xeeeeee))
            .flex()
            .flex_row()
            .gap_3()
            .p_4()
            .children(self.lanes.iter().map(render_lane))
    }
}

fn render_lane(lane: &Lane) -> impl IntoElement {
    div()
        .flex_1()
        .border_1()
        .border_color(rgb(0x444444))
        .rounded_md()
        .p_4()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_lg().child(lane.title.clone()))
        .children(lane.groups.iter().map(render_group))
}

fn render_group(group: &Group) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_color(rgb(0x9aaaff)).child(group.heading.clone()))
        .children(group.items.iter().map(render_item))
}

fn render_item(item: &Item) -> impl IntoElement {
    let row = div()
        .flex()
        .flex_row()
        .justify_between()
        .items_center()
        .child(item.title.clone());

    let t = match &item.status {
        Some(Status::未着手) => "未着手",
        Some(Status::着手中) => "着手中",
        Some(Status::待ち) => "待ち",
        Some(Status::順延) => "順延",
        Some(Status::完了) => "完了",
        None => "未着手",
    };

    row.child(
        div()
            .bg(rgb(0xf0a85a))
            .text_color(rgb(0x111111))
            .px_2()
            .rounded_full()
            .child(t),
    )
}

fn mock_lanes() -> Vec<Lane> {
    vec![
        Lane {
            title: "仕事管理".into(),
            groups: vec![
                Group {
                    heading: "mugenup".into(),
                    items: vec![Item {
                        title: "REDIS調査".into(),
                        status: Some(Status::待ち),
                    }],
                },
                Group {
                    heading: "社内".into(),
                    items: vec![Item {
                        title: "EOL対応".into(),
                        status: Some(Status::待ち),
                    }],
                },
            ],
        },
        Lane {
            title: "人間管理".into(),
            groups: vec![],
        },
        Lane {
            title: "シークレット".into(),
            groups: vec![Group {
                heading: "（後で統合）".into(),
                items: vec![Item {
                    title: "……".into(),
                    status: None,
                }],
            }],
        },
    ]
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(640.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| LogodexWindow {
                    lanes: mock_lanes(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
