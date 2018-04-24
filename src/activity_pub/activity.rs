use activity_pub::actor::Actor;
use activity_pub::object::Object;

pub struct Create<'a, T, U> where T: Actor + 'static, U: Object {
    by: &'a T,
    object: U
}

impl<'a, T: Actor, U: Object> Create<'a, T, U> {
    pub fn new(by: &T, obj: U) -> Create<T, U> {
        Create {
            by: by,
            object: obj
        }
    }
}
