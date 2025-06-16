use dereferencing::demo_of_dereferencing;
use foreign_function_interface::ffi_c;
use inline_assembly::demo_of_assembly;
use mutable_statics::demo_of_mutable_static_variables;
use unsafe_functions::demo_of_unsafe_functions;
use unsafe_traits::demo_of_unsafe_traits;

mod dereferencing;
mod foreign_function_interface;
mod inline_assembly;
mod mutable_statics;
mod unsafe_functions;
mod unsafe_traits;

pub fn demo_of_unsafe() {
    demo_of_dereferencing();
    demo_of_unsafe_functions();
    demo_of_unsafe_traits();
    demo_of_mutable_static_variables();
    demo_of_assembly();
    ffi_c();
}
