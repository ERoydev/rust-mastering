use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

/*
With the combination of using
    - derive_more with features = ["from", "display"]
    - Rust10x extension that provides snippets

I am able to generate this out of the box by using
`1error` keyword which prompts me for the snippets and i picked `10x-error-rs-03-with-custom`

The idea is that in production code this is what i will use to define Result and Error and then just use in my entire application
i can use serde_as and other methods to extend and give custom error messages.

So generally in my functions i will just use `?` the error propagation pattern and display error when its time

*/

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
    // -- fs (i do organization by module)
    FsEmptyFolderError,
    // #[from]
    // Fs(fs::Error),

    // -- Externals
    #[from]
    Io(std::io::Error), // as example
}

// region:    --- Custom

impl std::error::Error for Error {}
