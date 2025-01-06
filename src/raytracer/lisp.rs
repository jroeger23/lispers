use crate::raytracer::{scene::Scene, types::Light};

use lispers_macro::{native_lisp_function, native_lisp_function_proxy};

use lispers_core::lisp::{
    environment::EnvironmentLayer,
    eval::{eval, EvalError},
    expression::ForeignDataWrapper,
    Environment, Expression,
};

use super::{
    camera::Camera,
    plane::{Checkerboard, Plane},
    sphere::Sphere,
    types::{Color, Material, Point3, RTObjectWrapper, Vector3},
};

#[native_lisp_function(eval)]
pub fn point(x: f64, y: f64, z: f64) -> Result<ForeignDataWrapper<Point3>, EvalError> {
    Ok(ForeignDataWrapper::new(Point3::new(x, y, z)))
}

#[native_lisp_function(eval)]
pub fn vector(x: f64, y: f64, z: f64) -> Result<ForeignDataWrapper<Vector3>, EvalError> {
    Ok(ForeignDataWrapper::new(Vector3::new(x, y, z)))
}

#[native_lisp_function(eval)]
pub fn color(r: f64, g: f64, b: f64) -> Result<ForeignDataWrapper<Color>, EvalError> {
    Ok(ForeignDataWrapper::new(Color::new(r, g, b)))
}

#[native_lisp_function(eval)]
pub fn light(
    pos: ForeignDataWrapper<Point3>,
    col: ForeignDataWrapper<Color>,
) -> Result<ForeignDataWrapper<Light>, EvalError> {
    Ok(ForeignDataWrapper::new(Light::new(*pos, *col)))
}

#[native_lisp_function(eval)]
pub fn material(
    amb: ForeignDataWrapper<Color>,
    dif: ForeignDataWrapper<Color>,
    spe: ForeignDataWrapper<Color>,
    shi: f64,
    mir: f64,
) -> Result<ForeignDataWrapper<Material>, EvalError> {
    Ok(ForeignDataWrapper::new(Material::new(
        *amb, *dif, *spe, shi, mir,
    )))
}

#[native_lisp_function(eval)]
pub fn sphere(
    pos: ForeignDataWrapper<Point3>,
    rad: f64,
    mat: ForeignDataWrapper<Material>,
) -> Result<ForeignDataWrapper<RTObjectWrapper>, EvalError> {
    Ok(ForeignDataWrapper::new(RTObjectWrapper::from(Sphere::new(*pos, rad, *mat))).into())
}

#[native_lisp_function(eval)]
pub fn plane(
    pos: ForeignDataWrapper<Point3>,
    dir: ForeignDataWrapper<Vector3>,
    mat: ForeignDataWrapper<Material>,
) -> Result<ForeignDataWrapper<RTObjectWrapper>, EvalError> {
    Ok(ForeignDataWrapper::new(RTObjectWrapper::from(Plane::new(*pos, *dir, *mat))).into())
}

#[native_lisp_function(eval)]
pub fn checkerboard(
    pos: ForeignDataWrapper<Point3>,
    norm: ForeignDataWrapper<Vector3>,
    mat1: ForeignDataWrapper<Material>,
    mat2: ForeignDataWrapper<Material>,
    sca: f64,
    up: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<RTObjectWrapper>, EvalError> {
    Ok(
        ForeignDataWrapper::new(RTObjectWrapper::from(Checkerboard::new(
            *pos, *norm, *mat1, *mat2, sca, *up,
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

#[native_lisp_function]
pub fn scene_add_object(
    mut sce: ForeignDataWrapper<Scene>,
    obj: ForeignDataWrapper<RTObjectWrapper>,
) -> Result<ForeignDataWrapper<Scene>, EvalError> {
    sce.add_object(obj.clone());
    Ok(sce)
}

#[native_lisp_function]
pub fn scene_add_light(
    mut sce: ForeignDataWrapper<Scene>,
    lgt: ForeignDataWrapper<Light>,
) -> Result<ForeignDataWrapper<Scene>, EvalError> {
    sce.add_light(*lgt);
    Ok(sce)
}

native_lisp_function_proxy!(
    fname = scene_add,
    eval,
    dispatch = scene_add_object,
    dispatch = scene_add_light
);

#[native_lisp_function(eval)]
pub fn camera(
    pos: ForeignDataWrapper<Point3>,
    cnt: ForeignDataWrapper<Point3>,
    up: ForeignDataWrapper<Vector3>,
    fovy: f64,
    w: i64,
    h: i64,
) -> Result<ForeignDataWrapper<Camera>, EvalError> {
    Ok(ForeignDataWrapper::new(Camera::new(
        *pos, *cnt, *up, fovy, w as usize, h as usize,
    )))
}

#[native_lisp_function(eval)]
pub fn render(
    cam: ForeignDataWrapper<Camera>,
    sce: ForeignDataWrapper<Scene>,
    dpt: i64,
    sbp: i64,
    out: String,
) -> Result<Expression, EvalError> {
    println!("Rendering to {}...", out);
    let img = cam.render(&sce, dpt as u32, sbp as u32);

    match img.save(out) {
        Ok(_) => Ok(Expression::Nil),
        Err(e) => Err(EvalError::RuntimeError(e.to_string())),
    }
}

#[native_lisp_function]
pub fn vadd_vv(
    a: ForeignDataWrapper<Vector3>,
    b: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<Vector3>, EvalError> {
    Ok(ForeignDataWrapper::new(*a + *b))
}

#[native_lisp_function]
pub fn vadd_vp(
    a: ForeignDataWrapper<Vector3>,
    b: ForeignDataWrapper<Point3>,
) -> Result<ForeignDataWrapper<Point3>, EvalError> {
    Ok(ForeignDataWrapper::new(*b + *a))
}

#[native_lisp_function]
pub fn vadd_pv(
    a: ForeignDataWrapper<Point3>,
    b: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<Point3>, EvalError> {
    Ok(ForeignDataWrapper::new(*a + *b))
}

native_lisp_function_proxy!(
    fname = vadd,
    eval,
    dispatch = vadd_vv,
    dispatch = vadd_vp,
    dispatch = vadd_pv
);

#[native_lisp_function]
pub fn vsub_vv(
    a: ForeignDataWrapper<Vector3>,
    b: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<Vector3>, EvalError> {
    Ok(ForeignDataWrapper::new(*a - *b))
}

#[native_lisp_function]
pub fn vsub_vp(
    a: ForeignDataWrapper<Vector3>,
    b: ForeignDataWrapper<Point3>,
) -> Result<ForeignDataWrapper<Point3>, EvalError> {
    Ok(ForeignDataWrapper::new(*b - *a))
}

#[native_lisp_function]
pub fn vsub_pv(
    a: ForeignDataWrapper<Point3>,
    b: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<Point3>, EvalError> {
    Ok(ForeignDataWrapper::new(*a - *b))
}

native_lisp_function_proxy!(
    fname = vsub,
    eval,
    dispatch = vsub_vv,
    dispatch = vsub_vp,
    dispatch = vsub_pv
);

#[native_lisp_function]
pub fn vmul_vs(
    a: ForeignDataWrapper<Vector3>,
    b: f64,
) -> Result<ForeignDataWrapper<Vector3>, EvalError> {
    Ok(ForeignDataWrapper::new(*a * b))
}

#[native_lisp_function]
pub fn vmul_sv(
    a: f64,
    b: ForeignDataWrapper<Vector3>,
) -> Result<ForeignDataWrapper<Vector3>, EvalError> {
    Ok(ForeignDataWrapper::new(*b * a))
}

native_lisp_function_proxy!(fname = vmul, eval, dispatch = vmul_vs, dispatch = vmul_sv);

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
    layer.set("vadd".to_string(), Expression::Function(vadd));
    layer.set("vsub".to_string(), Expression::Function(vsub));
    layer.set("vmul".to_string(), Expression::Function(vmul));
}
