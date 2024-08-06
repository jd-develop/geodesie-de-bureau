pub trait Identified {
    /// This function should be able to identifiy any object from any network
    /// The returned string MUST include :
    /// - The network name
    /// - Either a matricule or an ID used by the network to identify it’s objects
    /// (to ensure unique identification across all networks)
    fn object_id(&self) -> String;
}