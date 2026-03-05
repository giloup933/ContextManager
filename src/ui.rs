//use eframe::egui;
use egui_extras::{Column, TableBuilder};
use crate::{launch_project_with_logs, MyApp};

pub fn render_dashboard(ui: &mut eframe::egui::Ui, myapp: &mut MyApp) {
    ui.vertical_centered(|ui| {
        ui.heading("Context Manager");
    }); 

    ui.add_space(20.0);

    let table = TableBuilder::new(ui)
        .striped(true) // Alternating row color! cool!
        .resizable(true)
        .cell_layout(eframe::egui::Layout::left_to_right(eframe::egui::Align::Center))
        .column(Column::initial(120.0).at_least(100.0)) // Name
        .column(Column::remainder().at_least(200.0))    // Context (Notes)
        .column(Column::initial(150.0))                 // Last Update
        .column(Column::initial(100.0));                // Launch Button

    table.header(20.0, |mut header| {
        header.col(|ui| { ui.strong("Project Name"); });
        header.col(|ui| { ui.strong("Context / Notes"); });
        header.col(|ui| { ui.strong("Last Updated"); });
        header.col(|ui| { ui.strong("Action"); });
    })
    .body(|mut body| {
        for project in &myapp.projects {
            body.row(30.0, |mut row| {
                row.col(|ui| { ui.label(&project.name); });
                row.col(|ui| { ui.label(&project.recent_notes[0]); });
                //row.col(|ui| {
                    // Multiline text edit so you can update notes right in the table
                    // TODO: figure this out later (how to make modifiable)
                    //ui.text_edit_singleline(&mut project.recent_notes); 
                //});
                row.col(|ui| { ui.label(&project.last_update); });
                row.col(|ui| {
                    if ui.button("🚀 Wake Up").clicked() {
                        // Our logic from before:
                        launch_project_with_logs(&project);
                        // Update timestamp to Now (March 4, 2026)
                        // For this I will need to add cloning of the projectdisplay struct, makes sense anyway to modify notes
                        //project.last_update = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
                    }
                });
            });
        }
    });
}
