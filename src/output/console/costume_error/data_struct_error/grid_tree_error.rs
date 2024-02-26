#[derive(Debug, Clone, Copy)]
pub enum GridTreeError {
    ParentNotFound,
    NodeAlreadyExists,
    NodeNotConnectedToTree,
    NodeNotFound
}