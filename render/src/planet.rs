use world;
use kiss3d::resource::Mesh;
use gl::types::{GLfloat, GLuint};
use na::{Point3, Vector3};

fn push_p(points: &mut Vec<Point3<GLfloat>>, p: world::Vec3) {
    points.push(Point3::new(p.x as f32, p.y as f32, p.z as f32));
}

fn push_n(points: &mut Vec<Vector3<GLfloat>>, p: world::Vec3) {
    points.push(Vector3::new(p.x as f32, p.y as f32, p.z as f32));
}

fn push_f(faces: &mut Vec<Point3<GLuint>>, p1: usize, p2: usize, p3: usize) {
    faces.push(Point3::new(p1 as u32, p2 as u32, p3 as u32));    
}

pub fn planet_mesh(p: &world::Planet) -> Mesh {
    let mut points = Vec::new();
    let mut faces = Vec::new();
    let mut normals = Vec::new();

    let mut face = 0;

    loop { 
        let scale_range = -(p.scale() as isize/2)..p.scale() as isize/2 + 1;
        for x in scale_range.clone() {
            for y in scale_range.clone() {
                push_p(&mut points, p.warped(face, x, y));
                push_n(&mut normals, p.warped(face, x, y));
            }
        }

        let base = face as usize * p.scale() * p.scale();
        let mut f = 0;
        loop {
            if f + 1 == (p.scale() - 1) * p.scale() {
                break;
            }

            if f % p.scale() + 1 == p.scale() {
                f += 1;
                continue;
            }

            if face == 0 || face == 3 || face == 4 {
                push_f(&mut faces, f + p.scale() + base, f + base, f + p.scale() + 1 + base);
                push_f(&mut faces, f + p.scale() + 1 + base, f + base, f + 1 + base);
            }
            else {
                push_f(&mut faces, f + base, f + p.scale() + base, f + p.scale() + 1 + base);
                push_f(&mut faces, f + base, f + p.scale() + 1 + base, f + 1 + base);
            }
            f += 1;
        }

        face += 1;
        if face == 6 { break; }
    }

    Mesh::new(points, faces, Some(normals), None, true)   
}