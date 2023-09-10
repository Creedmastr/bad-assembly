use super::reg_struct::Register;

pub trait RegisterOperationsArithmetic {
    fn update(&mut self, v: &Register<i32>);

    fn search(&self, x: &String) -> (Register<i32>, bool); // Returns 0 or 1 depending on whether the register was just created or not
}

impl RegisterOperationsArithmetic for Vec<Register<i32>> {
    fn update(&mut self, v: &Register<i32>) {
        let mut index: usize = 0;
        let mut is_updated = false;
        let for_loop = self.clone();

        for item in for_loop {
            match item.name == v.name {
                true => {
                    self[index] = v.clone();
                    is_updated = true;
                }

                false => {
                    continue;
                }
            }

            index += 1;
        }

        if !is_updated {
            self.push(v.clone());
        }
    }

    fn search(&self, x: &String) -> (Register<i32>, bool) {
        for item in self {
            if &item.name == x {
                return (item.clone(), false);
            } else {
                continue;
            }
        }

        return (Register {
            name: x.to_string(),
            value: 0,
        }, true);
    }
}


pub trait RegisterOperationsString {
    fn update(&mut self, v: &Register<String>);

    fn search(&self, x: &String) -> (Register<String>, bool); // Returns 0 or 1 depending on whether the register was just created or not
}

impl RegisterOperationsString for Vec<Register<String>> {
    fn update(&mut self, v: &Register<String>) {
        let mut index: usize = 0;
        let mut is_updated = false;
        let for_loop = self.clone();

        for item in for_loop {
            match item.name == v.name {
                true => {
                    self[index] = v.clone();
                    is_updated = true;
                }

                false => {
                    continue;
                }
            }

            index += 1;
        }

        if !is_updated {
            self.push(v.clone());
        }
    }

    fn search(&self, x: &String) -> (Register<String>, bool) {
        for item in self {
            if &item.name == x {
                return (item.clone(), false);
            } else {
                continue;
            }
        }

        return (Register {
            name: x.to_string(),
            value: String::from("Null"),
        }, true);
    }
}
