pub fn rect_is_clicked(ui: &mut egui::Ui, rect: egui::Rect) -> bool {
    ui.rect_contains_pointer(rect) && ui.input(|i| i.pointer.any_released())
}
