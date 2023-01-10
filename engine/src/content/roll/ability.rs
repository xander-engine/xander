use crate::utils::identity::Identity;

///
/// Represents an ability.
/// 
/// **Source** SRD 5.1E (pg. 76)
/// 
pub trait Ability : Identity {
    // TODO!

    fn id() -> &'static str
        where Self : Sized;
}