//!
//! Collection of utility macros
//! for creating content with xander.
//! 

use convert_case::Casing;
use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, LitInt, Token, parse_macro_input, parse::Parse, Ident, token::Paren, parenthesized, Path, LitStr, ItemFn, Visibility, ItemStruct, Attribute};

///
/// Auto-generates implementation of the `Die` trait for
/// multiple sides of die.
/// 
/// ### Syntax
/// `dice!(4, 6, 8, 10, 12, 20, 100)`
/// 
#[proc_macro]
pub fn dice(tokens : TokenStream) -> TokenStream {
    struct DiceSides(Punctuated<LitInt, Token![,]>);

    impl DiceSides {
        fn into_iter(self) -> impl Iterator<Item = LitInt> {
            self.0.into_iter()
        }
    }

    impl Parse for DiceSides {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }

    let sides = parse_macro_input!(tokens as DiceSides);

    let dice_impls = sides.into_iter()
        .map(|sides| {
            let ident = Ident::new(&format!("D{}", sides.base10_digits()), sides.span());
            let doc_string = format!("A {}-sided die.", sides.base10_digits());
            quote! {
                #[doc = #doc_string]
                #[doc = "***"]
                #[doc = "🤖 Autogenerated by [xander_macros::dice]"]
                #[doc = ""]
                #[derive(Debug, Clone, Copy)]
                pub struct #ident;

                impl Die for #ident {
                    fn sides(&self) -> usize {
                        #sides
                    }
                }

                impl FnOnce<()> for #ident {
                    type Output = Rolls;

                    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
                        self.roll(1)
                    }
                }

                
                impl FnMut<()> for #ident {
                    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
                        self.roll(1)
                    }
                }

                impl Fn<()> for #ident {
                    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
                        self.roll(1)
                    }
                }

                impl FnOnce<(usize,)> for #ident {
                    type Output = Rolls;
                
                    extern "rust-call" fn call_once(self, args: (usize,)) -> Self::Output {
                        self.roll(args.0)
                    }
                } 
                
                impl FnMut<(usize,)> for #ident {
                    extern "rust-call" fn call_mut(&mut self, args: (usize,)) -> Self::Output {
                        self.roll(args.0)
                    }
                }
                
                impl Fn<(usize,)> for #ident {
                    extern "rust-call" fn call(&self, args: (usize,)) -> Self::Output {
                        self.roll(args.0)
                    }
                }

                impl Add<i32> for #ident {
                    type Output = Rolls;
                
                    fn add(self, rhs: i32) -> Self::Output {
                        self.roll(1)
                            .then(|x| x + rhs)
                    }
                }
                impl Sub<i32> for #ident {
                    type Output = Rolls;
                
                    fn sub(self, rhs: i32) -> Self::Output {
                        self.roll(1)
                            .then(|x| x - rhs)
                    }
                }
                impl Mul<i32> for #ident {
                    type Output = Rolls;
                
                    fn mul(self, rhs: i32) -> Self::Output {
                        self.roll(1)
                            .then(|x| x * rhs)
                    }
                }
                impl Div<i32> for #ident {
                    type Output = Rolls;
                
                    fn div(self, rhs: i32) -> Self::Output {
                        self.roll(1)
                            .then(|x| x / rhs)
                    }
                }

                impl Into<Rolls> for #ident {
                    fn into(self) -> Rolls {
                        self.roll(1)
                    }
                }
            }
        });

    quote! {
        #(#dice_impls)*
    }.into()
}

///
/// Defines new abilites, implementing all the necessary
/// traits.
/// 
/// Supports attributes (and rustdoc!)
/// 
/// ***
/// 
/// ```
/// use xander_macros::abilities;
/// use xander::ability::Ability;
/// 
/// abilities!(
///     ///
///     /// Homewbrew abilitiy: how 'cool' you are.
///     ///
///     Coolness
/// );
/// ```
/// 
#[proc_macro]
pub fn abilities(tokens : TokenStream) -> TokenStream {
    struct AbilityDecl(Vec<Attribute>, Ident);

    impl Parse for AbilityDecl {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(Attribute::parse_outer(input)?, input.parse()?))
        }
    }

    struct AbilityList(Punctuated<AbilityDecl, Token![,]>);

    impl Parse for AbilityList {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }

    impl AbilityList {
        fn into_iter(self) -> impl Iterator<Item = (Vec<Attribute>, Ident)> {
            self.0.into_iter().map(|a| (a.0, a.1))
        }
    }
    let idents = parse_macro_input!(tokens as AbilityList);

    let iter = idents
        .into_iter()
        .map(|(attrs, ident)| {
            let id = format!("5E::ABILITY::{}",ident.to_string().to_uppercase());
            quote! {
                #(#attrs)*
                #[doc = "***"]
                #[doc = "🤖 Autogenerated by [xander_macros::abilities]"]
                #[doc = ""]
                #[derive(std::fmt::Debug, Copy, Clone, Hash, PartialEq, Eq)]
                pub struct #ident;

                impl Ability  for #ident {
                    fn default() -> Self {
                        Self
                    }
                }
                impl Identity for #ident {
                    fn id(&self) -> &'static str {
                        #id
                    }

                    fn __id() -> &'static str {
                        #id
                    }
                }
                impl Save  for #ident {}
                impl Check for #ident {
                    fn base() -> Box<dyn Ability> {
                        Box::new(Self)
                    } 
                }
            }
        });

    quote! {
        #(#iter)*
    }.into()
}

///
/// Defines new skills, with each based on an ability.
/// 
/// ### Syntax
/// `skills!(BaseAbility(Skill1, Skill2, ...), ...)`
/// 
#[proc_macro]
pub fn skills(tokens : TokenStream) -> TokenStream {
    struct SkillDecl(Vec<Attribute>, Ident);
    
    impl Parse for SkillDecl {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(Attribute::parse_outer(input)?, input.parse()?))
        }
    }
    
    struct AbilitySkills(Ident, Paren, Punctuated<SkillDecl, Token![,]>);
    
    impl Parse for AbilitySkills {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let stream;
            Ok(Self(
                input.parse()?,
                parenthesized!(stream in input),
                stream.parse_terminated(SkillDecl::parse, Token![,])?
            ))
        }
    }
    
    struct Skills(Punctuated<AbilitySkills, Token![,]>);
    
    impl Parse for Skills {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(
                Punctuated::parse_terminated(input)?
            ))
        }
    }
    
    impl Skills {
        fn into_iter(self) -> impl Iterator<Item = AbilitySkills> {
            self.0.into_iter()
        }
    }
    let abilities = parse_macro_input!(tokens as Skills);

    let iter = abilities.into_iter()
        .flat_map(|AbilitySkills(ab_ident, _, ab)| {
            ab.into_iter()
                .map(move |sk| {
                    let ab = &ab_ident;
                    let attrs = sk.0;
                    let sk = sk.1;
                    let id = format!("5E::SKILL::{}", sk.to_string().to_uppercase());
                    let doc_string = format!("Base ability: [{}]", ab.to_string());
                    quote! {
                        #(#attrs)*
                        #[doc = "***"]
                        #[doc = #doc_string]
                        #[doc = "***"]
                        #[doc = "🤖 Autogenerated by [xander_macros::skills]"]
                        #[doc = ""]
                        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
                        pub struct #sk;

                        impl Identity for #sk {
                            fn id(&self) -> &'static str {
                                #id
                            }

                            fn __id() -> &'static str {
                                #id
                            }
                        }

                        impl Skill for #sk {
                            fn base(&self) -> Box<dyn Ability> {
                                Box::new(#ab)
                            }
                        }

                        impl Check for #sk {
                            fn base() -> Box<dyn Ability> {
                                Box::new(#ab)
                            } 
                        }
                    }
                })
        });

    quote! {
        #(#iter)*
    }.into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Proficiency(attr : TokenStream, body : TokenStream) -> TokenStream {

    struct AttrInput(LitStr);

    impl Parse for AttrInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?))
        }
    }

    let AttrInput(id) = parse_macro_input!(attr as AttrInput);

    let body = parse_macro_input!(body as ItemStruct);
    
    let original_attrs = body.attrs.iter(); 
    let ident = &body.ident;
    let traits = body.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    quote! {
        #(#original_attrs)*
        #[derive(std::fmt::Debug, core::hash::Hash, core::cmp::PartialEq, core::cmp::Eq)]
        pub struct #ident<T: #(#traits)+* + std::fmt::Debug + core::hash::Hash + core::cmp::Eq>(pub T);

        impl<T: #(#traits)+* + std::fmt::Debug + core::hash::Hash + core::cmp::Eq> Proficiency<T> for #ident<T> {
            fn value(&self) -> &T {
                &self.0
            }
        }

        impl<T: #(#traits)+* + std::fmt::Debug + core::hash::Hash + core::cmp::Eq> Identity for #ident<T> {
            fn id(&self) -> &'static str {
                #id
            }

            fn __id() -> &'static str 
                where Self : Sized
            {
                #id
            }
        }

    }.into()
}

#[proc_macro]
pub fn identify(tokens : TokenStream) -> TokenStream {
    struct Input(Path, Token![,], LitStr);
    impl Parse for Input {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?, input.parse()?, input.parse()?))
        }
    }

    let Input(ident, _, id) = parse_macro_input!(tokens as Input);

    quote! {
        impl Identity for #ident {
            fn id(&self) -> &'static str {
                #id
            }

            fn __id() -> &'static str 
                where Self : Sized
            {
                #id
            }
        }
    }.into()
}

///
/// Simplifies the process of creating a new proficiency
/// type.
/// ***
/// ```
/// use xander_macros::ProficiencyType;
/// use xander::{
///     Identity,
///     creature::{
///         Creature,
///         proficiency::{Proficiency, ProficiencyType, ProficiencyTyped}
///     }
/// };
/// 
/// ///
/// /// The cool new 'triple' proficiency.
/// /// It's even better than Expertise!
/// ///
/// #[ProficiencyType("HOMEBREW::PROFICIENCY_TYPE::TRIPLE")]
/// pub fn Triple(&self, _ : &Creature, prof_bonus : i32) -> i32 {
///     prof_bonus * 3
/// }
/// ```
/// 
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn ProficiencyType(attrs : TokenStream, body : TokenStream) -> TokenStream {
    use convert_case::Case::Snake;
    struct AttrInput(LitStr);
    impl Parse for AttrInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?))
        }
    }

    let AttrInput(id) = parse_macro_input!(attrs as AttrInput);

    let mut body = parse_macro_input!(body as ItemFn);
    let ident = body.sig.ident.clone();
    let original_attrs = body.attrs;
    let vis = body.vis;
    body.vis = Visibility::Inherited;
    body.attrs = vec![];
    body.sig.ident = Ident::new("bonus", Span::call_site());

    let util_trait_ident = Ident::new(&format!("Into{}", ident.to_string()), Span::call_site());
    let util_trait_fn_ident = Ident::new(&ident.to_string().to_case(Snake), Span::call_site());

    let doc_string = ident.to_string();

    quote! {
        #(#original_attrs)*
        #[derive(Debug)]
        #vis struct #ident;

        ::xander_macros::identify!(#ident, #id);

        impl ProficiencyType for #ident {
            #body
        }

        pub trait #util_trait_ident<I, P>
            where
            I : Identity + core::hash::Hash + core::cmp::Eq,
            P : Proficiency<I> 
        {   
            #[doc = "Sets the type of this proficiency to"]
            #[doc = #doc_string]
            #[doc = "***"]
            #[doc = "🤖 Autogenerated by [xander_macros::ProficiencyType]"]
            fn #util_trait_fn_ident(self) -> ProficiencyTyped<#ident, I, P>;
        }

        impl<I, P> #util_trait_ident<I, P> for P
            where
                I : Identity + core::hash::Hash + core::cmp::Eq,
                P : Proficiency<I>
        {
            fn #util_trait_fn_ident(self) -> ProficiencyTyped<#ident, I, P> {
                ProficiencyTyped(#ident, self, std::marker::PhantomData::<I>)
            }
        }
        
    }.into()
     
}