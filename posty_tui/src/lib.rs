pub mod tabs;
///If implemented it must be absolute. Regarding how layout is handled its up to the parent to do
///so and not the child. If its nested where its like parent has a child which has a child then the
///child after the parent must handle the layout and then the parent handles that child. 
///Basically a tree structure where nested parents are possible.
pub trait Resizable {
    fn resize(&mut self, new_x: usize, new_y: usize);
    ///This is required for a blanket implem
}
