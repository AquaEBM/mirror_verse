use super::*;

#[derive(Clone, Copy)]
pub struct EuclideanSphereMirror<const D: usize> {
    center: SVector<f32, D>,
    radius: f32,
}

impl<const D: usize> Mirror<D> for EuclideanSphereMirror<D> {
    fn append_intersecting_points(&self, ray: &Ray<D>, list: &mut Vec<Tangent<D>>) {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let delta = b * b - a * c;

        if delta > 0. {
            let sqrt_delta = delta.sqrt();
            let neg_b = -b;
            let t = [neg_b - sqrt_delta / a, neg_b + sqrt_delta / a];
            for &t in t.iter() {
                if t > 0. {
                    let origin = ray.at(t);
                    let normal = Unit::new_normalize(origin - self.center);
                    list.push(Tangent::Normal { origin, normal });
                }
            }
        }
    }

    fn get_json_type() -> String {
        "sphere".into()
    }

    fn get_json_type_inner(&self) -> String {
        "sphere".into()
    }

    fn from_json(json: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        /* example json
        {
            "center": [1., 2., 3.],
            "radius": 4.,
        }
         */

        let center = json
            .get("center")
            .and_then(serde_json::Value::as_array)
            .map(Vec::as_slice)
            .and_then(util::json_array_to_vector)
            .ok_or("Failed to parse center")?;

        let radius = json
            .get("radius")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Failed to parse radius")? as f32;

        Ok(Self { center, radius })
    }

    fn to_json(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        todo!()
    }

    fn render_data(&self, display: &gl::Display) -> Vec<Box<dyn render::RenderData>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}