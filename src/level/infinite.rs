use bevy::prelude::*;

use crate::{data::GameConfig, player::Player};
use super::{build::BuildChunk, Chunk, ChunkStore};

pub fn manage_chunks(
    mut commands: Commands,
    mut current_chunk: Local<IVec2>,
    player: Query<&Transform, With<Player>>,
    mut chunk_store: ResMut<ChunkStore>,
    chunks: Query<
        (Entity, &Mesh3d),
        With<Chunk>,
    >,
    config: Res<GameConfig>,
) {
    if !config.defaults.infinite_terrian {
        return;
    }

    let chunk_size = 1000.;

    let Ok(transform) = player.single() else {
        warn!("No player!");
        return;
    };

    let xz = (transform.translation.xz() / chunk_size)
        .trunc()
        .as_ivec2();

    if *current_chunk != xz {
        *current_chunk = xz;
        let chunks_to_render = [
            *current_chunk + IVec2::new(-1, -1),
            *current_chunk + IVec2::new(-1, 0),
            *current_chunk + IVec2::new(-1, 1),
            *current_chunk + IVec2::new(0, -1),
            *current_chunk + IVec2::new(0, 0),
            *current_chunk + IVec2::new(0, 1),
            *current_chunk + IVec2::new(1, -1),
            *current_chunk + IVec2::new(1, 0),
            *current_chunk + IVec2::new(1, 1),
        ];
        // extract_if is perfect here, but its nightly
        let chunks_to_despawn: Vec<(IVec2, Handle<Mesh>)> =
            chunk_store
                .0
                .clone()
                .into_iter()
                .filter(|(key, _)| {
                    !chunks_to_render.contains(&key)
                })
                .collect();

        for (chunk, mesh) in chunks_to_despawn {
            let Some((entity, _)) = chunks
                .iter()
                .find(|(_, handle)| ***handle == mesh)
            else {
                continue;
            };
            commands.entity(entity).despawn(); // TODO: old chunk meshes aren't actually despawned. 
            chunk_store.0.remove(&chunk);
        }

        for chunk in chunks_to_render {
            commands.queue(BuildChunk(chunk));
        }
    }
}