#[macro_export]
macro_rules! builder {
    ($fn: ident => $builder: ident <$opts: ty> + $($derives: ident),+ {
        $($field: ident : $type: ty = $val: expr),* $(,)?
    }) => {
        pub fn $fn(name: &str) -> $builder {
            <$builder>::new(name)
        }

        #[derive(Default, $($derives),+)]
        pub struct $builder {
            name: String,
            opts: $opts,
            $($field: $type),*
        }

        impl $builder {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_owned(),
                    opts: <$opts>::new(),
                    $($field: $val),*
                }
            }

            pub fn without_log(mut self) -> Self {
                self.opts.log = false;
                self
            }

        }
    };
}
