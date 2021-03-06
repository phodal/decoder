use crate::r8::graph::app_view::AppView;
use crate::r8::graph::program_method::ProgramMethod;
use core::fmt;

pub trait CodeClone {
    fn clone_box(&self) -> Box<dyn Code>;
}

impl<T> CodeClone for T
where
    T: 'static + Code + Clone,
{
    fn clone_box(&self) -> Box<dyn Code> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Code> {
    fn clone(&self) -> Box<dyn Code> {
        self.clone_box()
    }
}

impl fmt::Debug for dyn Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi")
    }
}

pub trait Code: CodeClone {
    fn build_ir(&self, method: ProgramMethod, app_view: AppView);
}
