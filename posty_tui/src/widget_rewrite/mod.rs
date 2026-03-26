///Rewriting cause a lot of the widgets I wrote didn't require a seperate state struct.
pub mod dropdown;
pub mod input_box;
pub mod input_table;
pub mod tabs;
pub mod toggle;
pub mod tree;
enum WidgetType {
    Dropdown,
    InputBox,
    InputTable,
    Toggle,
    Tree,
}
