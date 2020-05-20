pub struct NumArray {
    numbers: Vec<i32>
}

impl NumArray {
    pub fn new(mut numbers: Vec<i32>) -> Self {
        let mut sum = 0;
        for n in numbers.iter_mut() {
            sum += *n;
            *n = sum;
        }
        NumArray { numbers: numbers }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        let top = self.numbers[j as usize];
        /*let bottom = match i {    //functional programming
            0 => 0,
            _ => self.numbers[i as usize - 1]
        };*/
        let bottom = if i == 0 {
            0
        } else {
            self.numbers[i as usize - 1]
        };
        top - bottom
        
        /*let mut sum = 0;
        for index in i..=j {
            sum += self.numbers[index as usize]
        }
        sum*/
    }
}
