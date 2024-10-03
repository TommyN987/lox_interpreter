#[macro_export]
macro_rules! define_ast {
    ($enum_name:ident,
        $($variant_name:ident ( $lowercase_name:ident )  { $( $field_name:ident : $field_type:ty ),* }),* $(,)?) => {

            #[derive(Debug, Clone)]
            pub enum $enum_name {
                $(
                    $variant_name($variant_name),
                )*
            }

            impl $enum_name {
                pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
                    match self {
                        $(
                            $enum_name::$variant_name(inner) => inner.accept(visitor),
                        )*
                    }
                }
            }

            $(
                #[derive(Debug, Clone)]
                pub struct $variant_name {
                    $(pub $field_name: $field_type,)*
                }

                impl $variant_name {
                    pub fn new($($field_name: $field_type),*) -> Self {
                        Self { $($field_name),* }
                    }

                    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
                        visitor.$lowercase_name(self)
                    }
                }
            )*

            pub trait Visitor<T> {
                $(
                    fn $lowercase_name(&mut self, expr: &$variant_name) -> T;
                )*
            }
    };
}
