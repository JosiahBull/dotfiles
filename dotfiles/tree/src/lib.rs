use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use dependencies::{DependencyGraph, DependencyInstallable};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

// TODO: write a proc macro to generate the dependency graph
// TOOD: rename to cache
