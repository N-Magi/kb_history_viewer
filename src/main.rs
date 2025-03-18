use std::string;

use iced::{advanced::{graphics::{core::widget, futures::backend::default}, widget::text}, alignment::{Horizontal, Vertical}, futures::{future::Select, task}, widget::{center, column, combo_box::{self, State}, row, shader::wgpu::hal::InstanceDescriptor, text_editor::{self, Action, Content}, Container, Row}, Element, Font, Padding, Settings, Size, Task};
use kb_dbcontext::KbDbContext;
use similar::{self, ChangeTag};
mod kb_dbcontext;
mod diff_tool_error;

fn main() {
    let mut settings = Settings::default();

    let _ = iced::application("viewer",MainWindow::update,MainWindow::view).window_size(Size::new(900.0,900.0)).run_with(||{
        let instance = MainWindow{..Default::default()};
        let task = Task::done(MainWindowMessage::Init);
        return (instance,task);
    });
}

#[derive(Debug,Default)]
struct MainWindow{
    kb_serchbox_content:text_editor::Content,
    window_width:u16,
    window_height:u16,
    view1_state: iced::widget::combo_box::State<String>,
    view1_combo_select :Option<String>,
    view2_combo_select :Option<String>,
    view2_state: iced::widget::combo_box::State<String>,
    view1_content:text_editor::Content,
    view2_content:text_editor::Content,
    diff_content:text_editor::Content,

    db_conn:kb_dbcontext::KbDbContext,
    kb_entities: Vec<kb_dbcontext::KbDiffEntity>,
}

#[derive(Debug,Clone)]
enum MainWindowMessage {
    Init,
    OnKbSerchBoxInput(Action),
    OnSerachButtonPress,
    View1ComboBoxSelected(String),
    View2ComboBoxSelected(String),

    View1TextInput(Action),
    View2TextInput(Action),

    SelectChanged,

    DiffTextInput(Action),
    DiffCalculationFinished(String),
}

pub async fn diff_calculation(old:String,new:String) -> String {
    let diff = similar::TextDiff::from_lines(&old, &new);

    let mut diff_reports = String::new();

    for change in diff.iter_all_changes() {
        if(change.tag() == ChangeTag::Equal) {
            continue;    
        }

        let mut op = "";
        match  change.tag(){
            ChangeTag::Delete => {
                op = "-";
            },
            ChangeTag::Insert => {
                op = "+";
            }
            _default => {
            }
        }   

        let report = 
        format!( "{}{}     {}\n" , 
            change.new_index().map_or("none".to_string(),|f| f.to_string()),
            op,
            change.value()
        );
        diff_reports += &report;
    }
    return  diff_reports;

}


impl MainWindow {
    
    fn new() -> Self {
        todo!()
    }

    fn update(&mut self, msg:MainWindowMessage) -> Task<MainWindowMessage>{
        
        match msg {

            MainWindowMessage::Init => {
                self.db_conn = KbDbContext::new();
                let _ = self.db_conn.connect("./kbdb.sqlite".to_string());

                return Task::none();
            }

            MainWindowMessage::OnKbSerchBoxInput(action) => {
                self.kb_serchbox_content.perform(action);
                return  Task::none();
            }

            MainWindowMessage::OnSerachButtonPress => {

                let kb_num = self.kb_serchbox_content.text().replace("\n", "").parse::<i64>().unwrap();
                self.kb_entities = self.db_conn.get_history(kb_num).unwrap();
                
                let history_dates:Vec<String> = self.kb_entities.iter().map(|entity| entity.last_modified_date.to_rfc3339()).collect();

                self.view1_state = combo_box::State::new(history_dates.clone());
                self.view2_state = combo_box::State::new(history_dates.clone());
                
                return Task::done(MainWindowMessage::SelectChanged);
            }
            
            MainWindowMessage::View1ComboBoxSelected(selected) => {

                self.view1_combo_select = Some(selected.clone());

                let opt_entry = self.kb_entities.iter().find(|p| p.last_modified_date.to_rfc3339() == selected);
                if let Some(entiry) = opt_entry {
                    self.view1_content = Content::with_text(&entiry.content);
                }


                return Task::done(MainWindowMessage::SelectChanged);
            }



            MainWindowMessage::View2ComboBoxSelected(selected) => {
                
                self.view2_combo_select = Some(selected.clone());

                let opt_entry = self.kb_entities.iter().find(|p| p.last_modified_date.to_rfc3339() == selected);
                if let Some(entiry) = opt_entry {
                    self.view2_content = Content::with_text(&entiry.content);
                }

                return Task::done(MainWindowMessage::SelectChanged);
            }

            MainWindowMessage::SelectChanged => {
                if self.view1_combo_select.is_some() & self.view2_combo_select.is_some() {
                    //diff calculation
                    let a = self.view1_content.text();
                    let b = self.view2_content.text() + "test";
                    let task = Task::perform(diff_calculation(a, b),MainWindowMessage::DiffCalculationFinished);

                    return task;

                }

                return Task::none();
            }

            MainWindowMessage::DiffCalculationFinished(result) => {
                self.diff_content = text_editor::Content::with_text(&result);
                return  Task::none();
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

        let view1_combo_box = iced::widget::combo_box(&self.view1_state, "日付", self.view1_combo_select.as_ref().map(|value| value), MainWindowMessage::View1ComboBoxSelected);
        let view2_combo_box = iced::widget::combo_box(&self.view2_state, "日付", self.view2_combo_select.as_ref().map(|value| value), MainWindowMessage::View2ComboBoxSelected);

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

