use rustc_abi::Align;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Instance, TyCtxt};

use super::BackendTypes;

pub trait StaticCodegenMethods: BackendTypes {
    fn static_addr_of(&self, cv: Self::Value, align: Align, kind: Option<&str>) -> Self::Value;
    fn codegen_static(&mut self, def_id: DefId);

    /// Emit metadata for a #[dynexport] item.
    /// Creates a companion static `__dynexport_meta_<name>` containing type hash and version info.
    fn emit_dynexport_metadata(&mut self, symbol_name: &str, type_hash: u64);
}

pub trait StaticBuilderMethods: BackendTypes {
    fn get_static(&mut self, def_id: DefId) -> Self::Value;
}

/// Compute a type hash for a function signature for ABI verification.
pub fn compute_fn_type_hash<'tcx>(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let fn_sig = tcx.fn_sig(instance.def_id()).skip_binder();

    // Create a simple hash of the function signature
    let mut hasher = DefaultHasher::new();

    // Hash input types
    for input in fn_sig.inputs().skip_binder() {
        // Use the type's Debug representation as a simple hash source
        format!("{:?}", input).hash(&mut hasher);
    }

    // Hash output type
    format!("{:?}", fn_sig.output().skip_binder()).hash(&mut hasher);

    // Hash ABI
    format!("{:?}", fn_sig.abi()).hash(&mut hasher);

    hasher.finish()
}
