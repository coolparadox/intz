extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Uintz)]
pub fn intz_utrait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_intz_utrait(&ast)
}

fn impl_intz_utrait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Uintz for #name {

            fn zero(&self) -> Self {
                Self {
                    hi: self.hi.zero(),
                    lo: self.lo.zero(),
                }
            }

            fn addc(self, other: Self, carry: bool) -> (Self, bool) {
                let (lo, loc) = self.lo.addc(other.lo, carry);
                let (hi, hic) = self.hi.addc(other.hi, loc);
                (Self { hi, lo }, hic)
            }

        }
    };
    gen.into()
}

