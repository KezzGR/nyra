use nyra::{App, Entity, EntityID, Mesh, RunError};
use nyra_math::{Quat, Transform};

struct AnimationState {
    cube: EntityID,
    angle: f32,
}

fn main() -> Result<(), RunError> {
    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let cube = ctx
                .world
                .spawn(Entity::from_mesh(Transform::default(), mesh));

            AnimationState { cube, angle: 0.0 }
        })
        .run_each_frame_with_state(|ctx, state| {
            state.angle += ctx.delta_time;

            ctx.world[state.cube].transform.rotation = Quat::from_rotation_y(state.angle - 0.6);
        })
}
