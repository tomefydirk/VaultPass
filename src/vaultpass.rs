use eframe::egui::{self, Button, CentralPanel, Color32, Context, Frame, Pos2, Rect, SidePanel, Stroke, TextEdit, Ui, Vec2};
 #[derive(Default)]
pub struct Vaultpass{
    pub nom_obtained:String,
    pub pass_obtained:String

}
impl Vaultpass{
     pub fn new(_cc:&eframe::CreationContext<'_>)->Self{
        Default::default()
    }
}
fn add_label_field(ui:&mut Ui,text:String,saisie:&mut String){
        ui.horizontal(|ui1|{
            ui1.label(text);
            ui1.text_edit_singleline(saisie);
        });
}
impl eframe::App for Vaultpass{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{
           ui.vertical(|ui|{
            add_label_field(ui,"Nom".to_string() , &mut self.nom_obtained);
            ui.end_row();
            add_label_field(ui,"Mot de passe".to_string() , &mut self.pass_obtained);
            if ui.button("Soummettre").clicked(){
                
            }
           }
           )
        });
    }
}
pub fn run_vaultpass() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Vaultpass",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Vaultpass::new(cc))))
    )
}
