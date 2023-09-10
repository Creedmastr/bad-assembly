#[derive(Clone, Debug)]
pub struct Register<T> {
    pub name: String,
    pub value: T
}

impl Register<i32> {
    pub fn update_value(&self, x: i32) -> Self {
        let new = Register {
            name: self.name.clone(),

            value: x
        };

        return new;
    }
}

impl Register<String> {
    pub fn update_value(&self, x: String) -> Self {
        let new = Register {
            name: self.name.clone(),

            value: x
        };

        return new;
    }
}