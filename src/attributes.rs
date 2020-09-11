use crate::builder::Naming;
use proc_macro2::Span;
use quote::format_ident;
use syn::{Attribute, Error, Ident, LitStr, Meta, MetaNameValue, NestedMeta, Result};

// TODO: Add attribute filter
// TODO: Add attribute adder
// TODO: Add generics, where, lifetimes filter

enum AttributeType {
    Ignore(Span),
    Remove(Span),
    Rename(String, Span),
    AddStart(String, Span),
    AddEnd(String, Span),
    TrimStart(String, Span),
    TrimStartAll(String, Span),
    TrimEnd(String, Span),
    TrimEndAll(String, Span),
    KeepRc(Span),
    KeepArc(Span),
    HashMap(Span),
    Derive(String, Span),
    CfgFeature(String, Span), //TODO: Is this worth it? Since will need the feature on the base struct to build the generated one? Easier to just not import?
}

impl AttributeType {
    const IGNORE: &'static str = "ignore";
    const REMOVE: &'static str = "remove";
    const RENAME: &'static str = "rename";
    const ADD_START: &'static str = "add_start";
    const ADD_END: &'static str = "add_end";
    const TRIM_START: &'static str = "trim_start";
    const TRIM_START_ALL: &'static str = "trim_start_all";
    const TRIM_END: &'static str = "trim_end";
    const TRIM_END_ALL: &'static str = "trim_end_all";
    const KEEP_RC: &'static str = "keep_rc";
    const KEEP_ARC: &'static str = "keep_arc";
    const HASHMAP: &'static str = "hashmap";
    const DERIVE: &'static str = "derive";
    const CFG_FEATURE: &'static str = "cfg_feature";

    fn err_only_str(span: Span) -> Result<Self> {
        Err(Error::new(span, "Only string literals are allowed"))
    }

    fn err_invalid_option(span: Span) -> Result<Self> {
        Err(Error::new(span,"Attribute option was not a single identifier: the path had more than a single segement."))
    }

    fn err_invalid_ident(i: &Ident) -> Result<Self> {
        Err(Error::new(
            i.span(),
            format!("Unrecognized attribute identifier option: {}", i),
        ))
    }

    fn make_from_meta_name(nv: &MetaNameValue) -> Result<Self> {
        let make =
            |name: &str, span: &Span, return_type: &dyn Fn(String, Span) -> Self| match &nv.lit {
                syn::Lit::Str(s) => {
                    let v = s.value();
                    if v.is_empty() {
                        Err(Error::new(s.span(), format!("{} can't be empty", name)))
                    } else {
                        Ok(return_type(v, *span))
                    }
                }
                syn::Lit::ByteStr(v) => Self::err_only_str(v.span()),
                syn::Lit::Byte(v) => Self::err_only_str(v.span()),
                syn::Lit::Char(v) => Self::err_only_str(v.span()),
                syn::Lit::Int(v) => Self::err_only_str(v.span()),
                syn::Lit::Float(v) => Self::err_only_str(v.span()),
                syn::Lit::Bool(v) => Self::err_only_str(v.span),
                syn::Lit::Verbatim(v) => Self::err_only_str(v.span()),
            };

        match nv.path.get_ident() {
            Some(i) => {
                let name = i.to_string();
                let span = i.span();
                match name.as_str() {
                    Self::RENAME => make(&name, &span, &Self::Rename),
                    Self::ADD_START => make(&name, &span, &Self::AddStart),
                    Self::ADD_END => make(&name, &span, &Self::AddEnd),
                    Self::TRIM_START => make(&name, &span, &Self::TrimStart),
                    Self::TRIM_START_ALL => make(&name, &span, &Self::TrimStartAll),
                    Self::TRIM_END => make(&name, &span, &Self::TrimEnd),
                    Self::TRIM_END_ALL => make(&name, &span, &Self::TrimEndAll),
                    Self::DERIVE => make(&name, &span, &Self::Derive),
                    Self::CFG_FEATURE => make(&name, &span, &Self::CfgFeature),
                    _ => Self::err_invalid_ident(&i),
                }
            }
            None => Self::err_invalid_option(nv.path.segments[0].ident.span()),
        }
    }

    fn new(meta: &NestedMeta) -> Result<Self> {
        match meta {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => match path.get_ident() {
                    Some(i) => match i.to_string().as_str() {
                        Self::IGNORE => Ok(Self::Ignore(i.span())),
                        Self::REMOVE => Ok(Self::Remove(i.span())),
                        Self::KEEP_RC => Ok(Self::KeepRc(i.span())),
                        Self::KEEP_ARC => Ok(Self::KeepArc(i.span())),
                        Self::HASHMAP => Ok(Self::HashMap(i.span())),
                        // These check if it is a valid attribute, but not formated in the right way
                        s if s == Self::RENAME || s == Self::ADD_START || s == Self::ADD_END || s == Self::TRIM_START || s == Self::TRIM_END => {
                            Err(Error::new(i.span(), format!("You need to provide a way to rename the struct like `{} = \"NoSignals\"", s)))
                        }
                        s if s ==  Self::DERIVE || s == Self::CFG_FEATURE => Err(Error::new(
                            i.span(),
                            format!("{} must be formated like `{} = \"Your value\"`", i, s),
                        )),
                        _ => Self::err_invalid_ident(i),
                    },
                    None => Self::err_invalid_option(path.segments[0].ident.span()),
                },
                Meta::NameValue(nv) => Self::make_from_meta_name(&nv),
                Meta::List(l) => Err(Error::new(l.paren_token.span, "Unable to parse attributes")),
            },
            NestedMeta::Lit(l) => {
                let s = match l {
                    syn::Lit::Str(v) => v.value(),
                    syn::Lit::ByteStr(v) => String::from_utf8(v.value()).unwrap(),
                    syn::Lit::Byte(v) => v.value().to_string(),
                    syn::Lit::Char(v) => v.value().to_string(),
                    syn::Lit::Int(v) => v.to_string(),
                    syn::Lit::Float(v) => v.to_string(),
                    syn::Lit::Bool(v) => v.value.to_string(),
                    syn::Lit::Verbatim(v) => v.to_string(),
                };
                Err(Error::new(
                    l.span(),
                    format!("Literals are not allowed. You passed in: {}", s),
                ))
            }
        }
    }
}

#[derive(Clone)]
pub(crate) enum Renamer {
    Rename(String, Span),
    AddStart(String, Span),
    AddEnd(String, Span),
    TrimStart(String, Span),
    TrimStartAll(String, Span),
    TrimEnd(String, Span),
    TrimEndAll(String, Span),
}

impl Renamer {
    pub fn span(&self) -> &Span {
        match self {
            Renamer::Rename(_, s)
            | Renamer::AddStart(_, s)
            | Renamer::AddEnd(_, s)
            | Renamer::TrimStart(_, s)
            | Renamer::TrimEnd(_, s) => s,
            Renamer::TrimStartAll(_, s) => s,
            Renamer::TrimEndAll(_, s) => s,
        }
    }

    pub fn make_new_name(&self, current: &Ident, att_location: AttributeLocation) -> Result<Ident> {
        let err_naming = |span: &Span, name: &str, remove: &str, msg: &str| {
            let location = match att_location {
                AttributeLocation::Struct(_) => "struct",
                AttributeLocation::Field(_) => "field",
            };
            Err(Error::new(
                *span,
                format!("{} {} does not {} with {}", location, name, msg, remove),
            ))
        };

        match self {
            Self::Rename(new_str, _) => Ok(format_ident!("{}", new_str)),
            Self::AddStart(pre, _) => Ok(format_ident!("{}{}", pre, current)),
            Self::AddEnd(post, _) => Ok(format_ident!("{}{}", current, post)),
            Self::TrimStart(remove, s) => {
                let name = current.to_string();
                if name.starts_with(remove) {
                    Ok(format_ident!("{}", name.trim_start_matches(remove)))
                } else {
                    err_naming(s, &name, remove, "start")
                }
            }
            Self::TrimStartAll(remove, s) => {
                let name = current.to_string();
                let id = || Ok(format_ident!("{}", name.trim_start_matches(remove)));
                match att_location {
                    AttributeLocation::Struct(_) => {
                        if name.starts_with(remove) {
                            id()
                        } else {
                            err_naming(s, &name, remove, "start")
                        }
                    }
                    AttributeLocation::Field(_) => {
                        if name.starts_with(remove) {
                            id()
                        } else {
                            Ok(current.clone()) //Can this be changed to a ref?
                        }
                    }
                }
            }
            Self::TrimEnd(remove, s) => {
                let name = current.to_string();
                if name.ends_with(remove) {
                    Ok(format_ident!("{}", name.trim_end_matches(remove)))
                } else {
                    err_naming(s, &name, remove, "end")
                }
            }
            Self::TrimEndAll(remove, s) => {
                let name = current.to_string();
                let id = || Ok(format_ident!("{}", name.trim_end_matches(remove)));
                match att_location {
                    AttributeLocation::Struct(_) => {
                        if name.ends_with(remove) {
                            id()
                        } else {
                            err_naming(s, &name, remove, "end")
                        }
                    }
                    AttributeLocation::Field(_) => {
                        if name.ends_with(remove) {
                            id()
                        } else {
                            Ok(current.clone()) //Can this be changed to a ref?
                        }
                    }
                }
            }
        }
    }
}

pub(crate) enum AttributeLocation {
    Struct(Span),
    Field(Naming),
}

pub(crate) struct AttributeOptions<'a> {
    pub(crate) remove: Option<Span>,
    pub(crate) ignore: Option<Span>,
    pub(crate) renamer: Option<Renamer>, //TODO: Should this rather be an enum since now mandatory for a struct?
    pub(crate) keep_rc: Option<Span>,
    pub(crate) keep_arc: Option<Span>,
    pub(crate) hashmap: Option<Span>,
    pub(crate) others_to_keep: Vec<&'a Attribute>,
    pub(crate) derives: Option<Vec<Ident>>,
    pub(crate) cfg_feature: Option<Vec<LitStr>>, //TODO: Is this best?
}

impl<'a> AttributeOptions<'a> {
    /// Only want to merge in keep_rc, keep_arc, hashmap, trim_start_all, trim_end_all
    /// only update when the struct level is_some()
    // TODO: Do nothing if already some?
    pub(crate) fn add_struct_level_to_field_level(
        mut self,
        struct_level: &AttributeOptions,
    ) -> Self {
        if let Some(_) = struct_level.keep_rc {
            self.keep_rc = struct_level.keep_rc;
        }
        if let Some(_) = struct_level.keep_arc {
            self.keep_arc = struct_level.keep_arc;
        }
        if let Some(_) = struct_level.hashmap {
            self.hashmap = struct_level.hashmap;
        }
        // Struct is only applied if the field has no renamer
        if let (None, Some(renamer)) = (&self.renamer, &struct_level.renamer) {
            match renamer {
                Renamer::TrimStartAll(_, _) | Renamer::TrimEndAll(_, _) => {
                    self.renamer = struct_level.renamer.clone();
                }
                _ => (),
            }
        }

        self
    }

    fn is_designal_att(att: &Attribute) -> bool {
        att.path.is_ident("designal")
    }

    fn get_designal_meta(att: &Attribute) -> Vec<Result<AttributeType>> {
        match att.parse_meta() {
            Ok(meta) => match meta {
                syn::Meta::List(meta_list) => {
                    meta_list.nested.iter().map(AttributeType::new).collect()
                }
                Meta::Path(p) => vec![Err(Error::new(
                    p.segments[0].ident.span(),
                    "Unsupported attribute type",
                ))],
                Meta::NameValue(nv) => {
                    vec![Err(Error::new(nv.lit.span(), "Unsupported attribute type"))]
                }
            },
            Err(e) => vec![Err(e)],
        }
    }

    // TODO: This will remove all cfg, should it only do feature?
    // fn remove_cfg_feature(att: &&Attribute) -> bool {
    //     match att.path.get_ident() {
    //         Some(i) => i.to_string() != "cfg",
    //         None => false,
    //     }
    // }
    // let others: Vec<&Attribute> = others
    //     .into_iter()
    //     .filter(Self::remove_cfg_feature)
    //     .collect();

    // TODO: Avoid iterating twice?
    fn get_designal_attributes(
        atts: &Vec<Attribute>,
    ) -> Result<(Vec<AttributeType>, Vec<&Attribute>)> {
        let (designal, others): (Vec<&Attribute>, Vec<&Attribute>) =
            atts.iter().partition(|att| Self::is_designal_att(att));
        let designal: Result<Vec<AttributeType>> = designal
            .into_iter()
            .map(Self::get_designal_meta)
            .flatten()
            .collect();
        Ok((designal?, others))
    }

    // TODO: Check struct derived against field? eg. if keep_rc etc.
    fn validate(&self, att_location: AttributeLocation) -> Result<()> {
        match att_location {
            AttributeLocation::Struct(struct_span) => {
                if let Some(span) = self.remove {
                    return Err(Error::new(
                        span,
                        "Remove is not valid at the container level",
                    ));
                } else if let Some(span) = self.ignore {
                    return Err(Error::new(
                        span,
                        "Ignore is not valid at the container level",
                    ));
                } else if self.renamer.is_none() {
                    //TODO: Add example to error?
                    Err(Error::new(struct_span, "To use designal a struct must be renamed using rename, add_start, add_end, trim_start, trim_end"))
                } else {
                    Ok(())
                }
            }
            AttributeLocation::Field(naming) => {
                let all_but_ignore = self.remove.is_some()
                    || self.renamer.is_some()
                    || self.keep_rc.is_some()
                    || self.keep_arc.is_some();

                if let (Some(remove), Some(_)) = (&self.remove, &self.renamer) {
                    return Err(Error::new(*remove, "You have removed and renamed a field"));
                } else if self.ignore.is_some() && all_but_ignore {
                    return Err(Error::new(
                        self.ignore.unwrap(),
                        "You are ignoring designal on this field, but have added other attributes",
                    ));
                } else if let (Some(renamer), true) = (&self.renamer, naming.is_unnamed()) {
                    return Err(Error::new(
                        *renamer.span(),
                        "You cannot rename a unnamed field",
                    ));
                } else if let Some(renamer) = &self.renamer {
                    let e = |name: &str| {
                        return Err(Error::new(
                            *renamer.span(),
                            format!("`trim_{}_all` is only valid at the container level", name),
                        ));
                    };
                    match renamer {
                        Renamer::TrimStartAll(_, _) => e("start"),
                        Renamer::TrimEndAll(_, _) => e("end"),
                        _ => Ok(()),
                    }
                } else if let Some(s) = &self.derives {
                    // Will only be some if there is somethihng
                    return Err(Error::new(s[0].span(), "You can't derive on a field"));
                } else if let Some(s) = &self.cfg_feature {
                    return Err(Error::new(
                        s[0].span(),
                        "You can't use cfg_feature on a field",
                    ));
                } else {
                    Ok(())
                }
            }
        }
    }

    fn make_vec<T, F>(items: &mut Option<Vec<T>>, value: &str, span: Span, maker: F)
    where
        F: Fn(&str, Span) -> T,
    {
        match items {
            Some(ref mut items) => {
                // TODO: use syn::parse_str to give better error message
                for name in value.split(",") {
                    items.push(maker(&name.trim(), span));
                }
            }
            None => {
                *items = Some(
                    value
                        .split(",")
                        .map(|name| maker(name.trim(), span))
                        .collect(),
                );
            }
        }
    }

    pub(crate) fn new(atts: &'a Vec<Attribute>, att_location: AttributeLocation) -> Result<Self> {
        let (designal_atts, others_to_keep) = Self::get_designal_attributes(atts)?;
        let mut ignore: Option<Span> = None;
        let mut remove: Option<Span> = None;
        let mut rename: Option<Renamer> = None;
        let mut add_end: Option<Renamer> = None;
        let mut add_start: Option<Renamer> = None;
        let mut trim_start: Option<Renamer> = None;
        let mut trim_end: Option<Renamer> = None;
        let mut keep_rc: Option<Span> = None;
        let mut keep_arc: Option<Span> = None;
        let mut hashmap: Option<Span> = None;
        let mut derives: Option<Vec<Ident>> = None;
        let mut cfg_feature: Option<Vec<LitStr>> = None;

        let set_span = |existing: &mut Option<Span>, name: &str, new_value: &Span| match existing {
            Some(_) => Err(Error::new(
                *new_value,
                format!("You should only {} once", name),
            )),
            None => {
                *existing = Some(*new_value);
                Ok(())
            }
        };

        let set_renamer =
            |existing: &mut Option<Renamer>, name: &str, new_value: Renamer| match existing {
                Some(_) => Err(Error::new(
                    *new_value.span(),
                    format!("You should only {} once", name),
                )),
                None => {
                    *existing = Some(new_value);
                    Ok(())
                }
            };

        for att in designal_atts {
            match att {
                AttributeType::Ignore(span) => set_span(&mut ignore, "ignore", &span)?,
                AttributeType::Remove(span) => set_span(&mut remove, "remove", &span)?,
                AttributeType::Rename(name, span) => {
                    set_renamer(&mut rename, "rename", Renamer::Rename(name, span))?
                }
                AttributeType::AddStart(name, span) => {
                    set_renamer(&mut add_start, "prefix", Renamer::AddStart(name, span))?
                }
                AttributeType::AddEnd(name, span) => {
                    set_renamer(&mut add_end, "postfix", Renamer::AddEnd(name, span))?
                }
                AttributeType::TrimStart(name, span) => set_renamer(
                    &mut trim_start,
                    "trim_start",
                    Renamer::TrimStart(name, span),
                )?,
                AttributeType::TrimStartAll(name, span) => set_renamer(
                    &mut trim_start,
                    "trim_start",
                    Renamer::TrimStartAll(name, span),
                )?,
                AttributeType::TrimEnd(name, span) => {
                    set_renamer(&mut trim_end, "trim_end", Renamer::TrimEnd(name, span))?
                }
                AttributeType::TrimEndAll(name, span) => {
                    set_renamer(&mut trim_end, "trim_end", Renamer::TrimEndAll(name, span))?
                }
                AttributeType::KeepRc(span) => set_span(&mut keep_rc, "keep_rc", &span)?,
                AttributeType::KeepArc(span) => set_span(&mut keep_arc, "keep_arc", &span)?,
                AttributeType::HashMap(span) => set_span(&mut hashmap, "hashmap", &span)?,
                AttributeType::Derive(value, span) => {
                    Self::make_vec(&mut derives, &value, span, Ident::new)
                }
                AttributeType::CfgFeature(value, span) => {
                    Self::make_vec(&mut cfg_feature, &value, span, LitStr::new)
                }
            }
        }

        let renamer = {
            let all = [rename, add_start, add_end, trim_start, trim_end];
            let renamer: Vec<&Renamer> = all.iter().filter_map(|v| v.as_ref()).collect();
            if renamer.len() == 1 {
                Some(renamer[0].to_owned()) //TODO: remove the clone?
            } else {
                match renamer.last() {
                    Some(&v) => return Err(Error::new(
                        *v.span(),
                        "You can only do one of rename, add_start, add_end, trim_start, trim_end",
                    )),
                    None => None,
                }
            }
        };

        let atts = Self {
            ignore,
            remove,
            renamer,
            keep_rc,
            keep_arc,
            hashmap,
            others_to_keep,
            derives,
            cfg_feature,
        };
        atts.validate(att_location)?;
        Ok(atts)
    }
}
