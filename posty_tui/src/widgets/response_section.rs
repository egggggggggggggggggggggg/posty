//The data shown here is immutable and should not allow the user to modify it any way. If they do
//wish to modify it, allow them to copy the data somewhere and let them modify it there. This is
//for when the user wants to re use a response's data.
use crate::widgets::dropdown::Displayable;

///These are for what to do with the response.
pub enum ResponseOptions {
    CopyBody,
    CopyAsRequestBody,
    SendToEditor,
    SaveAsExample,
}

#[derive(Default)]
pub struct ResponseSection {}

impl ResponseSection {
    fn new() {}
}
