use crate::{
    lisp::{
        environment::EnvironmentLayer,
        eval::{eval, EvalError},
        expression::ForeignDataWrapper,
        Environment, Expression,
    },
    raytracer::{scene::Scene, types::Light},
};

use super::{
    camera::Camera,
    plane::{Checkerboard, Plane},
    sphere::Sphere,
    types::{Color, Material, Point3, RTObjectWrapper, Vector3},
};

pub fn point(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [x, y, z]: [Expression; 3] = expr.try_into()?;

    let x: f64 = eval(env, x)?.try_into()?;
    let y: f64 = eval(env, y)?.try_into()?;
    let z: f64 = eval(env, z)?.try_into()?;

    Ok(ForeignDataWrapper::new(Point3::new(x, y, z)).into())
}

pub fn vector(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [x, y, z]: [Expression; 3] = expr.try_into()?;

    let x: f64 = eval(env, x)?.try_into()?;
    let y: f64 = eval(env, y)?.try_into()?;
    let z: f64 = eval(env, z)?.try_into()?;

    Ok(ForeignDataWrapper::new(Vector3::new(x, y, z)).into())
}

pub fn color(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [r, g, b]: [Expression; 3] = expr.try_into()?;

    let r: f64 = eval(env, r)?.try_into()?;
    let g: f64 = eval(env, g)?.try_into()?;
    let b: f64 = eval(env, b)?.try_into()?;

    Ok(ForeignDataWrapper::new(Color::new(r, g, b)).into())
}

pub fn light(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [pos, col]: [Expression; 2] = expr.try_into()?;

    let pos: ForeignDataWrapper<Point3> = eval(env, pos)?.try_into()?;
    let col: ForeignDataWrapper<Color> = eval(env, col)?.try_into()?;

    let pos: Point3 = *pos;
    let col: Color = *col;

    Ok(ForeignDataWrapper::new(Light::new(pos, col)).into())
}

pub fn material(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [amb, dif, spe, shi, mir]: [Expression; 5] = expr.try_into()?;

    let amb: ForeignDataWrapper<Color> = eval(env, amb)?.try_into()?;
    let dif: ForeignDataWrapper<Color> = eval(env, dif)?.try_into()?;
    let spe: ForeignDataWrapper<Color> = eval(env, spe)?.try_into()?;
    let shi: f64 = eval(env, shi)?.try_into()?;
    let mir: f64 = eval(env, mir)?.try_into()?;

    let amb: Color = *amb;
    let dif: Color = *dif;
    let spe: Color = *spe;

    Ok(ForeignDataWrapper::new(Material::new(amb, dif, spe, shi, mir)).into())
}

pub fn sphere(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [pos, rad, mat]: [Expression; 3] = expr.try_into()?;

    let pos: ForeignDataWrapper<Point3> = eval(env, pos)?.try_into()?;
    let rad: f64 = eval(env, rad)?.try_into()?;
    let mat: ForeignDataWrapper<Material> = eval(env, mat)?.try_into()?;

    let pos: Point3 = *pos;
    let mat: Material = *mat;

    Ok(ForeignDataWrapper::new(RTObjectWrapper::from(Sphere::new(pos, rad, mat))).into())
}

pub fn plane(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [pos, dir, mat]: [Expression; 3] = expr.try_into()?;

    let pos: ForeignDataWrapper<Point3> = eval(env, pos)?.try_into()?;
    let dir: ForeignDataWrapper<Vector3> = eval(env, dir)?.try_into()?;
    let mat: ForeignDataWrapper<Material> = eval(env, mat)?.try_into()?;

    let pos: Point3 = *pos;
    let dir: Vector3 = *dir;
    let mat: Material = *mat;

    Ok(ForeignDataWrapper::new(RTObjectWrapper::from(Plane::new(pos, dir, mat))).into())
}

pub fn checkerboard(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [pos, norm, mat1, mat2, sca, up]: [Expression; 6] = expr.try_into()?;

    let pos: ForeignDataWrapper<Point3> = eval(env, pos)?.try_into()?;
    let norm: ForeignDataWrapper<Vector3> = eval(env, norm)?.try_into()?;
    let mat1: ForeignDataWrapper<Material> = eval(env, mat1)?.try_into()?;
    let mat2: ForeignDataWrapper<Material> = eval(env, mat2)?.try_into()?;
    let sca: f64 = eval(env, sca)?.try_into()?;
    let up: ForeignDataWrapper<Vector3> = eval(env, up)?.try_into()?;

    let pos: Point3 = *pos;
    let norm: Vector3 = *norm;
    let mat1: Material = *mat1;
    let mat2: Material = *mat2;
    let up: Vector3 = *up;

    Ok(
        ForeignDataWrapper::new(RTObjectWrapper::from(Checkerboard::new(
            pos, norm, mat1, mat2, sca, up,
        )))
        .into(),
    )
}

pub fn scene(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [amb, objs, lgts]: [Expression; 3] = expr.try_into()?;

    let amb: ForeignDataWrapper<Color> = eval(env, amb)?.try_into()?;
    let objs: Vec<Expression> = eval(env, objs)?.try_into()?;
    let lgts: Vec<Expression> = eval(env, lgts)?.try_into()?;

    let mut scene = Scene::new();

    scene.set_ambient(*amb);
    for o in objs {
        let o: ForeignDataWrapper<RTObjectWrapper> = eval(env, o)?.try_into()?;
        scene.add_object(o.clone());
    }
    for l in lgts {
        let l: ForeignDataWrapper<Light> = eval(env, l)?.try_into()?;
        scene.add_light(*l);
    }

    Ok(ForeignDataWrapper::new(scene).into())
}

pub fn scene_add_object(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [sce, obj]: [Expression; 2] = expr.try_into()?;

    let mut sce: ForeignDataWrapper<Scene> = eval(env, sce)?.try_into()?;
    let obj: ForeignDataWrapper<RTObjectWrapper> = eval(env, obj)?.try_into()?;

    sce.add_object(obj.clone());

    Ok(sce.into())
}

pub fn scene_add_light(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [sce, lgt]: [Expression; 2] = expr.try_into()?;

    let mut sce: ForeignDataWrapper<Scene> = eval(env, sce)?.try_into()?;
    let lgt: ForeignDataWrapper<Light> = eval(env, lgt)?.try_into()?;

    sce.add_light(*lgt);

    Ok(sce.into())
}

pub fn camera(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [pos, cnt, up, fovy, w, h]: [Expression; 6] = expr.try_into()?;

    let pos: ForeignDataWrapper<Point3> = eval(env, pos)?.try_into()?;
    let cnt: ForeignDataWrapper<Point3> = eval(env, cnt)?.try_into()?;
    let up: ForeignDataWrapper<Vector3> = eval(env, up)?.try_into()?;
    let fovy: f64 = eval(env, fovy)?.try_into()?;
    let w: i64 = eval(env, w)?.try_into()?;
    let h: i64 = eval(env, h)?.try_into()?;

    Ok(ForeignDataWrapper::new(Camera::new(*pos, *cnt, *up, fovy, w as usize, h as usize)).into())
}

pub fn render(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
    let [cam, sce, dpt, sbp, out]: [Expression; 5] = expr.try_into()?;

    let cam: ForeignDataWrapper<Camera> = eval(env, cam)?.try_into()?;
    let sce: ForeignDataWrapper<Scene> = eval(env, sce)?.try_into()?;
    let dpt: i64 = eval(env, dpt)?.try_into()?;
    let sbp: i64 = eval(env, sbp)?.try_into()?;
    let out: String = eval(env, out)?.try_into()?;

    println!("Rendering to {}...", out);
    let img = cam.render(&sce, dpt as u32, sbp as u32);

    match img.save(out) {
        Ok(_) => Ok(Expression::Nil),
        Err(e) => Err(EvalError::RuntimeError(e.to_string())),
    }
}

/// Adds the raytracing functions to the given environment layer.
pub fn mk_raytrace(layer: &mut EnvironmentLayer) {
    layer.set("point".to_string(), Expression::Function(point));
    layer.set("vector".to_string(), Expression::Function(vector));
    layer.set("color".to_string(), Expression::Function(color));
    layer.set("light".to_string(), Expression::Function(light));
    layer.set("material".to_string(), Expression::Function(material));
    layer.set("plane".to_string(), Expression::Function(plane));
    layer.set(
        "checkerboard".to_string(),
        Expression::Function(checkerboard),
    );
    layer.set("sphere".to_string(), Expression::Function(sphere));
    layer.set("scene".to_string(), Expression::Function(scene));
    layer.set(
        "scene-add-object".to_string(),
        Expression::Function(scene_add_object),
    );
    layer.set(
        "scene-add-light".to_string(),
        Expression::Function(scene_add_light),
    );
    layer.set("camera".to_string(), Expression::Function(camera));
    layer.set("render".to_string(), Expression::Function(render));
}
