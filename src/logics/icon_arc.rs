use std::sync::Arc;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::IconShape;

pub struct IconArc(pub Arc<dyn IconShape>);


impl IconShape for IconArc {
    fn view_box(&self) -> &str { 
        self.0.view_box() 
    }
    fn xmlns(&self) -> &str { 
        self.0.xmlns() 
    }
    fn child_elements(&self) -> Element {
        self.0.child_elements()
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        self.0.fill_and_stroke(user_color)
    }
    fn stroke_linecap(&self) -> &str { 
        self.0.stroke_linecap() 
    }
    fn stroke_linejoin(&self) -> &str { 
        self.0.stroke_linejoin() 
    }
}

impl Clone for IconArc {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl PartialEq for IconArc {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}