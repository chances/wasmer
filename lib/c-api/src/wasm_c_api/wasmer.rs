//! Non-standard Wasmer-specific extensions to the Wasm C API.

use super::module::wasm_module_t;
use super::types::wasm_name_t;
use std::ptr;
use std::str;
use std::sync::Arc;

/// Non-standard Wasmer-specific API to get the module's name,
/// otherwise `out->size` is set to `0` and `out->data` to `NULL`.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the engine and the store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create a WebAssembly module from a WAT definition.
///     wasm_byte_vec_t wat;
///     wasmer_byte_vec_new_from_string(&wat, "(module $moduleName)");
///     //                                             ^~~~~~~~~~~ that's the name!
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///
///     // Create the module.
///     wasm_module_t* module = wasm_module_new(store, &wasm);
///
///     // Read the module's name.
///     wasm_name_t name;
///     wasm_module_name(module, &name);
///
///     // It works!
///     wasmer_assert_name(&name, "moduleName");
///
///     // Free everything.
///     wasm_byte_vec_delete(&name);
///     wasm_module_delete(module);
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
#[no_mangle]
pub unsafe extern "C" fn wasm_module_name(
    module: &wasm_module_t,
    // own
    out: &mut wasm_name_t,
) {
    let name = match module.inner.name() {
        Some(name) => name,
        None => {
            out.data = ptr::null_mut();
            out.size = 0;

            return;
        }
    };

    *out = name.as_bytes().to_vec().into();
}

/// Non-standard Wasmer-specific API to set the module's name. The
/// function returns `true` if the name has been updated, `false`
/// otherwise.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the engine and the store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create a WebAssembly module from a WAT definition.
///     wasm_byte_vec_t wat;
///     wasmer_byte_vec_new_from_string(&wat, "(module)");
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///
///     // Create the module.
///     wasm_module_t* module = wasm_module_new(store, &wasm);
///
///     // Read the module's name. There is none for the moment.
///     {
///         wasm_name_t name;
///         wasm_module_name(module, &name);
///
///         assert(name.size == 0);
///     }
///
///     // So, let's set a new name.
///     {
///         wasm_name_t name;
///         wasmer_byte_vec_new_from_string(&name, "hello");
///         wasm_module_set_name(module, &name);
///     }
///
///     // And now, let's see the new name.
///     {
///         wasm_name_t name;
///         wasm_module_name(module, &name);
///
///         // It works!
///         wasmer_assert_name(&name, "hello");
///
///         wasm_byte_vec_delete(&name);
///     }
///
///     // Free everything.
///     wasm_module_delete(module);
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
#[no_mangle]
pub unsafe extern "C" fn wasm_module_set_name(
    module: &mut wasm_module_t,
    // own
    name: &wasm_name_t,
) -> bool {
    let name = match name.into_slice() {
        Some(name) => match str::from_utf8(name) {
            Ok(name) => name,
            Err(_) => return false, // not ideal!
        },
        None => return false,
    };

    match Arc::get_mut(&mut module.inner) {
        Some(module) => module.set_name(name),
        None => false,
    }
}
