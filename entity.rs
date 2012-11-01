use amulet::ll;

struct Entity {
    proto: @Prototype,
    mut position: (uint, uint),
}
impl Entity {
    // PHYSICS
    fn is_passable() -> bool {
        return self.proto.passable;
    }
}


struct Prototype {
    display: char,
    style: ll::Style,
    passable: bool,
}
