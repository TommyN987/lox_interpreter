#[macro_export]
macro_rules! define_ast {
    ($enum_name:ident,
        $($variant_name:ident ( $lowercase_name:ident )  { $( $field_name:ident : $field_type:ty ),* }),* $(,)?) => {

            pub enum $enum_name {
                $(
                    $variant_name($variant_name),
                )*
            }

            $(
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
