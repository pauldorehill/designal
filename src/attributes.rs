use crate::builder::Naming;
use proc_macro2::Span;
use quote::format_ident;
use syn::{Attribute, Error, Ident, Meta, MetaNameValue, NestedMeta, Result};

// TODO: Add attribute filter
// TODO: Add attribute adder
// TODO: Add conditonal compliation to allow removing code etc
// TODO: Add generics, where, lifetimes filter

enum AttributeType {
    Ignore(Span),
    Remove(Span),
    Rename(String, Span),
    AddPrefix(String, Span),
    AddPostfix(String, Span),
    RemoveStart(String, Span),
    RemoveEnd(String, Span),
    KeepRc(Span),
    KeepArc(Span),
    Derive(String, Span),
}

impl AttributeType {
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
                    "rename" => make(&name, &span, &Self::Rename),
                    "add_prefix" => make(&name, &span, &Self::AddPrefix),
                    "add_postfix" => make(&name, &span, &Self::AddPostfix),
                    "remove_start" => make(&name, &span, &Self::RemoveStart),
                    "remove_end" => make(&name, &span, &Self::RemoveEnd),
                    "derive" => make(&name, &span, &Self::Derive),
                    _ => Self::err_invalid_ident(&i),
                }
            }
            None => Self::err_invalid_option(nv.path.segments[0].ident.span()), // TODO: test hitting this
        }
    }

    fn new(meta: &NestedMeta) -> Result<Self> {
        let err_formating = |ident: &Ident| {
            Err(Error::new(
                ident.span(),
                format!("{} must be formated like `{} = \"Value\"", ident, ident),
            ))
        };

        match meta {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => match path.get_ident() {
                    Some(i) => match i.to_string().as_str() {
                        "ignore" => Ok(Self::Ignore(i.span())),
                        "remove" => Ok(Self::Remove(i.span())),
                        "keep_rc" => Ok(Self::KeepRc(i.span())),
                        "keep_arc" => Ok(Self::KeepArc(i.span())),
                        "rename" => err_formating(i),
                        "prefix" => err_formating(i),
                        "postfix" => err_formating(i),
                        "derive" => Err(Error::new(
                            i.span(),
                            format!("{} must be formated like `derive = Debug`", i),
                        )),
                        _ => Self::err_invalid_ident(i),
                    },
                    None => Self::err_invalid_option(path.segments[0].ident.span()), // TODO: test hitting this
                },
                Meta::NameValue(nv) => Self::make_from_meta_name(&nv),
                Meta::List(l) => Err(Error::new(l.paren_token.span, "Unable to parse attributes")), // TODO: test hitting this
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
                )) // TODO: test hitting this
            }
        }
    }
}

#[derive(Clone)]
pub(crate) enum Renamer {
    Rename(String, Span),
    AddPrefix(String, Span),
    AddPostfix(String, Span),
    RemoveStart(String, Span),
    RemoveEnd(String, Span),
}

impl Renamer {
    pub fn span(&self) -> &Span {
        match self {
            Renamer::Rename(_, s)
            | Renamer::AddPrefix(_, s)
            | Renamer::AddPostfix(_, s)
            | Renamer::RemoveStart(_, s)
            | Renamer::RemoveEnd(_, s) => s,
        }
    }
    pub fn make_new_name(&self, current: &Ident) -> Result<Ident> {
        let err_naming = |span: &Span, name: &str, remove: &str, msg: &str| {
            Err(Error::new(
                *span,
                format!("struct {} does not {} with {}", name, msg, remove),
            ))
        };
        match self {
            Renamer::Rename(new_str, _) => Ok(format_ident!("{}", new_str)),
            Renamer::AddPrefix(pre, _) => Ok(format_ident!("{}{}", pre, current)),
            Renamer::AddPostfix(post, _) => Ok(format_ident!("{}{}", current, post)),
            Renamer::RemoveStart(remove, s) => {
                let name = current.to_string();
                if name.starts_with(remove) {
                    Ok(format_ident!("{}", name[remove.len()..]))
                } else {
                    err_naming(s, &name, remove, "start")
                }
            }
            Renamer::RemoveEnd(remove, s) => {
                let name = current.to_string();
                if name.ends_with(remove) {
                    Ok(format_ident!("{}", name[..remove.len() + 1]))
                } else {
                    err_naming(s, &name, remove, "end")
                }
            }
        }
    }
}

pub(crate) enum AttributeLocation {
    Struct,
    Field(Naming),
}

pub(crate) struct AttributeOptions<'a> {
    pub(crate) remove: Option<Span>,
    pub(crate) ignore: Option<Span>,
    pub(crate) renamer: Option<Renamer>,
    pub(crate) keep_rc: Option<Span>,
    pub(crate) keep_arc: Option<Span>,
    pub(crate) others_to_keep: Vec<&'a Attribute>,
    pub(crate) derives: Option<Vec<Ident>>,
}

impl<'a> AttributeOptions<'a> {
    /// Only want to merge in keep_rc and keep_arc
    /// only update struct level when its Some
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
        self
    }

    fn is_designal_att(att: &Attribute) -> bool {
        att.path.is_ident("designal")
    }

    fn get_designal_meta(att: &Attribute) -> Vec<Result<AttributeType>> {
        match att.parse_meta() {
            Ok(meta) => {
                match meta {
                    syn::Meta::List(meta_list) => {
                        meta_list.nested.iter().map(AttributeType::new).collect()
                    }
                    // TODO: Test these paths
                    Meta::Path(p) => vec![Err(Error::new(
                        p.segments[0].ident.span(),
                        "Unsupported attribute type",
                    ))],
                    Meta::NameValue(nv) => {
                        vec![Err(Error::new(nv.lit.span(), "Unsupported attribute type"))]
                    }
                }
            }
            Err(e) => vec![Err(e)],
        }
    }

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
            AttributeLocation::Struct => {
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
                } else if let Some(s) = &self.derives {
                    // Will only be some if there is somethihng
                    return Err(Error::new(s[0].span(), "You can't derive on a field"));
                } else {
                    Ok(())
                }
            }
        }
    }

    pub(crate) fn new(atts: &'a Vec<Attribute>, att_location: AttributeLocation) -> Result<Self> {
        let (designal_atts, others_to_keep) = Self::get_designal_attributes(atts)?;
        let mut ignore: Option<Span> = None;
        let mut remove: Option<Span> = None;
        let mut rename: Option<Renamer> = None;
        let mut add_postfix: Option<Renamer> = None;
        let mut add_prefix: Option<Renamer> = None;
        let mut remove_start: Option<Renamer> = None;
        let mut remove_end: Option<Renamer> = None;
        let mut keep_rc: Option<Span> = None;
        let mut keep_arc: Option<Span> = None;
        let mut derives: Option<Vec<Ident>> = None;

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
                AttributeType::AddPrefix(name, span) => {
                    set_renamer(&mut add_prefix, "prefix", Renamer::AddPrefix(name, span))?
                }
                AttributeType::AddPostfix(name, span) => {
                    set_renamer(&mut add_postfix, "postfix", Renamer::AddPostfix(name, span))?
                }
                AttributeType::RemoveStart(name, span) => set_renamer(
                    &mut remove_start,
                    "remove_start",
                    Renamer::RemoveStart(name, span),
                )?,
                AttributeType::RemoveEnd(name, span) => set_renamer(
                    &mut remove_end,
                    "remove_end",
                    Renamer::RemoveEnd(name, span),
                )?,
                AttributeType::KeepRc(span) => set_span(&mut keep_rc, "keep_rc", &span)?,
                AttributeType::KeepArc(span) => set_span(&mut keep_arc, "keep_arc", &span)?,
                AttributeType::Derive(name, span) => match derives {
                    Some(ref mut traits) => {
                        // TODO: use syn::parse_str to give better error message
                        for name in name.split(",") {
                            traits.push(Ident::new(&name.trim(), span));
                        }
                    }
                    None => {
                        derives = Some(
                            name.split(",")
                                .map(|name| Ident::new(name.trim(), span))
                                .collect(),
                        );
                    }
                },
            }
        }
        let all = [rename, add_prefix, add_postfix, remove_start, remove_end];
        let renamer: Vec<&Renamer> = all.iter().filter_map(|v| v.as_ref()).collect();

        let renamer = if renamer.len() == 1 {
            Some(renamer[0].to_owned())
        } else {
            match renamer.last() {
                Some(&v) => return Err(Error::new(
                    *v.span(),
                    "Can only do one of rename, add_prefix, add_postfix, remove_start, remove_end",
                )),
                None => None,
            }
        };

        let atts = Self {
            ignore,
            remove,
            renamer,
            keep_rc,
            keep_arc,
            others_to_keep,
            derives,
        };
        atts.validate(att_location)?;
        Ok(atts)
    }
}
