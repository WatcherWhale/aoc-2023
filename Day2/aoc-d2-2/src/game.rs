pub struct Game {
    pub id: i32,
    data_points : Vec<DataPoint>,
}

impl Game {
    pub fn from(game_str : String) -> Game {
        let cleaned = game_str.replace(" ", "");
        let splitted : Vec<_> = cleaned.split(":").collect();
        let id_str = splitted[0].replace("Game", "");

        let mut dps : Vec<DataPoint> = Vec::new();
        for dp in splitted[1].split(";") {
            dps.push(DataPoint::from(&dp.to_string()))
        }

        return Game { id: id_str.parse().unwrap(), data_points: dps };
    }

    pub fn fewest_possible(&self) -> DataPoint {
        let mut red : i32 = 0;
        let mut green : i32 = 0;
        let mut blue : i32 = 0;

        for (_, dp) in self.data_points.iter().enumerate() {
            if dp.red > red {
                red = dp.red;
            }

            if dp.green > green {
                green = dp.green;
            }

            if dp.blue > blue {
                blue = dp.blue;
            }
        }

        return DataPoint{red, green, blue};
    }
}

pub struct DataPoint{
    red: i32,
    green: i32,
    blue: i32,
}

impl DataPoint {
    fn from(str: &String) -> DataPoint {
        let mut red : i32 = 0;
        let mut green : i32 = 0;
        let mut blue : i32 = 0;

        for cube_str in str.split(",") {
            if cube_str.contains("red") {
                red = get_amount(cube_str, "red")
            }

            if cube_str.contains("green") {
                green = get_amount(cube_str, "green")
            }

            if cube_str.contains("blue") {
                blue = get_amount(cube_str, "blue")
            }
        }


        return DataPoint{red, green, blue};
    }

    pub fn power(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
}

fn get_amount(str: &str, key: &str) -> i32 {
    return str.replace(key, "").parse().unwrap();
}
