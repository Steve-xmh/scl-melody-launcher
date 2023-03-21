#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use druid::{
    commands::{CLOSE_ALL_WINDOWS, QUIT_APP},
    widget::Flex,
    AppLauncher, Color, Data, ImageBuf, Insets, Target, Widget, WidgetExt, WindowDesc,
};
use pollster::FutureExt;
use scl_gui_widgets::{
    widgets::{label, Button, WindowWidget, QUERY_CLOSE_WINDOW, SET_BACKGROUND_IMAGE},
    WidgetExt as _,
};

#[derive(Data, Clone, Default)]
struct AppState {}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_flex_spacer(1.)
                .with_child(
                    Button::new("启动游戏")
                        .on_click(|_, _, _| {
                            // 启动游戏

                            // 先提供 .minecraft/versions 路径和要启动的版本名称
                            let mut version_info = scl_core::version::structs::VersionInfo {
                                version_base: ".minecraft/versions".into(),
                                version: "1.19.4".into(),
                                ..Default::default()
                            };

                            // 执行加载版本元数据
                            version_info.load().block_on().unwrap();

                            // 构造客户端对象配置
                            let config = scl_core::client::ClientConfig {
                                auth: scl_core::auth::structs::AuthMethod::Offline {
                                    player_name: "Steve".into(),
                                    uuid: "".into(),
                                },
                                version_info,
                                java_path: "java".into(),
                                max_mem: 4096,
                                version_type: String::new(),
                                custom_java_args: Vec::new(),
                                custom_args: Vec::new(),
                                recheck: false,
                            };

                            // 创建客户端对象
                            let mut client =
                                scl_core::client::Client::new(config).block_on().unwrap();

                            // 启动游戏！
                            client.launch().block_on().unwrap();

                            std::process::exit(0);
                        })
                        .fix_size(149., 74.),
                )
                .with_spacer(11.)
                .with_child(
                    Flex::column()
                        .with_child(Button::new("下载游戏").fix_size(99., 34.))
                        .with_spacer(6.)
                        .with_child(Button::new("设置").fix_size(99., 34.)),
                )
                .with_spacer(25.),
        )
        .with_spacer(7.)
        .with_child(
            Flex::row()
                .with_flex_spacer(1.)
                .with_child(Button::new("游戏版本").fix_width(259.))
                .with_spacer(25.),
        )
        .with_spacer(7.)
        .with_child(
            label::new(" MineCraft Launcher 5.31 by 忘却的旋律 SteveXMH 重置").with_text_size(12.),
        )
        .with_spacer(2.)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Fill)
}

fn gen_win() -> WindowDesc<AppState> {
    let size = (720. - 12., 450. - 12.);
    let win = WindowDesc::new(
        WindowWidget::new("", ui_builder())
            .on_notify(QUERY_CLOSE_WINDOW, |ctx, _, _| {
                ctx.submit_command(CLOSE_ALL_WINDOWS);
                ctx.submit_command(QUIT_APP.to(Target::Window(ctx.window_id())));
            })
            .on_added(|_, ctx, _, _| {
                let background_raw_data = include_bytes!("../assets/background.webp");
                let img = image::load_from_memory_with_format(
                    background_raw_data,
                    image::ImageFormat::WebP,
                )
                .unwrap()
                .to_rgb8();
                let size = img.dimensions();
                ctx.submit_command(SET_BACKGROUND_IMAGE.with(ImageBuf::from_raw(
                    img.to_vec(),
                    druid::piet::ImageFormat::Rgb,
                    size.0 as _,
                    size.1 as _,
                )));
            }),
    )
    .title("MClauncher")
    .resizable(false)
    .transparent(true)
    .window_size(druid::Size::new(size.0, size.1));
    let win = {
        #[cfg(target_os = "windows")]
        {
            let monitors = druid::Screen::get_monitors();
            let screen = monitors.iter().find(|a| a.is_primary()).unwrap();
            let screen_rect = screen.virtual_work_rect();

            win.set_position(druid::Point::new(
                (screen_rect.width() - size.0) / 2.,
                (screen_rect.height() - size.1) / 2.,
            ))
        }
        #[cfg(target_os = "macos")]
        {
            win
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            win
        }
    };
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        win.show_titlebar(false).transparent(true)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        win
    }
}

fn main() {
    let _ = AppLauncher::with_window(gen_win())
        .configure_env(|env, _| {
            scl_gui_widgets::theme::color::set_color_to_env(
                env,
                scl_gui_widgets::theme::color::Theme::Dark, // 更换此处来切换黑白主题（或者自己触发 Command）
            );

            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::TRANSPARENT);

            // 设置 Druid 自带的主题的大小以匹配 WinUI 3 的风格
            env.set(
                druid::theme::TEXTBOX_INSETS,
                Insets::new(12.0, 6.0, 12.0, 6.0),
            );
            env.set(
                druid::theme::BACKGROUND_LIGHT,
                env.get(scl_gui_widgets::theme::color::alt::HIGH),
            );
            env.set(
                druid::theme::BACKGROUND_DARK,
                env.get(scl_gui_widgets::theme::color::alt::HIGH),
            );
            env.set(
                druid::theme::SELECTED_TEXT_BACKGROUND_COLOR,
                env.get(scl_gui_widgets::theme::color::accent::ACCENT),
            );
            env.set(
                druid::theme::CURSOR_COLOR,
                env.get(scl_gui_widgets::theme::color::base::HIGH),
            );
            env.set(druid::theme::TEXTBOX_BORDER_WIDTH, 1.);
            env.set(druid::theme::TEXTBOX_BORDER_RADIUS, 4.);
        })
        .launch(AppState::default());
    std::process::exit(0);
}
