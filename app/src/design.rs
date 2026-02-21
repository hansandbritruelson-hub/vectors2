#[derive(Clone, Debug)]
pub struct VectorHandle {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct VectorPoint {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub handle_in: Option<VectorHandle>,
    pub handle_out: Option<VectorHandle>,
}

#[derive(Clone, Debug)]
pub struct VectorObject {
    pub id: String,
    pub name: String,
    pub object_type: String,
    pub z_index: f32,
    pub closed: bool,
    pub points: Vec<VectorPoint>,
    pub path_data: String,
}

#[derive(Clone, Debug)]
pub struct VectorFile {
    pub path: String,
    pub objects: Vec<VectorObject>,
}
