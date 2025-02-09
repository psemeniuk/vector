/// A component with a well-known name.
///
/// Users can derive this trait automatically by using the
/// [`component_name`][vector_config::component_name] macro on their structs/enums.
pub trait NamedComponent {
    const NAME: &'static str;

    /// Gets the name of the component.
    fn get_component_name(&self) -> &'static str {
        Self::NAME
    }
}
