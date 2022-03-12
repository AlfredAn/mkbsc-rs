macro_rules! derive_visitable {
    ($name:ty, $($args:tt)*) => {
        impl<$($args)*> petgraph::visit::Visitable for $name {
            type Map = std::collections::hash_set::HashSet<<$name as petgraph::visit::GraphBase>::NodeId>;

            fn visit_map(&self) -> Self::Map {
                Self::Map::new()
            }

            fn reset_map(&self, map: &mut Self::Map) {
                map.clear();
            }
        }
    }
}
