use std::collections::HashMap;

struct Array {
    // Should name better but eh
    a: Vec<i32>,
}

impl Array {
    fn remove_duplicates_without_sorting(&mut self) {
        let mut encountered_elements: HashMap<i32, bool> = HashMap::new();
        let mut temp: Vec<i32> = Vec::new();
        for i in self.a.iter() {
            let value = encountered_elements.get(&i);
            if value == None {
                encountered_elements.insert(*i, true);
                temp.append(&mut [*i].to_vec());
            }
        }
        self.a = temp;
    }
    fn stalin_sort(&mut self) {
        let mut prev_val = self.a[0];
        let mut temp: Vec<i32> = Vec::new();
        temp.append(&mut [prev_val].to_vec());
        for i in 1..self.a.len() {
            if self.a[i] > prev_val {
                temp.append(&mut [self.a[i]].to_vec());
                prev_val = self.a[i];
            }
        }
        self.a = temp
    }
    fn print_vec(&self) {
        for val in self.a.iter() {
            println!("{}", val)
        }
    }
}

fn main() {
    let mut a = Array {
        a: [1, 2, 1, 4, 9, 6, 10, 8].to_vec(),
    };
    // Remove duplicates without sorting then sort using stalin SORT
    a.remove_duplicates_without_sorting();
    a.print_vec();
    a.stalin_sort();
    a.print_vec();
}
