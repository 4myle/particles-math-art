
use std::ops::RangeInclusive;
use eframe::emath::Numeric;

use eframe::egui:: {
    Ui,
    Widget,
    Label,
    RichText,
    Slider
};

pub struct Factor;
impl Factor
{
    // pub fn add<T: Numeric>(ui: &mut Ui, value: &mut T, range: RangeInclusive<T>, label: &str) -> Response {
    //     ui.vertical(|ui|  {
    //         ui.add(Label::new(RichText::new(label.to_uppercase()).small().weak()));
    //         ui.add(Slider::new(value, range));
    //     }).response
    // }
    
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, T: Numeric> (value: &'a mut T, range: RangeInclusive<T>, label: &'a str) -> impl Widget + 'a {
        move |ui: &mut Ui| {
            ui.vertical(|ui|  {
                ui.add(Label::new(RichText::new(label.to_uppercase()).small().weak()));
                ui.add(Slider::new(value, range));
            }).response
        }
    }

}
