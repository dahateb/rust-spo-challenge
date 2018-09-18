use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    rooms: Vec<i32>,
    senior: i32,
    junior: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    senior: i32,
    junior: i32
}

impl Input {

    pub fn calculate(&self) -> Vec<Output> {
        println!("Num Seniors: {}", self.senior);
        println!("Num Juniors: {}", self.junior);

        self.rooms.iter()
        .map(|total| {            
            let mut map = HashMap::new();            
            let mut list = Vec::new();
            let mut num_seniors = 0;                                              
            println!("Room size: {}", total);
            while num_seniors * self.senior < *total +1 {
                let mut num_juniors = 0;
                num_seniors +=1;
                let tmp_total_sen = num_seniors * self.senior;
                if  tmp_total_sen >= *total {
                    println!("Possible Seniors: {}, senior: {}, junior: {}", num_seniors * self.senior, num_seniors, num_juniors);
                    map.insert(tmp_total_sen, Output::new(num_seniors, num_juniors));
                    list.push(tmp_total_sen);
                }                    
                while num_seniors * self.senior + num_juniors * self.junior < *total +1 {
                    num_juniors += 1;
                    let tmp_total = num_seniors * self.senior + num_juniors * self.junior;
                    if tmp_total >= *total  {
                        println!("Possible Seniors + Juniors: {}, senior: {}, junior: {}", tmp_total, num_seniors, num_juniors);
                        map.insert(tmp_total, Output::new(num_seniors, num_juniors));
                        list.push(tmp_total);
                    }
                }
            }                        
            list.sort();
            let smallest = list.get(0).unwrap();
            map.get_mut(smallest).unwrap().clone()
        })
        .collect::<Vec<Output>>()
    }
}

impl Output {
    pub fn new(senior: i32, junior: i32) -> Output {
        Output {
            senior: senior,
            junior: junior
        }
    }
}