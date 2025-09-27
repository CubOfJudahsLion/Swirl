//! # Intermediate Representation
//!
//! Data structures that more closely represent the
//! code to generate.
//!
//! The IR is implemented as a two-tier acyclic
//! non-directed graph.  The root and every other
//! node will be `Group`s (i.e., `enum`s or
//! `struct`s) with optional `Entry` parents and *n*
//! `Entry` children. An `Entry` (i.e., an `enum`
//! variant or `struct` field) have mandatory
//! `Group` parents and may contain either an simple
//! value (a file name) or allow for values of
//! another `enum` (i.e., a `Group`.)
//!
//! - *Version* : 0.2.0
//! - *Author*  : Alex Feterman
//! - *Created* : Fri 10/22/2021 02:38 PM
//! - *Modified*: Thu 06 Jan 2022 04:19:29 PM CST


use std::fmt::{self, Debug, Formatter};
use syn::{Ident, LitStr};
use super::common::Shared;
use super::util::pascal_to_snake;
use super::ast::*;


/// A `Group` represents an `enum` or a `struct`.
#[derive(Clone)]
pub struct Group {
    enum_name   : Ident,
    struct_name : Ident,
    parent      : Option<Shared<Entry>>,
    entries     : Vec<Shared<Entry>>,
}

impl Group {
    const INITIAL_ENTRY_CAPACITY: usize = 256;

    pub fn new_shared(parent: Option<&Shared<Entry>>, enum_name: &Ident, cap: Option<usize>) -> Shared<Self> {
        Shared::new(Group {
            enum_name   : enum_name.clone(),
            struct_name : Ident::new(&(enum_name.to_string() + "Struct"), enum_name.span()),
            parent      : parent.map(|p| p.clone()),
            entries     : Vec::with_capacity(cap.unwrap_or(Self::INITIAL_ENTRY_CAPACITY)),
        })
    }

    pub fn set_parent(&mut self, parent: Option<&Shared<Entry>>) {
        self.parent = parent.map(|p| p.clone());
    }

    #[inline(always)]
    pub fn enum_name(&self) -> &Ident { &self.enum_name }

    #[inline(always)]
    pub fn struct_name(&self) -> &Ident { &self.struct_name }

    #[inline(always)]
    pub fn parent(&self) -> Option<&Shared<Entry>> { self.parent.as_ref() }

    #[inline(always)]
    pub fn entries(&self) -> &[Shared<Entry>] { &self.entries }

    #[inline]
    fn add_entry(&mut self, entry: &Shared<Entry>) {
        self.entries.push(entry.clone());
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Group {{ enum_name: {:?}, struct_name: {:?}, parent: {}, entries: {:?} }}",
            self.enum_name.to_string(),
            self.struct_name.to_string(),
            self.parent
                .as_ref()
                .map(|p| "Some(<<Entry ".to_string() + &p.var_name.to_string() + ">>)")
                .unwrap_or("None".to_string()),
            &self.entries
        ))
    }
}


/// A set of files for any of several purposes.
pub enum FileSet {
    FontFile(LitStr),
    ImageFile(LitStr),
    ImageSequence(Vec<LitStr>),
    SoundFiles(LitStr, LitStr),
}

impl From<&VariantFiles> for FileSet {
    fn from(var_files: &VariantFiles) -> Self {
        use VariantFiles::*;
        match var_files {
            FontFile(lit)       => FileSet::FontFile(lit.clone()),
            ImageFile(lit)      => FileSet::ImageFile(lit.clone()),
            SoundFiles(l1, l2)  => FileSet::SoundFiles(l1.clone(), l2.clone()),
        }
    }
}


pub enum EntryValue {
    FileValue(FileSet),
    GroupValue(Shared<Group>),
}

impl EntryValue {
    #[inline(always)]
    fn new_file_value(var_files: &VariantFiles) -> Self {
        EntryValue::FileValue(FileSet::from(var_files))
    }

    #[inline]
    fn new_group_value(group: &Shared<Group>) -> Self {
        EntryValue::GroupValue(group.clone())
    }
}


#[derive(Clone)]
pub struct Entry {
    var_name    : Ident,
    field_name  : Ident,
    parent      : Shared<Group>,
    value       : EntryValue,
}

impl Entry {
    pub fn new_shared(parent: &mut Shared<Group>, var_name: &Ident, value: EntryValue) -> Shared<Self> {
        let result = Shared::new(Entry {
            var_name    : var_name.clone(),
            field_name  : Ident::new(&pascal_to_snake(&var_name.to_string()), var_name.span()),
            parent      : parent.clone(),
            value,
        });
        //parent.add_entry(&result);
        result
    }

    #[inline(always)]
    pub fn var_name(&self) -> &Ident { &self.var_name }

    #[inline(always)]
    pub fn field_name(&self) -> &Ident { &self.field_name }

    #[inline(always)]
    pub fn parent(&self) -> &Shared<Group> { &self.parent }

    #[inline(always)]
    pub fn value(&self) -> &EntryValue { &self.value }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Entry {{ var_name: {:?}, field_name: {:?}, parent: <<Group {}>>, value: {:?} }}",
            self.var_name.to_string(),
            self.field_name.to_string(),
            self.parent.enum_name.to_string(),
            &self.value
        ))
    }
}


pub fn ast_to_ir(ast: Enum) -> Result<Enum, syn::Error> {
    fn simple_variant(parent: &mut Shared<Group>, var: Variant) -> Entry {
        let entry = Entry::new_shared(parent, var.name(), EntryValue::from(var.files()));
        parent.add_entry(&entry);
        entry
    }

    fn ranged_variant(parent: &mut Shared<Group>, rng: RangedEnum) -> Entry {
        let mut enum_grp    =   Group::new_shared(None, rng.enum_name(), Some(rng.max() - rng.min() + 1));
        for n in rng.min() ..= rng.max() {
            let idx_str     =   format!("{:0width$}", n, width = rng.zero_padding());
            let entry_name  =   rng.enum_name().to_string() + &idx_str;
            let file_name   =   rng.file_prefix() + &idx_str + rng.file_postfix();
            //  Note that the entry won't be lost in the following
            let entry       =   Entry::new_shared(
                                    &enum_grp,
                                    &LitStr::new(&entry_name),
                                    EntryValue::FileValue(FileSet::ImageFile(&LitStr::new(&file_name)))
                                );
            enum_grp.add_entry(&entry);
        }
        let entry           =   Entry::new_shared(
    }
}
