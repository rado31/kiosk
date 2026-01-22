pub fn rect_is_clicked(ui: &mut egui::Ui, rect: egui::Rect) -> bool {
    ui.rect_contains_pointer(rect) && ui.input(|i| i.pointer.any_released())
}

pub fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}
