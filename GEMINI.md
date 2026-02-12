reason from first principles.
do not use web-sys
components: import via `mod Name;` in script, use `<Name />` in template.
props: define `pub struct Props { ... }` in component script, use `...="props.propName"` in template.