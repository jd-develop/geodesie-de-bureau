pub trait Identified {
    /// This function should be able to identifiy any object from any network
    /// The returned string MUST include the network name, and either a matricule or an ID used by the network to identify it’s objects
    fn object_id(&self) -> String;
}