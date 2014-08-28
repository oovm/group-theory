pub type Result<T> = std::result::Result<T, GroupError>;

#[derive(Copy, Clone, Debug)]
pub struct GroupError {
    kind: GroupErrorKind,
}

#[derive(Copy, Clone, Debug)]
pub enum GroupErrorKind {
    InvalidPermutation,
}

impl GroupError {
    pub fn invalid_permutation() -> Self {
        Self {
            kind: GroupErrorKind::InvalidPermutation,
        }
    }
}

impl GroupError {
    pub fn kind(&self) -> &GroupErrorKind {
        &self.kind
    }
}