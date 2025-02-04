use agui::{
    widget::{Layout, Size, Units},
    widgets::Button,
    UI,
};

fn main() -> Result<(), agpu::BoxError> {
    let program = agpu::GpuProgram::builder("agui widgets").build()?;

    let mut ui = UI::new(agui_agpu::WidgetRenderer::new(&program));

    ui.set_root(Button {
        layout: Layout {
            size: Size::Set {
                width: Units::Pixels(100.0),
                height: Units::Pixels(100.0),
            },
            ..Default::default()
        },
    });

    program.run_draw(move |frame| {
        if ui.update() {
            ui.get_renderer().render(frame);
        }
    })
}
