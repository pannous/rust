Go through all of the following failing tests and see which one should work with some small compiler modifications. Mark them as ☐ e.g.
✗ test_all_synonyms ☐

/opt/other/rust/ ./run_all_tests.sh
Probe Test Runner
Using: /opt/other/rust/rustc
Mode: all (includes WIP files)

✓ test_add
✗ test_all_synonyms (compile)
✓ test_and_or
✗ test_array_1indexed (compile)
✗ test_array_like_slice (compile)
✗ test_as_cast_comprehensive (compile)
✗ test_as_cast_convert (compile)
✗ test_as_cast (compile)
✗ test_assert (compile)
✗ test_assign_fetch (compile)
✗ test_assign_fetch2 (compile)
✓ test_at_vec
✗ test_auto_return (compile)
✓ test_box
✗ test_broken_fixed (compile)
✓ test_check_reverse
✗ test_class_funcs (compile)
✗ test_class_methods (compile)
✗ test_class_parse (compile)
✗ test_class_parse2 (compile)
✗ test_class_parse3 (compile)
✗ test_class (compile)
✗ test_comma_compatibility (compile)
✓ test_comments
✗ test_compound (compile)
✗ test_comprehensive_units (compile)
✓ test_const_pow
✗ test_continue (compile)
✓ test_debug_imports
✗ test_debug (compile)
✗ test_debug2 (compile)
✗ test_debug3 (compile)
✗ test_debug4 (compile)
✗ test_def_simple (compile)
✗ test_def (compile)
✗ test_dynexport_lib (compile)
✗ test_dynexport_linked (compile)
✗ test_dynexport_nostd (compile)
✗ test_dynexport_user (compile)
✗ test_dynexport (compile)
✗ test_dynload_complete (compile)
✗ test_dynload_forked (compile)
✗ test_dynload_prelude (compile)
✗ test_ellipsis (compile)
✗ test_enum_string (compile)
✗ test_enum (compile)
✗ test_exclamation_syntax (compile)
✗ test_explicit_main (compile)
✗ test_fetch_debug (compile)
✗ test_fetch_debug2 (compile)
✗ test_fetch_parse (compile)
✗ test_fetch_simple (compile)
✗ test_fib (compile)
✗ test_filter_simple (compile)
✗ test_filter_synonyms (compile)
✗ test_float_add (compile)
✗ test_fmt (compile)
✗ test_fn (compile)
✗ test_for_in_key_value (compile)
✓ test_for_loop
✗ test_global_debug (compile)
✗ test_global (compile)
✗ test_hash_index (compile)
✗ test_hash_minimal (compile)
✗ test_hash_with_if (compile)
✗ test_if_parse (compile)
✗ test_implicit_main (compile)
✗ test_import_bare_syntax (compile)
✗ test_import_folder (compile)
✗ test_in_operator_auto_import (compile)
✗ test_in_operator_maps (compile)
✗ test_in_operator_rune_strings (compile)
✗ test_in_operator_slices (compile)
✗ test_in_operator_strings (compile)
✗ test_interpolation (compile)
✗ test_is_operator (compile)
✗ test_iterator_for_in (compile)
✗ test_iterator_membership (compile)
✗ test_iterator_simple (compile)
✗ test_lambda_arg (compile)
✗ test_lambda (compile)
✗ test_list_comparison (compile)
✗ test_list_comparison2 (compile)
✗ test_list_equality (compile)
✗ test_list_filter (compile)
✗ test_list_lambda (compile)
✗ test_list_map (compile)
✗ test_list_methods_broken (compile)
✗ test_list_methods (compile)
✗ test_list_synonyms_only (compile)
✗ test_list_typed (compile)
✗ test_list (compile)
✓ test_main
✗ test_manual_slices (compile)
✗ test_manual_strings (compile)
✗ test_map_dot_comprehensive (compile)
✗ test_map_dot_nested (compile)
✗ test_map_dot_notation (compile)
✗ test_map_fields (compile)
✗ test_map_type_inference (compile)
✗ test_map (compile)
✓ test_minimal_conflict
✗ test_mixed (compile)
✗ test_modify (compile)
✗ test_nil (compile)
✗ test_non_modifying (compile)
✓ test_normal_rust
✗ test_not_truthiness (compile)
✓ test_not
✓ test_null_coalesce
✗ test_option_abi (exit 139)
✗ test_option_debug (exit 139)
✗ test_option_hashmap (compile)
✓ test_optional_chain
✓ test_optional_syntax
✗ test_parse_if (compile)
✗ test_parse_url (compile)
✗ test_parse_url2 (compile)
✗ test_parse (compile)
✗ test_parse2 (compile)
✗ test_parse3 (compile)
✗ test_parse4 (compile)
✗ test_pipe (compile)
✗ test_pow (compile)
✓ test_pow3
✗ test_power_basic (compile)
✓ test_power
✓ test_precedence
✗ test_print_comparison (compile)
✗ test_printf (compile)
✓ test_put_eq
✗ test_put_no_import (compile)
✓ test_put
✗ test_range_exclusive (compile)
✗ test_range_inclusive (compile)
✗ test_return_void (compile)
✓ test_script_complex
✗ test_shebang (compile)
✗ test_simple_for_range (compile)
✗ test_simple_hash_check (compile)
✗ test_simple_printf (compile)
✓ test_simple
✗ test_slice_inference_core (compile)
✗ test_slice_inference_final (compile)
✓ test_string_auto
✗ test_string_char_comparison (compile)
✗ test_string_char_literal (compile)
✗ test_string_comparison (compile)
✓ test_string_concat
✓ test_string_format_chain
✓ test_string_format_simple
✗ test_string_interpolation (compile)
✓ test_string_macro
✗ test_string_methods_todo (compile)
✗ test_string_methods (compile)
✗ test_string_ops_struct (compile)
✓ test_string_ops
✓ test_string_replace
✓ test_string_reverse
✓ test_string_special
✗ test_string_var_spacing (compile)
✗ test_strings_auto_import (compile)
✗ test_struct (compile)
✗ test_synonyms_simple (compile)
✗ test_tau_pi_approx (compile)
✗ test_transform_synonyms (compile)
✗ test_truthy_and_working (compile)
✗ test_truthy_and (compile)
✗ test_truthy (compile)
✗ test_try_assign_context_aware (compile)
✗ test_try_assign (compile)
✗ test_try_assignment (compile)
✗ test_try_catch (compile)
✗ test_try_propagation (compile)
✗ test_typeof (compile)
✓ test_unicode_ops
✗ test_unicode (compile)
✗ test_units (compile)
✓ test_unused_mut
✗ test_user_defined_put (compile)
✗ test_while_loops (compile)
✗ test_wit_discover (compile)
✗ test_wit_lib (compile)
✓ test_with_main
✗ test_xor (compile)

========================================
Results: 35 passed, 150 failed, 0 skipped
========================================