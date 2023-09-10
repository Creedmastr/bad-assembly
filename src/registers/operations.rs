use super::reg_struct::Register;

pub trait RegisterOperationsArithmetic {
    fn update(&mut self, x: String, v: &Register<i32>);

    fn search(&self, x: &String) -> Register<i32>;
}

impl RegisterOperationsArithmetic for Vec<Register<i32>> {
    fn update(&mut self, x: String, v: &Register<i32>) {
        let mut index: usize = 0;
        let mut is_updated = false;
        let for_loop = self.clone();

        for item in for_loop {
            match item.name == x {
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

    fn search(&self, x: &String) -> Register<i32> {
        for item in self {
            if &item.name == x {
                return item.clone();
            } else {
                continue;
            }
        }

        return Register {
            name: x.to_string(),
            value: 0,
        };
    }
}
