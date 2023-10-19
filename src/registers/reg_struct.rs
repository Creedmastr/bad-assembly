#[derive(Clone, Debug)]
pub struct Register<T> {
    pub name: String,
    pub value: T
}

impl<T> Register<T> {
    pub fn update_value(&self, x: T) -> Self {
        let new = Register {
            name: self.name.clone(),

            value: x
        };

        return new;
    }
}