use iced::{advanced::graphics::{core::widget, futures::backend::default}, alignment::{Horizontal, Vertical}, widget::{center, column, combo_box::State, row, shader::wgpu::hal::InstanceDescriptor, text_editor::{self, Action}, Container, Row}, Element, Font, Padding, Settings, Size, Task};
mod kb_dbcontext;
mod diff_tool_error;

fn main() {
    println!("Hello, world!");

    let mut settings = Settings::default();

    let _ = iced::application("viewer",MainWindow::update,MainWindow::view).window_size(Size::new(900.0,900.0)).run();
}

#[derive(Debug,Default)]
struct MainWindow{
    kb_serchbox_content:text_editor::Content,
    window_width:u16,
    window_height:u16,
    view1_state: iced::widget::combo_box::State<String>,
    view2_state: iced::widget::combo_box::State<String>,
    view1_content:text_editor::Content,
    view2_content:text_editor::Content,
    diff_content:text_editor::Content,

    //kb_entities: Vec<kb_dbcontext::KbDiffEntity>,
}

#[derive(Debug,Clone)]
enum MainWindowMessage {
    OnKbSerchBoxInput(Action),
    OnSerachButtonPress,
    View1ComboBoxSelected(String),
    View2ComboBoxSelected(String),

    View1TextInput(Action),
    View2TextInput(Action),

    DiffTextInput(Action),

}


impl MainWindow {
    
    fn new() -> Self {
        todo!()
    }

    fn update(&mut self, msg:MainWindowMessage) -> Task<MainWindowMessage>{
        
        match msg {
            MainWindowMessage::OnKbSerchBoxInput(action) => {
                self.kb_serchbox_content.perform(action);

                return  Task::none();
            }

            MainWindowMessage::View1ComboBoxSelected(selected) => {

                

                return Task::none();
            }

            default => {
                return Task::none();
            }
        }
        
    }

    fn view(&self) -> Element<MainWindowMessage>{

        let btn_search = iced::widget::button(iced::widget::text("search")).on_press(MainWindowMessage::OnSerachButtonPress);
        let kb_serch_textbox = iced::widget::text_editor(&self.kb_serchbox_content).on_action(MainWindowMessage::OnKbSerchBoxInput);

        let mut row1 = iced::widget::row![
            kb_serch_textbox.width(200),
            btn_search.padding(5)
            ];

        let view1_combo_box = iced::widget::combo_box(&self.view1_state, "日付", None, MainWindowMessage::View1ComboBoxSelected);
        let view2_combo_box = iced::widget::combo_box(&self.view2_state, "日付", None, MainWindowMessage::View2ComboBoxSelected);

        let row2 = iced::widget::row![
            view1_combo_box.padding(10),
            view2_combo_box.padding(10)
            ];

        let view1 = iced::widget::text_editor(&self.view1_content).on_action(MainWindowMessage::View1TextInput);
        let view2 = iced::widget::text_editor(&self.view2_content).on_action(MainWindowMessage::View2TextInput);

        let row3 = iced::widget::row![
            view1.height(500),
            view2.height(500),
        ];

        row1 = row1.align_y(Vertical::Top);
        
        let diff_view = iced::widget::text_editor(&self.diff_content).on_action(MainWindowMessage::DiffTextInput);

        let mut row4 = iced::widget::row![
            diff_view.height(325),
        ];

        let mut col1 = column![
            row1,
            row2,
            row3,
            row4,
        ].align_x(Horizontal::Left);

        let container = Container::new(col1);

        return  container.into();
    }

}

