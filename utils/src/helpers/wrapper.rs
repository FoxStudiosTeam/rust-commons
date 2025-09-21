#[macro_export]
macro_rules! wrappers {
    (
        $(
            $(#[$struct_meta:meta])*
            $wrapper_vis:vis $name:ident($inner_vis:vis $inner_ty:ty)
        )*
    ) => {
        $(
            $(#[$struct_meta])*
            $wrapper_vis struct $name (
                $inner_vis $inner_ty
            );

            impl std::ops::Deref for $name {
                type Target = $inner_ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$inner_ty> for $name {
                fn from(value: $inner_ty) -> Self {
                    Self ( value )
                }
            }
        )*
    };
}