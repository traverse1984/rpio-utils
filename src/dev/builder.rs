use super::{output, spi};

/// Create a mock device
#[derive(Debug)]
pub struct Mock;

impl Mock {
    pub fn pin(name: &str) -> output::mock::MockBuilder {
        output::mock::MockBuilder::new(name)
    }

    pub fn spi(name: &str) -> spi::mock::MockBuilder {
        spi::mock::MockBuilder::new(name)
    }
}

/// Create a device intercept
#[derive(Debug)]
pub struct Intercept;

impl Intercept {
    pub fn pin(name: &str) -> output::intercept::InterceptBuilder {
        output::intercept::InterceptBuilder::new(name)
    }

    pub fn spi(name: &str) -> spi::intercept::InterceptBuilder {
        spi::intercept::InterceptBuilder::new(name)
    }
}

macro_rules! builder {
    ($builder: ident <$opts: ty> + $($derives: ident),+ {
        $($field: ident : $type: ty = $val: expr),* $(,)?
    }) => {
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

            /// Do not print events to stdout.
            pub fn without_log(mut self) -> Self {
                self.opts.log = false;
                self
            }

        }
    };
}
