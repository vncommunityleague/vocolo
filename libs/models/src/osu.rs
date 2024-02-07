pub mod mappool {
    pub use dto::*;
    pub use handler::*;
    pub use model::*;

    mod dto;

    mod handler;

    mod model;
}

pub mod team {
    pub use dto::*;
    pub use handler::*;
    pub use model::*;

    mod dto;

    mod handler;

    mod model;
}

pub mod tournament {
    pub use dto::*;
    pub use handler::*;
    pub use model::*;

    mod dto;

    mod handler;

    mod model;
}
