mod support;

use gpui::{Modifiers, TestAppContext, VisualTestContext, px};
use jayjay_gpui::repo::RepoWindow;
use jj_test::LinearFixture;
use support::{install_test_globals, settle_visual};

#[gpui::test]
fn status_bar_last_operation_opens_operation_log(cx: &mut TestAppContext) {
    let fixture = LinearFixture::build();
    install_test_globals(cx);
    let (_view, cx) = cx.add_window_view(|_, cx| RepoWindow::new(fixture.path.clone(), cx));
    let cx: &mut VisualTestContext = cx;
    settle_visual(cx);

    let before = cx.cx.windows().len();
    let last_op = cx.debug_bounds("status-last-op").expect("last op bounds");
    cx.simulate_click(last_op.center(), Modifiers::default());
    settle_visual(cx);

    assert!(cx.cx.windows().len() > before);
    let window = cx
        .cx
        .windows()
        .last()
        .copied()
        .expect("operation log window");
    let mut op_cx = VisualTestContext::from_window(window, &cx.cx);
    settle_visual(&mut op_cx);
    assert!(op_cx.debug_bounds("operation-log-close").is_some());
    let description = op_cx
        .debug_bounds("operation-log-description")
        .expect("operation description bounds");
    assert!(
        description.size.width > px(200.),
        "operation description collapsed to {:?}",
        description.size.width
    );
}
