use std::f32::consts::PI;

use bevy::{math::ops::powf, pbr::{light_consts::lux, CascadeShadowConfigBuilder}, prelude::*, render::mesh::VertexAttributeValues};
use noise::{BasicMulti, NoiseFn, Perlin};

use crate::level::{gen_data::GenData, Chunk};

use super::{gen_data::BiomeData, ChunkStore};

/*fn get_biome(
    gen_data: &GenData, 
    noise: &BasicMulti<Perlin>, 
    pos: IVec2
) -> BiomeData {
    let biome_noise = noise.get([
        (pos.x as i64 + gen_data.biome_noise.offs) * gen_data.biome_noise.scale,
        (pos.y as i64 + gen_data.biome_noise.offs) * gen_data.biome_noise.scale,
        0.0
    ]);

    for biome in gen_data.biomes {

    }

    ()
}*/

pub struct BuildChunk(pub IVec2);
impl Command for BuildChunk {
    fn apply(self, world: &mut World) -> () {
        /*let gen_data = world
            .get_resource::<GenData>()
            .expect("GenData to be available");*/

        if world
            .get_resource_mut::<ChunkStore>()
            .expect("ChunkStore to be available")
            .0
            .get(&self.0)
            .is_some()
        {
            warn!("Chunk {} already exists", self.0);
            return;
        };

        info!("Building chunk at {}", self.0);

        world.spawn((
            DirectionalLight::default(),
            Transform::from_xyz(4.0, 8.0, 4.0),
        ));
    
        let terrain_height = 70.;
        let noise = BasicMulti::<Perlin>::new(900);
        let mesh_size = 1000.;
    
        let mut terrain = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(mesh_size, mesh_size)
                .subdivisions(200)
        );
    
        if let Some(VertexAttributeValues::Float32x3(positions))
            = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                let mut val = noise.get([
                    (pos[0] as f64
                        + (mesh_size as f64
                            * self.0.x as f64))
                        / 300.,
                    (pos[2] as f64
                        + (mesh_size as f64
                            * self.0.y as f64))
                        / 300.,
                ]);

                val = powf(val as f32, 2.0) as f64;
    
                pos[1] = val as f32 * terrain_height;
            }
    
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_, g, _]| {
                    let g = *g / terrain_height * 2.;
    
                    if g > 0.8 {
                        (Color::LinearRgba(LinearRgba {
                            red: 20.,
                            green: 20.,
                            blue: 20.,
                            alpha: 1.,
                        }))
                        .to_linear()
                        .to_f32_array()
                    } else if g > 0.3 {
                        Color::from(Color::srgb(1.0, 1.0, 0.0))
                            .to_linear()
                            .to_f32_array()
                    } else if g < -0.8 {
                        Color::BLACK.to_linear().to_f32_array()
                    } else {
                        (Color::srgb(0.0, 1.0, 0.0).to_linear())
                            .to_f32_array()
                    }
                })
                .collect();
    
            terrain.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            );
        }
        terrain.compute_normals();
    
        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("meshes db to be available")
            .add(terrain);
        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("StandardMaterial db to be available")
            .add(Color::WHITE);

        world.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(
                self.0.x as f32 * mesh_size,
                0.,
                self.0.y as f32 * mesh_size,
            ),
            Chunk,
        ));
    }
}

pub fn setup_world(mut commands: Commands) {
    commands.queue(BuildChunk(IVec2::new(-1, -1)));
    commands.queue(BuildChunk(IVec2::new(-1, 0)));
    commands.queue(BuildChunk(IVec2::new(-1, 1)));
    commands.queue(BuildChunk(IVec2::new(0, -1)));
    commands.queue(BuildChunk(IVec2::new(0, 0)));
    commands.queue(BuildChunk(IVec2::new(0, 1)));
    commands.queue(BuildChunk(IVec2::new(1, -1)));
    commands.queue(BuildChunk(IVec2::new(1, 0)));
    commands.queue(BuildChunk(IVec2::new(1, 1)));

    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, -0.4, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config,
    ));
}

// This is taken from bevy's example, this will be changed to actually match day / night cycles. 
pub fn dynamic_scene(mut suns: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
    suns.iter_mut()
        .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * PI / 10.0));
}