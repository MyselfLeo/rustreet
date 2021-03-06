use crate::geo::BoundingBox;



pub struct AsciiMap {
    display_box: BoundingBox,
    data: Vec<Vec<String>>,
    height: u32,
    pub width: u32,
    is_decorated: bool,
}


impl AsciiMap {

    /// Return a new AsciiMap
    pub fn from(display_box: BoundingBox, data: Vec<Vec<String>>) -> AsciiMap {
        let height = data.len() as u32;
        let width = data[0].len() as u32;

        AsciiMap {
            display_box,
            data,
            height,
            width,
            is_decorated: false,
        }
    }



    /// Double each character of the asciimap.
    /// The width will be doubled.
    /// It makes the map more orthogonal when displayed in a terminal.
    pub fn double(&mut self) {
        let mut new_data: Vec<Vec<String>> = Vec::new();

        for x in 0..self.height {
            new_data.push(Vec::new());

            // Add 2 times the characters
            for y in 0..self.width {
                new_data[x as usize].push(String::from(&self.data[x as usize][y as usize]));
                //new_data[x as usize].push(String::from(&self.data[x as usize][y as usize]));
                new_data[x as usize].push(String::from(" "));
            }
        }

        self.data = new_data;
        self.width = self.width * 2;
    }




    /// Return a copy of this AsciiMap, with added decoration (border, overlay)
    pub fn with_decoration(&self) -> AsciiMap {
        let mut new_data: Vec<Vec<String>> = Vec::new();
        new_data.push(Vec::new());

        // Add the top border
        new_data[0].push(String::from("╚"));
        for _ in 0..self.width {new_data[0].push(String::from("═"));}
        new_data[0].push(String::from("╝"));

        // Add the whole ascii map, with borders on the left and the right
        for x in 1..self.height as usize {
            new_data.push(Vec::new());

            new_data[x].push(String::from("║"));

            for y in 0..self.width as usize {
                new_data[x].push(String::from(&self.data[x][y]));
            }

            new_data[x].push(String::from("║"));
        }

        new_data.push(Vec::new());

        // Add the bottom border
        new_data[self.height as usize].push(String::from("╔"));
        for _ in 0..self.width {new_data[self.height as usize].push(String::from("═"));}
        new_data[self.height as usize].push(String::from("╗"));



        // Add the compass rose (the north arrow) on the map
        new_data[3][(self.width - 3) as usize] = String::from("⇯");
        new_data[2][(self.width - 3) as usize] = String::from("N");


        // Return the decorated AsciiMap
        let mut res = AsciiMap::from(self.display_box, new_data);
        res.is_decorated = true;
        res
    }




    /// Return the approximate distance a character represents (in km)
    fn get_scale(&self) -> f64 {
        self.display_box.dim_km[0] / self.width as f64
    }



    /// Return a string representing the scale of this map.
    fn get_scale_repr(&self) -> String {

        let char_size = self.get_scale();           // in km

        // Compute the distance represented by the scale.
        let scale_value = char_size * 10.0;

        // TODO: Change the size of the scale dynamically
        let mut scale_repr = String::from("⊢────────⊣ ");

        if scale_value < 10.0 {
            let number_rounded = (scale_value * 100.0).round() as u64 * 10;
            scale_repr.push_str(&number_rounded.to_string());
            scale_repr.push_str("m");
        }
        else {
            let number_rounded = (scale_value * 10.0).round() as u64 / 10;
            scale_repr.push_str(&number_rounded.to_string());
            scale_repr.push_str("km");
        }

        scale_repr
    }





    /// Print the ascii map to the terminal
    pub fn print(&self) {
        for x in 0..(self.height) as usize {
            for y in 0..(self.width) as usize {
                print!("{}", self.data[self.height as usize - x - 1][y]);
            }
            print!("\n");
        }

        // Print the scale
        println!("{}", self.get_scale_repr());
    }
}