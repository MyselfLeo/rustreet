use std::f64::consts::PI;

/// Represent a bounding box (a square on the map)
#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub coo: [f64; 4], // [min_lat, min_lon, max_lat, max_lon]
    pub dim_deg: [f64; 2], // [delta lat, delta lon] (basically, width and height)
    pub dim_km: [f64; 2], // [delta lat in km, delta lon in km] (basically, width and height)
}

impl BoundingBox {

    /// Return a new bounding box
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> BoundingBox {
        let coo = [min_lat, min_lon, max_lat, max_lon];

        let mut bounding_box = BoundingBox{coo, dim_deg: [0.0, 0.0], dim_km: [0.0, 0.0]};
        bounding_box.compute_size();
        bounding_box.resize();

        bounding_box
    }



    /// Compute this bounding box width and height
    pub fn compute_size(&mut self) {
        self.dim_deg[0] = self.coo[2] - self.coo[0];
        self.dim_deg[1] = self.coo[3] - self.coo[1];
        
        self.dim_km = lat_lon_to_km(self.dim_deg);
    }


    /// Resize this bounding box to represent a square. The size of the square is the smallest side of the bounding box.
    /// The bounding box is resize accordingly to its size in kilometers, not in lon/lat degrees.
    pub fn resize(&mut self) {

        // Compute the new size of the bounding box
        let new_dim_km = [self.dim_km[0].min(self.dim_km[1]), self.dim_km[0].min(self.dim_km[1])];

        // Compute the new bounding box in degrees
        let new_dim_lat_lon = km_to_lat_lon(new_dim_km);

        // Compute the new coordinates of the bounding box
        let delta_width = (self.dim_deg[0] - new_dim_lat_lon[0]) / 2.0;
        let delta_height = (self.dim_deg[1] - new_dim_lat_lon[1]) / 2.0;

        self.coo[0] += delta_width;
        self.coo[2] -= delta_width;
        self.coo[1] += delta_height;
        self.coo[3] -= delta_height;

        // Compte new size
        self.compute_size();
    }


    /// Modify the bounding box to zoom in or out by a given factor.
    /// A factor > 1 will zoom in, a factor < 1 will zoom out.
    pub fn zoom(&mut self, factor: f64) {
        let new_width = self.dim_deg[0] / factor;
        let new_height = self.dim_deg[1] / factor;

        let width_difference = (self.dim_deg[0] - new_width) / 2.0;
        let height_difference = (self.dim_deg[1] - new_height) / 2.0;

        self.coo[0] += height_difference;
        self.coo[2] -= height_difference;
        self.coo[1] += width_difference;
        self.coo[3] -= width_difference;

        self.resize();
    }


    /// Modify the bounding box to move it by a given vector
    /// (in lat/lon coordinates)
    pub fn translate(&mut self, vector: [f64; 2]) {
        self.coo[0] += vector[0];
        self.coo[1] += vector[1];
        self.coo[2] += vector[0];
        self.coo[3] += vector[1];

        self.resize();
    }
}




/// Convert a vector (lat, lon) in degrees to a vector (lat, lon) in kilometers
fn lat_lon_to_km(vector: [f64; 2]) -> [f64; 2] {
    let lat_deg = vector[0];
    let lon_deg = vector[1];

    // Convert given latitude to radians
    let lat_rad = lat_deg * PI / 180.0;

    // Convert considering the earth like a perfect sphere
    [lat_deg * 110.574, lon_deg * 111.320 * lat_rad.cos()]
}


/// Convert a vector (lat, lon) in kilometers to a vector (lat, lon) in degrees
fn km_to_lat_lon(vector: [f64; 2]) -> [f64; 2] {
    let lat_km = vector[0];
    let lon_km = vector[1];

    // Convert given latitude to degrees and radians
    let lat_deg = lat_km / 110.574;
    let lat_rad = lat_deg * PI / 180.0;

    // Convert considering the earth like a perfect sphere
    [lat_deg, lon_km / 111.320 / lat_rad.cos()]
}