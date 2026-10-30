use nyra::{App, Material, Mesh, RunError, Thing, ThingID, Transform};
use nyra_math::Quat;

struct AnimationState {
    cube: ThingID,
    angle: f32,
}

fn main() -> Result<(), RunError> {
    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            let cube = ctx
                .world
                .spawn(Thing::new(Transform::default(), mesh).with_material(material));

            AnimationState { cube, angle: 0.0 }
        })
        .run_each_frame_with_state(|ctx, state| {
            state.angle += ctx.delta_time;

            ctx.world[state.cube].transform.rotation = Quat::from_rotation_y(state.angle - 0.6);
        })
}
