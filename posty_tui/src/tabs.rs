use std::collections::HashMap;

use ratatui::{
    layout::{Layout, Rect},
    widgets::Widget,
};

use crate::Resizable;
pub struct TabView {
    name: Vec<String>,
}
impl TabView {
    fn new(&mut self) {
        
    }
    fn add_tab(&mut self) {
        
    }
}
pub struct WidgetHandler {
    //removes the widget and then resizes to fit according to it. 
    
}
///Go with an ECS approach where we can seperate the components from its other info by using
///component ids. 







pub struct Tab {

}
pub struct TabHolder {
    //Holds all the tabs and determines whether to display the tab and how to partition/allocate
    //space to a given tab.
    tabs: Vec<Tab>,
    visible: Vec<Tab>,
    layout: Layout,
    dirty: bool,
    widget_positions: HashMap<String, Rect>,
}
impl TabHolder {
    fn new() {}
    fn add_tab() {}
    ///Adds an additonal tab to the visible tab section.
    fn split() {}
    fn close_tab(&mut self, tab_id: usize) {
        //Returns back the closed tab.
        //This gets stored in some sort of file.
        
    }
    fn increase_tab_size(&mut self, tab_id: usize) {}
}

impl Widget for TabHolder {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let prev_layout = self.layout;
        



        for tab in &self.visible {
            //Decide the default splitting strategy.
        }
    }
}






///All the widget traits must implement Default as well considering we have to have defaults for
///how the widgets appear. 
