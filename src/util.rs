/**
 * Create an enum that maps to and from the given value.
 *
 * ```
 * use smb1_tools::enum_mapped;
 *
 * enum_mapped!(
 *     MyCoolEnum (u32) {
 *         12 => Foo,
 *         13 => Bar,
 *         57 => Baz,
 *     }
 * );
 * ```
 *
 * Is functionally equivalent to:
 *
 * ```
 * #[derive(Debug)]
 * enum MyCoolEnum {
 *     Foo,
 *     Bar,
 *     Baz,
 * }
 *
 * impl MyCoolEnum {
 *     pub fn new(val: u32) -> Self {
 *         match val {
 *             12 => Self::Foo,
 *             13 => Self::Bar,
 *             57 => Self::Baz,
 *             _ => panic!("invalid {} val: {:?}", "MyCoolEnum", val),
 *         }
 *     }
 *
 *     pub fn value(&self) -> u32 {
 *         match self {
 *             Self::Foo => 12,
 *             Self::Bar => 13,
 *             Self::Baz => 57,
 *         }
 *     }
 * }
 *
 * impl PartialEq for MyCoolEnum {
 *     fn eq(&self, other: &Self) -> bool {
 *         self.value() == other.value()
 *     }
 * }
 * impl Eq for MyCoolEnum {}
 * ```
 */
#[macro_export]
macro_rules! enum_mapped {
    ($v:vis $name:ident ($type:ty) { $($val:expr => $variant:ident,)* } ) => {
        #[derive(Debug)]
        $v enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(val: $type) -> Self {
                match val {
                    $(
                        $val => Self::$variant,
                    )*
                    _ => panic!("invalid {} val: {:?}", stringify!($name), val),
                }
            }

            #[allow(dead_code)]
            pub fn value(&self) -> $type {
                match self {
                    $(
                        Self::$variant => $val,
                    )*
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value() == other.value()
            }
        }
        impl Eq for $name {}
    };
}
pub(crate) use enum_mapped;

#[cfg(test)]
mod tests {
    use super::*;

    enum_mapped!(
        MyNumberEnum (u32) {
            47 => Foo,
            78 => Bar,
        }
    );

    enum_mapped!(
        MyStrEnum (&str) {
            "foo" => Foo,
            "bar" => Bar,
        }
    );

    #[test]
    fn test_num_1() {
        assert_eq!(MyNumberEnum::Foo.value(), 47);
        assert_eq!(MyNumberEnum::Bar.value(), 78);
    }

    #[test]
    fn test_num_2() {
        assert_eq!(MyNumberEnum::new(47), MyNumberEnum::Foo);
        assert_eq!(MyNumberEnum::new(78), MyNumberEnum::Bar);
    }

    #[test]
    #[should_panic]
    fn test_num_3() {
        let _thing = MyNumberEnum::new(100);
    }

    #[test]
    fn test_str_1() {
        assert_eq!(MyStrEnum::Foo.value(), "foo");
        assert_eq!(MyStrEnum::Bar.value(), "bar");
    }

    #[test]
    fn test_str_2() {
        assert_eq!(MyStrEnum::new("foo"), MyStrEnum::Foo);
        assert_eq!(MyStrEnum::new("bar"), MyStrEnum::Bar);
    }

    #[test]
    #[should_panic]
    fn test_str_3() {
        let _thing = MyStrEnum::new("baz");
    }
}
