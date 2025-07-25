use crate::app::Vertex;

// pub const MANDELBROT_CANVAS_POSITIONS :[Vertex;4]=[
//     Vertex{position: [ 0.5, 0.5, 0.0, 1.]},
//     Vertex{position: [ 0.5,-0.5, 0.0, 1.]},
//     Vertex{position: [-0.5,-0.5, 0.0, 1.]},
//     Vertex{position: [-0.5, 0.5, 0.0, 1.]},
// ];

pub const MANDELBROT_CANVAS_POSITIONS :[Vertex;4]=[
    Vertex{position: [ 1., 1., 0.0, 1.]},
    Vertex{position: [ 1.,-1., 0.0, 1.]},
    Vertex{position: [-1.,-1., 0.0, 1.]},
    Vertex{position: [-1., 1., 0.0, 1.]},
];

pub const CANVAS_INDICES: [u8;4] = [0,1,2,3];

pub const MAT4_ID :[[f32;4];4]= [
    [1.,0.,0.,0.],
    [0.,1.,0.,0.],
    [0.,0.,1.,0.],
    [0.,0.,0.,1.],
];

#[macro_export]
macro_rules! info_display{
    ()=>{
        r#"
current set : {}      to change the drawed set press J => Julia, M => Mandelbrot
current start value : {} +i{} use the arrows to move it
current palette offset :{}    use + and - to change it
press h to see infos 
        "#
    };
}
