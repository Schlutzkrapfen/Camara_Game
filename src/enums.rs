use bevy::sprite_render::TileOrientation;

pub fn orientation_to_u8(o: TileOrientation) -> u8 {
    match o {
        TileOrientation::Default => 0,
        TileOrientation::Rotate90 => 1,
        TileOrientation::Rotate180 => 2,
        TileOrientation::Rotate270 => 3,
        TileOrientation::MirrorH => 4,
        TileOrientation::MirrorHRotate90 => 5,
        TileOrientation::MirrorHRotate180 => 6,
        TileOrientation::MirrorHRotate270 => 7,
    }
}

pub fn orientation_from_u8(n: u8) -> TileOrientation {
    match n {
        1 => TileOrientation::Rotate90,
        2 => TileOrientation::Rotate180,
        3 => TileOrientation::Rotate270,
        4 => TileOrientation::MirrorH,
        5 => TileOrientation::MirrorHRotate90,
        6 => TileOrientation::MirrorHRotate180,
        7 => TileOrientation::MirrorHRotate270,
        _ => TileOrientation::Default, // 0 e qualsiasi valore non valido
    }
}
