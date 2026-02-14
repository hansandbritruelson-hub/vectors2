reason from first principles.
do not use web-sys
components: import via `mod Name;` in script, use `<Name />` in template.
props: define `pub struct Props { ... }` in component script, use `...="props.propName"` in template.
css doesn't cascade for peformance, requires each element to have a class
no inline styles either
Custom renderer feature: Image elements (SVGs) support CSS styling for 'stroke', 'color' (fill), and 'stroke-width'.