use bevy::prelude::*;
pub struct BlockSprite {
    pub shape: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

#[derive(Resource)]
pub struct GameSprites {
    pub board_block: BlockSprite,
    pub tetroid_block: BlockSprite,
}
