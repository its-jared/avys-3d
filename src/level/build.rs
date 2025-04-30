use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use noise::{BasicMulti, NoiseFn, Perlin};

pub fn build(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Building demo chunk");

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let terrain_height = 70.;
    let noise = BasicMulti::<Perlin>::new(900);

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000.0, 1000.0)
            .subdivisions(200)
    );

    if let Some(VertexAttributeValues::Float32x3(positions))
        = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            let val = noise.get([
                pos[0] as f64 / 300.0,
                pos[2] as f64 / 300.0,
            ]);

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
    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}